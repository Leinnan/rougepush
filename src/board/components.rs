use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::vectors::Vector2Int;

#[derive(Component, Reflect, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TileType {
    #[default]
    None,
    BaseFloor,
    Pit,
}

#[derive(Default, Resource, Reflect)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, TileType>,
}

impl CurrentBoard {
    pub fn get(&self, x: i32, y: i32) -> Option<&TileType> {
        self.tiles.get(&Vector2Int { x, y })
    }
}
