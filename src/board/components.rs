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

#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut)]
pub struct PiecePos(pub Vector2Int);

impl From<PiecePos> for Transform {
    fn from(val: PiecePos) -> Self {
        Transform::from_xyz(val.x as f32, 0.1, val.y as f32)
    }
}
impl From<&PiecePos> for Transform {
    fn from(val: &PiecePos) -> Self {
        Transform::from_xyz(val.x as f32, 0.1, val.y as f32)
    }
}

#[derive(Component, Default, Reflect)]
pub struct Occupier;

#[derive(Component)]
pub struct MapTile;

#[derive(Default, Resource, Reflect)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, TileType>,
    pub spawn_points: HashMap<Vector2Int, Piece>,
    pub root: Option<Entity>,
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
            lines[pos.y as usize][pos.x as usize] = match (tile_type, self.spawn_points.get(pos)) {
                (TileType::None, _) => '#',
                (TileType::BaseFloor, spawn) => match spawn {
                    Some(piece) => {
                        if piece == &Piece::Enemy {
                            'E'
                        } else {
                            'P'
                        }
                    }
                    None => 'f',
                },
                (TileType::Pit, _) => 'p',
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
