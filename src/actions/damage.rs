use super::Action;
use crate::{board::components::*, input::InputAction, vectors::Vector2Int};
use bevy::prelude::*;

pub struct DamageAction(pub Entity, pub u32);
impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> bool {
        let Some(mut health) = world.get_mut::<Health>(self.0) else {
            return false;
        };
        health.value = health.value.saturating_sub(self.1);
        if health.value == 0 {
            world
                .entity_mut(self.0)
                .remove::<Health>()
                .remove::<Piece>()
                .remove::<Occupier>();
        }
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_input(&self) -> Option<InputAction> {
        None
    }
    fn action_type(&self) -> super::ActionType {
        super::ActionType::Damage
    }
    fn target_pos(&self) -> Option<Vector2Int> {
        None
    }
}
