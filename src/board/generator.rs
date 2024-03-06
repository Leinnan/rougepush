use super::components::{CurrentBoard, TileType};
use crate::{dungeon::*, states::ActionDelay, vectors::Vector2Int};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use rand::prelude::SliceRandom;
use super::components::*;

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
    let new_board = CurrentBoard { tiles };

    new_board.print();
    let area = || dungeon.areas.choose(&mut rand::thread_rng()).unwrap();
    let random_point = ||area().rooms.choose(&mut rand::thread_rng()).unwrap().random_point();
    commands.spawn((
        Piece::Player,
        Occupier,
        PlayerControl,
        ActionDelay(0),
        Health { value: 3 },
        Melee { damage: 2 },
        PiecePos(random_point()),
    ));

    commands.spawn((
        Piece::Enemy,
        Occupier,
        AiControl::default(),
        ActionDelay(1),
        Health { value: 1 },
        Melee { damage: 1 },
        PiecePos(random_point()),
    ));
    commands.spawn((
        Piece::Enemy,
        Occupier,
        AiControl::default(),
        ActionDelay(1),
        Health { value: 1 },
        Melee { damage: 1 },
        PiecePos(random_point()),
    ));

    commands.insert_resource(new_board);
}
