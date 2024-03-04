pub mod menu;
use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use bevy_button_released_plugin::*;

use crate::{
    actions::{Action, WalkAction},
    board::components::*,
    vectors::Vector2Int,
};

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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PreparingActions {
    SetCurrentActor,
    PrepareActionList,
    FindWrongMoves,
    TrimWrongMoves,
}

#[derive(Deref, DerefMut, Component)]
pub struct PossibleActions(pub Vec<Box<dyn Action>>);

#[derive(Deref, DerefMut, Component, Default)]
pub struct ActionsToRemove(pub Vec<usize>);

#[derive(Default, Debug, Reflect, Component)]
pub struct CurrentActorToken;

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugin, ButtonsReleasedPlugin))
            .init_state::<MainGameState>()
            .init_state::<GameTurnSteps>()
            .register_type::<CurrentActorToken>()
            .add_systems(Update, find_actor.run_if(in_state(GameTurnSteps::None)))
            .configure_sets(
                OnEnter(GameTurnSteps::ActionSelection),
                (
                    PreparingActions::SetCurrentActor,
                    PreparingActions::PrepareActionList,
                    PreparingActions::FindWrongMoves,
                    PreparingActions::TrimWrongMoves,
                )
                    .chain(),
            )
            .add_systems(
                OnEnter(GameTurnSteps::ActionSelection),
                set_current_actor.in_set(PreparingActions::SetCurrentActor),
            )
            .add_systems(
                OnEnter(GameTurnSteps::ActionSelection),
                prepare_action_list.in_set(PreparingActions::PrepareActionList),
            )
            .add_systems(
                OnEnter(GameTurnSteps::ActionSelection),
                (trim_moves_into_abyss).in_set(PreparingActions::FindWrongMoves),
            )
            .add_systems(
                OnEnter(GameTurnSteps::ActionSelection),
                remove_moves.in_set(PreparingActions::TrimWrongMoves),
            );
    }
}

fn find_actor(query: Query<(Entity, &Piece)>, mut next_state: ResMut<NextState<GameTurnSteps>>) {
    if !query.is_empty() {
        next_state.set(GameTurnSteps::ActionSelection);
    }
}

fn set_current_actor(mut commands: Commands, query: Query<(Entity, &Piece)>) {
    info!("set_current_actor");
    for (entity, piece) in query.iter() {
        if piece == &Piece::Player {
            info!("CURRENT TOKEN");
            commands.entity(entity).insert(CurrentActorToken);
            break;
        }
    }
}

fn prepare_action_list(world: &mut World) {
    info!("prepare_action_list");
    let mut query = world.query_filtered::<(Entity, &Piece, &PiecePos), With<CurrentActorToken>>();
    let Ok((entity, _piece, pos)) = query.get_single(world) else {
        return;
    };

    info!("Found piece!");
    let dirs = vec![
        (KeyCode::KeyA, Vector2Int::new(-1, 0)),
        (KeyCode::KeyD, Vector2Int::new(1, 0)),
        (KeyCode::KeyW, Vector2Int::new(0, 1)),
        (KeyCode::KeyS, Vector2Int::new(0, -1)),
    ];
    let mut possible_actions: Vec<Box<dyn Action>> = Vec::new();

    for (key_code, movement) in dirs {
        let target_pos = pos.0 + movement;
        let walk = WalkAction(entity, target_pos, key_code);

        possible_actions.push(Box::new(walk));
    }
    world
        .entity_mut(entity)
        .insert(PossibleActions(possible_actions))
        .insert(ActionsToRemove::default());
}

fn trim_moves_into_abyss(
    mut q: Query<(&PossibleActions, &mut ActionsToRemove)>,
    other_pieces: Query<&PiecePos, (With<Occupier>, Without<CurrentActorToken>)>,
    board: Res<CurrentBoard>,
) {
    let Ok((actions, mut to_remove)) = q.get_single_mut() else {
        return;
    };
    let actions = actions.deref().deref();
    let mut wrong_actions = Vec::new();
    for (index, boxed_action) in actions.iter().enumerate() {
        let Some(action) = boxed_action.as_any().downcast_ref::<WalkAction>() else {
            continue;
        };
        let mut is_valid_move = false;
        if let Some(tile) = board.tiles.get(&action.1) {
            if tile == &TileType::BaseFloor {
                is_valid_move = true;
            }
        }
        if other_pieces.iter().any(|p| p.0 == action.1) {
            is_valid_move = false;
        }
        if !is_valid_move {
            wrong_actions.push(index);
        }
    }
    info!("TOREMOVE: {:?}", wrong_actions);
    to_remove.0.append(&mut wrong_actions);
}

fn remove_moves(
    mut commands: Commands,
    mut q: Query<(&mut PossibleActions, &ActionsToRemove, Entity)>,
) {
    let Ok((mut actions, to_remove, entity)) = q.get_single_mut() else {
        return;
    };
    let mut remove_list = to_remove.0.clone();
    remove_list.sort();
    remove_list.reverse();
    let actions = actions.deref_mut().deref_mut();
    for index in remove_list.iter() {
        actions.remove(*index);
    }
    commands.entity(entity).remove::<ActionsToRemove>();
    info!(
        "Removed {} actions, {} actions remain",
        remove_list.len(),
        actions.len()
    );
}
