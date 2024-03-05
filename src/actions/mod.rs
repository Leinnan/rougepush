use bevy::prelude::*;
use std::any::Any;

use crate::{input::InputAction, vectors::Vector2Int};

use self::{melee_hit::MeleeHitAction, walk::WalkAction};

pub mod damage;
pub mod melee_hit;
pub mod walk;

pub trait Action: Send + Sync {
    fn get_input(&self) -> Option<InputAction>;
    fn execute(&self, world: &mut World) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn action_type(&self) -> ActionType;
    fn target_pos(&self) -> Option<Vector2Int>;
}

pub trait RegisterActions {
    fn register_all_actions(&mut self) -> &mut Self;
}

impl RegisterActions for App {
    fn register_all_actions(&mut self) -> &mut Self {
        WalkAction::register(self);
        MeleeHitAction::register(self);
        self
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Reflect)]
pub enum ActionType {
    Damage,
    MeleeeHit,
    Walk,
}
