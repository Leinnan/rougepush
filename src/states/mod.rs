pub mod menu;
use bevy::prelude::*;
use bevy_button_released_plugin::*;
use std::collections::VecDeque;
use std::ops::DerefMut;

use crate::{
    actions::{melee_hit::MeleeHitAction, walk::WalkAction, Action, RegisterActions},
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

#[derive(Default, Resource, Deref, DerefMut)]
pub struct PendingActions(pub VecDeque<Box<dyn Action>>);

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugin, ButtonsReleasedPlugin))
            .init_state::<MainGameState>()
            .init_state::<GameTurnSteps>()
            .register_type::<CurrentActorToken>()
            .register_all_actions()
            .init_resource::<PendingActions>()
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
                remove_moves.in_set(PreparingActions::TrimWrongMoves),
            )
            .add_systems(
                Update,
                select_action.run_if(in_state(GameTurnSteps::ActionSelection)),
            )
            .add_systems(
                Update,
                execute_pending_action.run_if(in_state(GameTurnSteps::PerformAction)),
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
    let mut query = world
        .query_filtered::<(Entity, &Piece, &PiecePos, Option<&Melee>), With<CurrentActorToken>>();
    let Ok((entity, _piece, pos, melee)) = query.get_single(world) else {
        return;
    };

    info!("Found piece!");
    let dirs = vec![
        (KeyCode::KeyD, Vector2Int::new(-1, 0)),
        (KeyCode::KeyA, Vector2Int::new(1, 0)),
        (KeyCode::KeyW, Vector2Int::new(0, 1)),
        (KeyCode::KeyS, Vector2Int::new(0, -1)),
    ];
    let mut possible_actions: Vec<Box<dyn Action>> = Vec::new();

    for (key_code, movement) in dirs {
        let target_pos = pos.0 + movement;
        let walk = WalkAction(entity, target_pos, key_code);

        possible_actions.push(Box::new(walk));

        if let Some(melee_attack) = melee {
            let attack = MeleeHitAction {
                attacker: entity,
                target: target_pos,
                damage: melee_attack.damage,
                key: Some(key_code),
            };
            possible_actions.push(Box::new(attack));
        }
    }
    world
        .entity_mut(entity)
        .insert(PossibleActions(possible_actions))
        .insert(ActionsToRemove::default());
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

fn select_action(
    keys: ResMut<ButtonInput<KeyCode>>,
    mut q: Query<&mut PossibleActions>,
    mut next_state: ResMut<NextState<GameTurnSteps>>,
    mut action_queue: ResMut<PendingActions>,
) {
    let Ok(mut actions) = q.get_single_mut() else {
        return;
    };
    let mut action_index = None;
    for (index, action) in actions.0.iter().enumerate() {
        if let Some(key) = action.get_key_code() {
            if keys.just_released(key) {
                action_index = Some(index);
            }
        }
    }
    if action_index.is_some() {
        let action_moved = actions.0.remove(action_index.unwrap());
        action_queue.push_back(action_moved);
        next_state.set(GameTurnSteps::PerformAction);
    }
}

fn execute_pending_action(world: &mut World) {
    let Some(mut actions) = world.get_resource_mut::<PendingActions>() else {
        return;
    };

    let Some(action) = actions.pop_front() else {
        let Some(mut state) = world.get_resource_mut::<NextState<GameTurnSteps>>() else {
            return;
        };
        state.set(GameTurnSteps::ActionSelection);
        return;
    };
    if !action.execute(world) {
        error!("Error during action ");
    };
}
