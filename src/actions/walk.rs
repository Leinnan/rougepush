use std::ops::Deref;

use super::Action;
use crate::{board::components::*, input::InputAction, states::*, vectors::Vector2Int};
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct WalkAction(pub Entity, pub Vector2Int, pub InputAction);

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
        let Ok((actions, mut to_remove)) = q.single_mut() else {
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
    fn get_input(&self) -> Option<InputAction> {
        Some(self.2)
    }
    fn execute(&self, world: &mut World) -> bool {
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
        };
        let Some(mut position) = world.get_mut::<PiecePos>(self.0) else {
            return false;
        };
        position.0 = self.1;

        true
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn action_type(&self) -> super::ActionType {
        super::ActionType::Walk
    }
    fn target_pos(&self) -> Option<Vector2Int> {
        Some(self.1)
    }
}
