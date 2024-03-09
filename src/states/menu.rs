use crate::consts;
use crate::states::MainGameState;
use bevy::prelude::*;

use bevy_button_released_plugin::{ButtonReleasedEvent, GameButton};

#[derive(Component)]
pub enum MainMenuButton {
    StartGame,
    #[cfg(not(target_arch = "wasm32"))]
    Exit,
}

#[derive(Component)]
pub struct MenuRoot;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainGameState::Menu), setup_menu)
            .add_systems(
                Update,
                (button_system).run_if(in_state(MainGameState::Menu)),
            )
            .add_systems(OnExit(MainGameState::Menu), cleanup_menu);
    }
}

fn button_system(
    mut reader: EventReader<ButtonReleasedEvent>,
    interaction_query: Query<&MainMenuButton>,
    mut next_state: ResMut<NextState<MainGameState>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    for event in reader.read() {
        if let Ok(button_type) = interaction_query.get(**event) {
            match *button_type {
                MainMenuButton::StartGame => next_state.set(MainGameState::Game),
                #[cfg(not(target_arch = "wasm32"))]
                MainMenuButton::Exit => {
                    exit.send(bevy::app::AppExit);
                }
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    let Ok(menu_root) = query.get_single() else {
        return;
    };
    commands.entity(menu_root).despawn_recursive();
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor::from(Color::hex("3A4D39").unwrap()),
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("menu-root"))
        .insert(MenuRoot)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "RougePush",
                    TextStyle {
                        font: asset_server.load(consts::BASE_FONT),
                        font_size: 55.0,
                        color: Color::hex("ECE3CE").unwrap(),
                    },
                )
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    margin: UiRect {
                        top: Val::Percent(5.0),
                        bottom: Val::Auto,
                        ..default()
                    },
                    ..default()
                }),
            );

            let btn_text_style = TextStyle {
                font: asset_server.load(consts::BASE_FONT),
                font_size: 25.0,
                color: Color::hex("ECE3CE").unwrap(),
            };

            for (text, label, margin) in [
                (
                    "Start Game",
                    MainMenuButton::StartGame,
                    UiRect {
                        top: Val::Auto,
                        bottom: Val::Px(15.0),
                        ..default()
                    },
                ),
                #[cfg(not(target_arch = "wasm32"))]
                (
                    "Exit Game",
                    MainMenuButton::Exit,
                    UiRect {
                        bottom: Val::Px(15.0),
                        ..default()
                    },
                ),
            ] {
                parent
                    .spawn((
                        ButtonBundle {
                            image: asset_server.load("ui/panel-024.png").into(),
                            style: Style {
                                width: Val::Px(250.0),
                                height: Val::Px(80.0),
                                margin,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: BackgroundColor::from(Color::hex("4F6F52").unwrap()),
                            ..default()
                        },
                        ImageScaleMode::Sliced(TextureSlicer {
                            // The image borders are 20 pixels in every direction
                            border: BorderRect::square(22.0),
                            center_scale_mode: SliceScaleMode::Stretch,
                            sides_scale_mode: SliceScaleMode::Stretch,
                            max_corner_scale: 1.0,
                        }),
                        Name::new(format!("button:{}", text)),
                        label,
                        GameButton::default(),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(text, btn_text_style.clone()));
                    });
            }
        });
}
