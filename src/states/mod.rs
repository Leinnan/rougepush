pub mod menu;
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_button_released_plugin::*;
use leafwing_input_manager::action_state::ActionState;
use std::collections::{BinaryHeap, VecDeque};
use std::ops::DerefMut;

use crate::{
    actions::{melee_hit::MeleeHitAction, walk::WalkAction, Action, ActionType, RegisterActions},
    board::components::*,
    despawn_recursive_by_component,
    input::InputAction,
    vectors::{Vector2Int, ORTHO_DIRECTIONS},
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

#[derive(Deref, DerefMut, Component, Default, Reflect)]
pub struct ActionDelay(pub usize);

#[derive(Event, Default, Reflect)]
pub struct PlayerIsDeadEvent;

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugin, ButtonsReleasedPlugin))
            .init_state::<MainGameState>()
            .init_state::<GameTurnSteps>()
            .register_type::<CurrentActorToken>()
            .add_event::<PlayerIsDeadEvent>()
            .register_type::<ActionDelay>()
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
                (select_action, ai_select_action)
                    .chain()
                    .run_if(in_state(GameTurnSteps::ActionSelection)),
            )
            .add_systems(
                Update,
                execute_pending_action.run_if(in_state(GameTurnSteps::PerformAction)),
            )
            .add_systems(
                Update,
                check_if_player_is_alive.run_if(in_state(MainGameState::Game)),
            )
            .add_systems(OnExit(GameTurnSteps::PerformAction), remove_token)
            .add_systems(
                OnExit(MainGameState::Game),
                (
                    despawn_recursive_by_component::<GameObject>,
                    despawn_recursive_by_component::<Piece>,
                    despawn_recursive_by_component::<Piece>,
                ).chain(),
            );
    }
}

fn find_actor(query: Query<(Entity, &Piece)>, mut next_state: ResMut<NextState<GameTurnSteps>>) {
    if !query.is_empty() {
        next_state.set(GameTurnSteps::ActionSelection);
    }
}

fn set_current_actor(mut commands: Commands, query: Query<(Entity, &ActionDelay), With<Piece>>) {
    info!("set_current_actor");
    let mut lowest_delay = (usize::MAX, Entity::PLACEHOLDER);
    for (entity, delay) in query.iter() {
        if **delay < lowest_delay.0 {
            lowest_delay.0 = **delay;
            lowest_delay.1 = entity;
        }
    }
    if lowest_delay.0 < usize::MAX {
        commands.entity(lowest_delay.1).insert(CurrentActorToken);
    }
}

