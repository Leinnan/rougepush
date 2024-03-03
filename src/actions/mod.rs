use bevy::prelude::*;
use std::any::Any;

use crate::{board::components::*, vectors::Vector2Int};

pub trait Action: Send + Sync {
    fn can_execute(&self, world: &mut World) -> bool;
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
    fn as_any(&self) -> &dyn Any;
}

pub struct WalkAction(pub Entity, pub Vector2Int);
impl Action for WalkAction {
    fn can_execute(&self, world: &mut World) -> bool {
        let Some(board) = world.get_resource::<CurrentBoard>() else {
            return false;
        };
        if !board.tiles.contains_key(&self.1) {
            return false;
        };
        if world
            .query_filtered::<&PiecePos, With<Occupier>>()
            .iter(world)
            .any(|p| p.0 == self.1)
        {
            return false;
        }

        true
    }
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let board = world.get_resource::<CurrentBoard>().ok_or(())?;
        if !board.tiles.contains_key(&self.1) {
            return Err(());
        };
        if world
            .query_filtered::<&PiecePos, With<Occupier>>()
            .iter(world)
            .any(|p| p.0 == self.1)
        {
            return Err(());
        };
        let mut position = world.get_mut::<PiecePos>(self.0).ok_or(())?;
        position.0 = self.1;
        Ok(Vec::new())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
