use bevy::prelude::*;

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
pub struct ActionInfo {
    pub action: InputAction,
    pub description: String,
}

#[derive(Component, Reflect)]
pub struct Compass(pub Entity);

pub struct GameGuiPlugin;

impl Plugin for GameGuiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CurrentActorInfo>()
            .register_type::<DeathScreenButton>()
            .register_type::<ActionInfo>()
            .add_systems(OnEnter(MainGameState::Game), add_actor_info)
            .add_systems(
                OnEnter(GameTurnSteps::ActionSelection),
                update_info.after(PreparingActions::TrimWrongMoves),
            )
            .add_systems(
                OnExit(GameTurnSteps::ActionSelection),
                despawn_recursive_by_component::<ActionInfo>,
            )
            .add_systems(
                Update,
                (
                    death_screen::handle_death_menu_buttons,
                    death_screen::create_death_screen,
                    spawn_action_info,
                    insert_compass,
                    update_compass_pos,
                ),
            );
    }
}

fn add_actor_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .insert(GameObject)
        .insert(CurrentActorInfoRoot)
        .with_children(|root| {
            root.spawn(
                TextBundle::from_section(
                    "".to_string(),
                    TextStyle {
                        font: asset_server.load(BASE_FONT),
                        font_size: 20.0,
                        color: MY_ACCENT_COLOR,
                    },
                )
                .with_text_justify(JustifyText::Left),
            )
            .insert(CurrentActorInfo);
        });
}

fn update_info(
    mut commands: Commands,
    mut q: Query<(&mut Text, &Parent), With<CurrentActorInfo>>,
    q2: Query<(&PossibleActions, Option<&PlayerControl>, &Piece), With<CurrentActorToken>>,
) {
    let Ok((mut t, parent)) = q.get_single_mut() else {
        return;
    };
    let Ok((possible_actions, player_control, piece)) = q2.get_single() else {
        return;
    };

    if player_control.is_some() {
        commands.entity(**parent).with_children(|p| {
            for a in possible_actions.0.iter() {
                let Some(action) = a.get_input() else {
                    continue;
                };
                p.spawn(ActionInfo {
                    action,
                    description: format!("{:?}", a.action_type()),
                })
                .insert(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        padding: UiRect::all(Val::Px(5.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                });
            }
        });
    }
    t.sections[0].value = format!("{:?} turn\n", piece);
}

fn spawn_action_info(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q: Query<(Entity, &ActionInfo), Added<ActionInfo>>,
) {
    for (e, info) in q.iter() {
        commands.entity(e).with_children(|r| {
            let img = match info.action {
                InputAction::Left => "ui/keyboard_arrows_left_outline.png".to_owned(),
                InputAction::Right => "ui/keyboard_arrows_right_outline.png".to_owned(),
                InputAction::Up => "ui/keyboard_arrows_up_outline.png".to_owned(),
                InputAction::Down => "ui/keyboard_arrows_down_outline.png".to_owned(),
            };
            r.spawn(ImageBundle {
                image: UiImage::new(asset_server.load(img)),
                background_color: MY_ACCENT_COLOR.with_a(0.6).into(),
                ..default()
            });
            r.spawn(TextBundle::from_section(
                info.description.clone(),
                TextStyle {
                    font: asset_server.load(BASE_FONT),
                    font_size: 12.0,
                    color: MY_ACCENT_COLOR.with_a(0.6),
                },
            ));
        });
    }
}

fn insert_compass(
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    q: Query<(Entity, &PiecePos), Added<PlayerControl>>,
) {
    for (e, pos) in q.iter() {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Plane3d::default().mesh().size(1.0, 1.0)),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load("ui/windrose.png")),
                    alpha_mode: AlphaMode::Mask(0.5),
                    // unlit: true,
                    base_color: Color::WHITE.with_a(0.5),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_xyz(pos.x as f32, 0.1, pos.y as f32),
                ..default()
            })
            .insert(Compass(e));
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
        *t = Transform::from_xyz(pos.x as f32, 0.1, pos.y as f32);
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