fn prepare_action_list(world: &mut World) {
    info!("prepare_action_list");
    let mut query = world
        .query_filtered::<(Entity, &Piece, &PiecePos, Option<&Melee>), With<CurrentActorToken>>();
    let Ok((entity, piece, pos, melee)) = query.get_single(world) else {
        return;
    };

    info!("Found piece!");
    let dirs = vec![
        (InputAction::Right, Vector2Int::new(-1, 0)),
        (InputAction::Left, Vector2Int::new(1, 0)),
        (InputAction::Up, Vector2Int::new(0, 1)),
        (InputAction::Down, Vector2Int::new(0, -1)),
    ];
    let mut possible_actions: Vec<Box<dyn Action>> = Vec::new();

    for (key_code, movement) in dirs {
        let target_pos = pos.0 + movement;
        let walk = WalkAction(entity, target_pos, key_code);

        possible_actions.push(Box::new(walk));

        if let Some(melee_attack) = melee {
            let attack = MeleeHitAction {
                attacker: entity,
                attacker_type: piece.clone(),
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
    mut q: Query<(&mut PossibleActions, &ActionsToRemove, Entity), With<CurrentActorToken>>,
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
    mut q: Query<(&mut PossibleActions, &ActionState<InputAction>), With<CurrentActorToken>>,
    mut next_state: ResMut<NextState<GameTurnSteps>>,
    mut action_queue: ResMut<PendingActions>,
) {
    let Ok((mut actions, action_state)) = q.get_single_mut() else {
        return;
    };
    let mut action_index = None;
    for (index, action) in actions.0.iter().enumerate() {
        if let Some(key) = action.get_input() {
            if action_state.just_released(&key) {
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

fn ai_select_action(
    mut q: Query<
        (&PiecePos, &mut PossibleActions, &AiControl, Option<&Flying>),
        With<CurrentActorToken>,
    >,
    mut next_state: ResMut<NextState<GameTurnSteps>>,
    player_query: Query<(&PiecePos, &Piece), With<PlayerControl>>,
    mut action_queue: ResMut<PendingActions>,
    occupier_query: Query<&PiecePos, With<Occupier>>,
    board: Res<CurrentBoard>,
) {
    let Ok((position, mut actions, ai, flying)) = q.get_single_mut() else {
        return;
    };
    let Ok((player_position, _)) = player_query.get_single() else {
        info!("THERE IS NO PLAYER LEFT");
        return;
    };
    let mut action_index = None;

    // find possible path to the player
    let path_to_player = find_path(
        position.0,
        player_position.0,
        &board.tiles.clone(),
        &occupier_query.iter().map(|p| p.0).collect(),
        flying.is_some(),
        ai.max_distance_to_player,
    );
    info!("{:?}", path_to_player);
    for (index, action) in actions.0.iter().enumerate() {
        if action.action_type() == ActionType::MeleeeHit {
            action_index = Some(index);
            break;
        }
        if action.action_type() == ActionType::Walk {
            if let Some(path) = &path_to_player {
                if path.contains(&action.target_pos().unwrap()) {
                    action_index = Some(index);
                }
            }
        }
    }
    if action_index.is_some() {
        let action_moved = actions.0.remove(action_index.unwrap());
        info!(
            "ACTION SELECTED: {:?} -> {:?}",
            action_moved.action_type(),
            action_moved.target_pos()
        );
        action_queue.push_back(action_moved);
    }
    next_state.set(GameTurnSteps::PerformAction);
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
        error!(
            "Error during action: {:?} -> {:?}",
            action.action_type(),
            action.target_pos()
        );
    };
}

fn remove_token(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ActionDelay), With<CurrentActorToken>>,
) {
    let Ok((entity, mut delay)) = query.get_single_mut() else {
        return;
    };
    delay.0 = **delay + 2;
    commands.entity(entity).remove::<CurrentActorToken>();
}

pub fn find_path(
    start: Vector2Int,
    end: Vector2Int,
    tiles: &HashMap<Vector2Int, TileType>,
    blockers: &HashSet<Vector2Int>,
    is_flying: bool,
    max_distance: usize,
) -> Option<VecDeque<Vector2Int>> {
    let mut queue = BinaryHeap::new();
    queue.push(crate::vectors::utils::Node { v: start, cost: 0 });
    let mut visited = HashMap::new();
    visited.insert(start, 0);
    let mut came_from = HashMap::new();

    while let Some(crate::vectors::utils::Node { v, cost }) = queue.pop() {
        if v == end {
            break;
        }
        for dir in ORTHO_DIRECTIONS {
            let n = v + dir;
            let new_cost = cost + 1;
            if !tiles.contains_key(&n) {
                continue;
            }
            match (&tiles[&n], is_flying) {
                (&TileType::Pit, false) => continue,
                (&TileType::None, _) => continue,
                _ => {}
            }
            // we allow the target to be a blocker
            if blockers.contains(&n) && n != end {
                continue;
            }
            match visited.get(&n) {
                Some(c) if *c <= new_cost => (),
                _ => {
                    visited.insert(n, new_cost);
                    queue.push(crate::vectors::utils::Node {
                        v: n,
                        cost: new_cost,
                    });
                    came_from.insert(n, v);
                }
            }
        }
    }
    let mut path = VecDeque::new();
    let mut cur = end;
    while let Some(v) = came_from.get(&cur) {
        path.push_front(cur);
        cur = *v;
        if cur == start && path.len() <= max_distance {
            return Some(path);
        }
    }
    None
}

fn check_if_player_is_alive(
    mut removed: RemovedComponents<Piece>,
    player_query: Query<&PlayerControl>,
    mut ev: EventWriter<PlayerIsDeadEvent>,
) {
    for e in removed.read() {
        if player_query.get(e).is_ok() {
            info!("PLAYER DEAD");
            ev.send(PlayerIsDeadEvent);
        }
    }
}
