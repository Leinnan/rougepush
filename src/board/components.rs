use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_inspector_egui::InspectorOptions;

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

#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord, Clone, Deref, DerefMut)]
pub struct PiecePos(pub Vector2Int);

#[derive(Component, Default, Reflect)]
pub struct Occupier;

#[derive(Default, Resource, Reflect)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, TileType>,
}

impl CurrentBoard {
    pub fn get(&self, x: i32, y: i32) -> Option<&TileType> {
        self.tiles.get(&Vector2Int { x, y })
    }
}

#[derive(Component, Reflect, InspectorOptions)]
pub struct Health {
    pub value: u32,
}

/// melee attack behaviour for the npcs
#[derive(Component, Reflect, InspectorOptions)]
pub struct Melee {
    pub damage: u32,
}

#[derive(Component, Reflect)]
pub struct PlayerControl;

#[derive(Component, Reflect)]
pub struct AiControl;

#[derive(Component, Reflect)]
pub struct Torch;
