use bevy::prelude::*;
use std::any::Any;

use self::walk::WalkAction;

pub mod walk;

pub trait Action: Send + Sync {
    fn get_key_code(&self) -> KeyCode;
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
    fn as_any(&self) -> &dyn Any;
}

pub trait RegisterActions {
    fn register_all_actions(&mut self) -> &mut Self;
}

impl RegisterActions for App {
    fn register_all_actions(&mut self) -> &mut Self {
        WalkAction::register(self);
        self
    }
}
