use std::ops::Deref;

use super::Action;
use crate::{board::components::*, states::*, vectors::Vector2Int};
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct WalkAction(pub Entity, pub Vector2Int, pub KeyCode);

impl WalkAction {
    pub fn register(app: &mut App) {
        app.add_systems(
            OnEnter(GameTurnSteps::ActionSelection),
            (Self::trim_moves_into_abyss).in_set(PreparingActions::FindWrongMoves),
        );
    }

    pub fn trim_moves_into_abyss(
        mut q: Query<(&PossibleActions, &mut ActionsToRemove)>,
        other_pieces: Query<&PiecePos, With<Occupier>>,
        board: Res<CurrentBoard>,
    ) {
        let Ok((actions, mut to_remove)) = q.get_single_mut() else {
            return;
        };
        let actions = actions.deref().deref();
        let mut wrong_actions = Vec::new();
        for (index, boxed_action) in actions.iter().enumerate() {
            let Some(action) = boxed_action.as_any().downcast_ref::<WalkAction>() else {
                continue;
            };
            let mut is_valid_move = false;
            if let Some(tile) = board.tiles.get(&action.1) {
                if tile == &TileType::BaseFloor {
                    is_valid_move = true;
                }
            }
            if other_pieces.iter().any(|p| p.0 == action.1) {
                is_valid_move = false;
            }
            if !is_valid_move {
                wrong_actions.push(index);
            }
        }
        to_remove.0.append(&mut wrong_actions);
    }
}

impl Action for WalkAction {
    fn get_key_code(&self) -> KeyCode {
        self.2
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
