use std::{collections::VecDeque, ops::Deref};

use super::{damage::DamageAction, Action};
use crate::{board::components::*, states::*, vectors::Vector2Int};
use bevy::prelude::*;

pub struct MeleeHitAction {
    pub attacker: Entity,
    pub attacker_type: Piece,
    pub target: Vector2Int,
    pub damage: u32,
    pub key: Option<KeyCode>,
}
impl Action for MeleeHitAction {
    fn execute(&self, world: &mut World) -> bool {
        let Some(attacker_position) = world.get::<PiecePos>(self.attacker) else {
            return false;
        };
        if attacker_position.0.manhattan(self.target) > 1 {
            return false;
        };
        let target_entities = world
            .query_filtered::<(Entity, &PiecePos, &Piece), With<Health>>()
            .iter(world)
            .filter(|(_, p, piece)| p.0 == self.target && &self.attacker_type != *piece)
            .collect::<Vec<_>>();
        if target_entities.is_empty() {
            return false;
        };
        let mut result = target_entities
            .iter()
            .map(|e| Box::new(DamageAction(e.0, self.damage)) as Box<dyn Action>)
            .collect::<VecDeque<_>>();
        if let Some(mut pending_actions) = world.get_resource_mut::<PendingActions>() {
            info!("APPEND THE SHIT OUT OF IT");
            pending_actions.append(&mut result);
        }
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_key_code(&self) -> Option<KeyCode> {
        self.key
    }
    fn action_type(&self) -> super::ActionType {
        super::ActionType::MeleeeHit
    }
    fn target_pos(&self) -> Option<Vector2Int> {
        Some(self.target)
    }
}

impl MeleeHitAction {
    pub fn register(app: &mut App) {
        app.add_systems(
            OnEnter(GameTurnSteps::ActionSelection),
            (Self::trim_attacks_without_enemies).in_set(PreparingActions::FindWrongMoves),
        );
    }

    pub fn trim_attacks_without_enemies(
        mut q: Query<(&PossibleActions, &mut ActionsToRemove)>,
        other_pieces: Query<(&PiecePos,&Piece), With<Health>>,
    ) {
        let Ok((actions, mut to_remove)) = q.get_single_mut() else {
            return;
        };
        let actions = actions.deref().deref();
        let mut wrong_actions = Vec::new();
        for (index, boxed_action) in actions.iter().enumerate() {
            let Some(action) = boxed_action.as_any().downcast_ref::<MeleeHitAction>() else {
                continue;
            };
            let mut is_valid_move = false;
            if other_pieces.iter().any(|(p, piece)| **p == action.target && action.attacker_type != *piece) {
                is_valid_move = true;
            }
            if !is_valid_move {
                wrong_actions.push(index);
            }
        }
        to_remove.0.append(&mut wrong_actions);
    }
}
