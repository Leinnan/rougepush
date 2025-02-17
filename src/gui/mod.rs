use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    board::components::*,
    consts::{BASE_FONT, MY_ACCENT_COLOR},
    despawn_recursive_by_component,
    input::InputAction,
    states::*,
};

use self::death_screen::DeathScreenButton;

mod death_screen;

#[derive(Component, Reflect)]
pub struct CurrentActorInfoRoot;

#[derive(Component, Reflect)]
pub struct CurrentActorInfo;

#[derive(Component, Reflect)]
#[require(Node(action_info_node))]
pub struct ActionInfo {
    pub action: InputAction,
    pub description: String,
}

#[derive(Component, Reflect)]
#[require(Transform)]
pub struct Compass(pub Entity);

#[derive(Resource, Reflect, Default)]
pub struct HelpDisplayEnabled(pub bool);

pub struct GameGuiPlugin;

impl Plugin for GameGuiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CurrentActorInfo>()
            .register_type::<DeathScreenButton>()
            .register_type::<ActionInfo>()
            .insert_resource(HelpDisplayEnabled(true))
            .add_systems(OnEnter(MainGameState::Game), add_actor_info)
            .add_systems(
                OnEnter(GameTurnSteps::ActionSelection),
                update_info.after(PreparingActions::TrimWrongMoves),
            )
            .add_systems(
                OnExit(GameTurnSteps::ActionSelection),
                despawn_recursive_by_component::<ActionInfo>,
            )
            .add_observer(on_action_info_added)
            .add_systems(
                Update,
                (
                    death_screen::handle_death_menu_buttons,
                    death_screen::create_death_screen,
                    insert_compass,
                    switch_help_ui,
                    update_compass_pos,
                ),
            );
    }
}

fn switch_help_ui(
    input: Query<&ActionState<InputAction>>,
    mut help: ResMut<HelpDisplayEnabled>,
    mut q: Query<&mut Visibility, With<ActionInfo>>,
) {
    let Ok(action_state) = input.get_single() else {
        return;
    };

    if !action_state.just_pressed(&InputAction::Hide) {
        return;
    }
    help.0 = !help.0;
    for mut vis in q.iter_mut() {
        *vis = if help.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

fn add_actor_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Vh(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                align_self: AlignSelf::Stretch,
                row_gap: Val::Px(15.0),
                ..default()
            },
            Name::new("CurrentActorInfoRoot"),
        ))
        .insert(GameObject)
        .insert(CurrentActorInfoRoot)
        .with_children(|root| {
            root.spawn((
                TextFont {
                    font: asset_server.load(BASE_FONT),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(MY_ACCENT_COLOR),
                TextLayout::new_with_justify(JustifyText::Left),
                Node {
                    margin: UiRect {
                        top: Val::Percent(5.0),
                        bottom: Val::Auto,
                        ..default()
                    },
                    ..default()
                },
                CurrentActorInfo,
                Text::new(""),
            ));
        });
}

fn update_info(
    mut commands: Commands,
    mut q: Query<(&mut Text, &Parent), With<CurrentActorInfo>>,
    q2: Query<(&PossibleActions, Option<&PlayerControl>, &Piece), With<CurrentActorToken>>,
    help: Res<HelpDisplayEnabled>,
) {
    let Ok((mut t, parent)) = q.get_single_mut() else {
        return;
    };
    let Ok((possible_actions, player_control, piece)) = q2.get_single() else {
        return;
    };
    **t = format!("{:?} turn\n", piece);
    if player_control.is_none() || !help.0 {
        return;
    }
    commands.entity(**parent).with_children(|p| {
        for a in possible_actions.0.iter() {
            let Some(action) = a.get_input() else {
                continue;
            };
            p.spawn(ActionInfo {
                action,
                description: format!("{:?}", a.action_type()),
            });
        }
        p.spawn(ActionInfo {
            action: InputAction::Space,
            description: "Switch camera movement".to_owned(),
        });
        p.spawn(ActionInfo {
            action: InputAction::Hide,
            description: "Hide this info".to_owned(),
        });
    });
}

fn action_info_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        margin: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
        align_items: AlignItems::Start,
        justify_items: JustifyItems::Center,
        ..default()
    }
}

fn on_action_info_added(
    trigger: Trigger<OnAdd, ActionInfo>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q: Query<&ActionInfo>,
) {
    let Ok(info) = q.get(trigger.entity()) else {
        return;
    };
    commands.entity(trigger.entity()).with_children(|r| {
        let img = match info.action {
            InputAction::Left => "ui/keyboard_arrows_left_outline.png".to_owned(),
            InputAction::Right => "ui/keyboard_arrows_right_outline.png".to_owned(),
            InputAction::Up => "ui/keyboard_arrows_up_outline.png".to_owned(),
            InputAction::Down => "ui/keyboard_arrows_down_outline.png".to_owned(),
            InputAction::Space => "ui/keyboard_space_outline.png".to_owned(),
            InputAction::Hide => "ui/keyboard_h_outline.png".to_owned(),
        };
        r.spawn((
            Node::default(),
            ImageNode {
                color: MY_ACCENT_COLOR.with_alpha(0.6),
                image: asset_server.load(img),
                ..default()
            },
        ));
        r.spawn((
            TextFont {
                font: asset_server.load(BASE_FONT),
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb_u8(159, 111, 97)),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                margin: UiRect {
                    top: Val::Percent(5.0),
                    bottom: Val::Auto,
                    ..default()
                },
                ..default()
            },
            Text::new(info.description.clone()),
        ));
    });
}

fn insert_compass(
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    q: Query<(Entity, &PiecePos), Added<PlayerControl>>,
) {
    for (e, pos) in q.iter() {
        let t: Transform = pos.into();
        commands.spawn((
            t,
            Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("ui/windrose.png")),
                alpha_mode: AlphaMode::Mask(0.5),
                // unlit: true,
                base_color: Color::WHITE.with_alpha(0.5),
                unlit: true,
                ..default()
            })),
            Compass(e),
        ));
        // .spawn(PbrBundle {
        //     mesh: meshes.add(Plane3d::default().mesh().size(1.0, 1.0)),
        // material: materials.add(),
        //     transform: Transform::from_xyz(pos.x as f32, 0.1, pos.y as f32),
        //     ..default()
        // })
        // .insert(Compass(e));
    }
}

pub fn update_compass_pos(
    q: Query<&PiecePos, Changed<PiecePos>>,
    mut compass_q: Query<(&mut Transform, &mut Visibility, &Compass)>,
    q_p: Query<&PlayerControl, With<CurrentActorToken>>,
    state: Res<State<GameTurnSteps>>,
) {
    for (mut t, _, compass) in compass_q.iter_mut() {
        let Ok(pos) = q.get(compass.0) else {
            continue;
        };
        *t = pos.into();
    }
    if state.is_changed() {
        let visible = !q_p.is_empty() && *state == GameTurnSteps::ActionSelection;
        for (_, mut vis, _) in compass_q.iter_mut() {
            *vis = if visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}
