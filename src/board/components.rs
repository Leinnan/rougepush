use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::vectors::Vector2Int;

#[derive(Component, Reflect, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TileType {
    #[default]
    None,
    BaseFloor,
    Pit,
}

#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Piece {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct GameObject;

#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord, Clone, Deref, DerefMut)]
pub struct PiecePos(pub Vector2Int);

#[derive(Component, Default, Reflect)]
pub struct Occupier;

#[derive(Component)]
pub struct MapTile;

#[derive(Default, Resource, Reflect)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, TileType>,
}

impl CurrentBoard {
    pub fn get(&self, x: i32, y: i32) -> Option<&TileType> {
        self.tiles.get(&Vector2Int { x, y })
    }

    pub fn print(&self) {
        let max_x = self.tiles.iter().map(|t| t.0.x).max().unwrap();
        let max_y = self.tiles.iter().map(|t| t.0.y).max().unwrap();

        let mut lines = Vec::new();
        for _ in 0..(max_y + 1) {
            lines.push(vec!['*'; max_x as usize + 1]);
        }
        for (pos, tile_type) in self.tiles.iter() {
            lines[pos.y as usize][pos.x as usize] = match tile_type {
                TileType::None => '#',
                TileType::BaseFloor => 'f',
                TileType::Pit => 'p',
            };
        }
        for line in lines {
            info!("{}", line.iter().collect::<String>());
        }
    }
}

#[derive(Component, Reflect)]
pub struct Health {
    pub value: u32,
}

/// melee attack behaviour for the npcs
#[derive(Component, Reflect)]
pub struct Melee {
    pub damage: u32,
}

#[derive(Component, Reflect)]
pub struct PlayerControl;

#[derive(Component, Reflect)]
pub struct AiControl {
    pub max_distance_to_player: usize,
}

#[derive(Component, Reflect)]
pub struct Flying;

impl Default for AiControl {
    fn default() -> Self {
        Self {
            max_distance_to_player: 5,
        }
    }
}

#[derive(Component, Reflect)]
pub struct Animation {
    /// indices of all the frames in the animation
    pub frames: Vec<usize>,
    pub current: usize,
    pub timer: Timer,
}
