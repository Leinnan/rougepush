use super::components::*;
use super::components::{CurrentBoard, TileType};
use crate::{dungeon::*, states::ActorTurn, vectors::Vector2Int};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use rand::Rng;

pub fn create_map(mut commands: Commands) {
    info!("Start world generate");

    let mut dungeon = Dungeon::new(3);

    for idx in 0..6 {
        let tun = match idx % 2 {
            0 => Box::new(LShapeTunneler) as Box<dyn Tunneler>,
            _ => Box::new(RandomTunneler) as Box<dyn Tunneler>,
        };
        dungeon.add_area(Area::new(tun))
    }
    dungeon.generate();
    let mut tiles: HashMap<Vector2Int, TileType> = dungeon
        .to_tiles()
        .iter()
        .map(|p| (*p, TileType::BaseFloor))
        .collect();
    let mut spawn_points = HashMap::new();
    let mut first_room = true;
    for area in dungeon.areas.iter() {
        for room in area.rooms.iter() {
            if first_room {
                spawn_points.insert(room.random_point_without_walls(), Piece::Player);
                first_room = false;
                continue;
            }
            let mut rng = rand::thread_rng();
            for _ in 0..rng.gen_range(0..4) {
                let point = room.random_point_without_walls();
                tiles.entry(point).and_modify(|e| *e = TileType::Pit);
            }

            let enemies_amount = rng.gen_range(1..=4);
            for _ in 0..enemies_amount {
                for _ in 0..5 {
                    let random_point = room.random_point();

                    if tiles[&random_point] == TileType::BaseFloor
                        && !spawn_points.contains_key(&random_point)
                    {
                        spawn_points.insert(random_point, Piece::Enemy);
                        break;
                    }
                }
            }
        }
    }
    let root = commands
        .spawn((Name::new("CurrentBoard"), Transform::default(), InheritedVisibility::VISIBLE))
        .id();
    let new_board = CurrentBoard {
        tiles,
        spawn_points,
        root: root.into(),
    };

    new_board.print();
    commands.insert_resource(new_board);
}

pub fn spawn_points(mut commands: Commands, board: Res<CurrentBoard>) {
    let parent = board.root.unwrap();
    for (point, piece) in board.spawn_points.iter() {
        let id = commands
            .spawn((
                piece.clone(),
                Occupier,
                ActorTurn(if piece == &Piece::Player { 0 } else { 1 }),
                PiecePos(*point),
                GameObject,
            ))
            .set_parent(parent)
            .id();
        match piece {
            Piece::Player => {
                commands.entity(id).insert((
                    PlayerControl,
                    Health { value: 3 },
                    Melee { damage: 1 },
                ));
            }
            Piece::Enemy => {
                commands.entity(id).insert((
                    AiControl::default(),
                    Health { value: 1 },
                    Melee { damage: 1 },
                ));
            }
        }
    }
}
