use super::components::*;
use super::components::{CurrentBoard, TileType};
use crate::{dungeon::*, states::ActionDelay, vectors::Vector2Int};
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
    let tiles: HashMap<Vector2Int, TileType> = dungeon
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
            let enemies_amount = rng.gen_range(1..=4) + 1;
            for _ in 0..enemies_amount {
                for _ in 0..5 {
                    let random_point = room.random_point_without_walls();
                    if !spawn_points.contains_key(&random_point) {
                        spawn_points.insert(random_point, Piece::Enemy);
                        break;
                    }
                }
            }
        }
    }
    let new_board = CurrentBoard {
        tiles,
        spawn_points,
    };

    new_board.print();
    commands.insert_resource(new_board);
}

pub fn spawn_points(mut commands: Commands, board: Res<CurrentBoard>) {
    for (point, piece) in board.spawn_points.iter() {
        let id = commands
            .spawn((
                piece.clone(),
                Occupier,
                ActionDelay(if piece == &Piece::Player { 0 } else { 1 }),
                PiecePos(*point),
            ))
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
