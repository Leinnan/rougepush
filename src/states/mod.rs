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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameTurnSteps {
    PrepareActionList,
    ActionSelection,
    PerformAction,
    AnimAction,
}

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugin, ButtonsReleasedPlugin))
            .init_state::<MainGameState>()
            .configure_sets(
                Update,
                (
                    GameTurnSteps::PrepareActionList,
                    GameTurnSteps::ActionSelection,
                    GameTurnSteps::PerformAction,
                    GameTurnSteps::AnimAction,
                )
                    .chain()
                    .run_if(in_state(MainGameState::Game)),
            );
    }
}
