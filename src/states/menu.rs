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
                    exit.send(bevy::app::AppExit::Success);
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
        .spawn((
            BackgroundColor::from(Srgba::hex("472D3C").unwrap()),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .insert(Name::new("menu-root"))
        .insert(MenuRoot)
        .with_children(|parent| {
            parent.spawn((
                TextFont {
                    font: asset_server.load(consts::BASE_FONT),
                    font_size: 55.0,
                    ..default()
                },
                TextColor(Srgba::hex("CFC6B8").unwrap().into()),
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    margin: UiRect {
                        top: Val::Percent(5.0),
                        bottom: Val::Auto,
                        ..default()
                    },
                    ..default()
                },
                Text::new("RougePush"),
            ));

            let btn_text_style = (
                TextFont {
                    font: asset_server.load(consts::BASE_FONT),
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Srgba::hex("CFC6B8").unwrap().into()),
            );

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
                        Button,
                        ImageNode {
                            color: Srgba::hex("7A444A").unwrap().into(),
                            image: asset_server.load("ui/panel-024.png"),
                            image_mode: bevy::ui::widget::NodeImageMode::Sliced(TextureSlicer {
                                // The image borders are 20 pixels in every direction
                                border: BorderRect::square(22.0),
                                center_scale_mode: SliceScaleMode::Stretch,
                                sides_scale_mode: SliceScaleMode::Stretch,
                                max_corner_scale: 1.0,
                            }),
                            ..default()
                        },
                        Node {
                            width: Val::Px(250.0),
                            height: Val::Px(80.0),
                            margin,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        Name::new(format!("button:{}", text)),
                        label,
                        GameButton::default(),
                    ))
                    .with_child((Text::new(text), btn_text_style.clone()));
            }
        });
}
