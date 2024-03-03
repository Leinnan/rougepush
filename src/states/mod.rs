pub mod menu;
use bevy::{prelude::*, utils::HashMap};
use bevy_button_released_plugin::*;

use crate::{actions::{Action, WalkAction}, board::components::*, vectors::Vector2Int};

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

#[derive(Deref, DerefMut,Component)]
pub struct PossibleActions(pub HashMap<KeyCode,Box<dyn Action>>);

#[derive(Default,Debug,Reflect,Component)]
pub struct CurrentActorToken;

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugin, ButtonsReleasedPlugin))
            .init_state::<MainGameState>()
            .init_state::<GameTurnSteps>()
            .register_type::<CurrentActorToken>()
            .add_systems(Update, find_actor.run_if(in_state(GameTurnSteps::None)))
            .add_systems(
                OnEnter(GameTurnSteps::ActionSelection),
                (set_current_actor,prepare_action_list).chain()
            );
    }
}

fn find_actor(
    query: Query<(Entity, &Piece)>, 
    mut next_state: ResMut<NextState<GameTurnSteps>>,){
        if !query.is_empty() {
            next_state.set(GameTurnSteps::ActionSelection);
        }
}

fn set_current_actor(
    mut commands: Commands,query: Query<(Entity, &Piece)>){
    info!("set_current_actor");
    if let Ok((entity,piece)) = query.get_single() {
        info!("CURRENT TOKEN");
        commands.entity(entity).insert(CurrentActorToken);
    }
}

fn prepare_action_list(
    world: &mut World
){
    info!("prepare_action_list");
    let mut query = world.query_filtered::<(Entity, &Piece,&PiecePos),With<CurrentActorToken>>();
    let Ok((entity,piece,pos)) = query.get_single(world) else {return;};

    info!("Found piece!");
    let Some(board) = world.get_resource::<CurrentBoard>() else {
        return;
    };
    let dirs = vec![(KeyCode::KeyA,Vector2Int::new(-1,0)),
    (KeyCode::KeyD,Vector2Int::new(1,0)),(KeyCode::KeyW,Vector2Int::new(0,1)),
    (KeyCode::KeyS,Vector2Int::new(0,-1))];
    
    for (key_code,movement) in dirs {
        let target_pos = pos.0+movement;
         let walk = WalkAction(entity,target_pos);
         
         if !board.tiles.contains_key(&target_pos) {
             continue;
         };
         // THIS DOESNT WORK- MAYBE I NEED TO SPLIT IT INTO TWO STEPS
         // FIRST GATHER ALL POSSIBLE ACTIONS
         // SECOND REMOVE THOSE THAT ARE NOT AVAILABLE
        //  let query = world
        //  .query_filtered::<&PiecePos, With<Occupier>>();
        //  if query.iter(world)
        //      .any(|p| p.0 == target_pos)
        //  {
        //      continue;
        //  }
    }
}