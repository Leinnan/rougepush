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

#[derive(Debug, Hash, PartialEq, Eq, Default, Clone, States)]
pub enum GameTurnSteps {
    #[default]
    None,
    ActionSelection,
    PerformAction,
}

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugin, ButtonsReleasedPlugin))
            .init_state::<MainGameState>()
            .init_state::<GameTurnSteps>()
            .add_systems(
                OnEnter(GameTurnSteps::ActionSelection),
                (set_current_actor,prepare_action_list).chain()
            );
    }
}

fn set_current_actor(){
    info!("CC");
}

fn prepare_action_list(){
    info!("DD");
}