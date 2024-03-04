use bevy::prelude::*;
use std::any::Any;

use self::{melee_hit::MeleeHitAction, walk::WalkAction};

pub mod damage;
pub mod melee_hit;
pub mod walk;

pub trait Action: Send + Sync {
    fn get_key_code(&self) -> Option<KeyCode>;
    fn execute(&self, world: &mut World) -> bool;
    fn as_any(&self) -> &dyn Any;
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
