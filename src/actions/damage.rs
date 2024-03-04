use super::Action;
use crate::{board::components::*, vectors::Vector2Int};
use bevy::prelude::*;

pub struct DamageAction(pub Entity, pub u32);
impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> bool {
        let Some(mut health) = world.get_mut::<Health>(self.0) else {
            return false;
        };
        health.value = health.value.saturating_sub(self.1);
        if health.value == 0 {
            world.despawn(self.0);
        }
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_key_code(&self) -> Option<KeyCode> {
        None
    }
    fn action_type(&self) -> super::ActionType {
        super::ActionType::Damage
    }
    fn target_pos(&self) -> Option<Vector2Int> {
        None
    }
}
