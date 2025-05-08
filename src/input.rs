use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::board::components::PlayerControl;

/// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum InputAction {
    Left,
    Right,
    Up,
    Down,
    Space,
    Hide,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<InputAction>::default())
            .add_systems(Update, add_input_bindings);
    }
}

fn add_input_bindings(query: Query<Entity, Added<PlayerControl>>, mut commands: Commands) {
    let input_map = InputMap::new([
        (InputAction::Left, KeyCode::KeyA),
        (InputAction::Left, KeyCode::ArrowLeft),
        (InputAction::Right, KeyCode::KeyD),
        (InputAction::Right, KeyCode::ArrowRight),
        (InputAction::Up, KeyCode::KeyW),
        (InputAction::Up, KeyCode::ArrowUp),
        (InputAction::Down, KeyCode::KeyS),
        (InputAction::Down, KeyCode::ArrowDown),
        (InputAction::Hide, KeyCode::KeyH),
    ]);
    for entity in query.iter() {
        commands.entity(entity).insert(input_map.clone());
    }
}
