pub mod menu;
use bevy::prelude::*;
use bevy_button_released_plugin::*;

use self::menu::MenuPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MainGameState {
    #[default]
    AssetLoading,
    Menu,
    Game,
}

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugin, ButtonsReleasedPlugin))
            .init_state::<MainGameState>();
    }
}
