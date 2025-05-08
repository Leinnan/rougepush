use std::time::Duration;

use crate::gui::extra::button::ButtonReleased;
use crate::states::MainGameState;
use crate::{consts, ObserverExtension};
use bevy::prelude::*;
use bevy_tweening::lens::TransformScaleLens;
use bevy_tweening::{Animator, Tween};

#[derive(Component)]
pub enum MainMenuButton {
    StartGame,
    #[cfg(not(target_arch = "wasm32"))]
    Exit,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainGameState::Menu), setup_menu);
    }
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
            StateScoped(MainGameState::Menu),
        ))
        .insert(Name::new("menu-root"))
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
                        Animator::new(Tween::new(
                            EaseFunction::QuadraticInOut,
                            Duration::from_millis(300),
                            TransformScaleLens {
                                start: Vec3::splat(0.1),
                                end: Vec3::ONE,
                            },
                        )),
                        ImageNode {
                            color: Srgba::hex("7A444A").unwrap().into(),
                            image: asset_server.load("ui/panel-024.png"),
                            image_mode: bevy::ui::widget::NodeImageMode::Sliced(TextureSlicer {
                                // The image borders are 20 pixels in every direction
                                border: BorderRect::all(22.0),
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
                    ))
                    .observe_in_child(on_menu_button)
                    .with_child((Text::new(text), btn_text_style.clone()));
            }
        });
}

fn on_menu_button(
    t: Trigger<ButtonReleased>,
    q: Query<&MainMenuButton>,
    mut next_state: ResMut<NextState<MainGameState>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    if let Ok(button_type) = q.get(t.target()) {
        match *button_type {
            MainMenuButton::StartGame => next_state.set(MainGameState::Game),
            #[cfg(not(target_arch = "wasm32"))]
            MainMenuButton::Exit => {
                exit.write(bevy::app::AppExit::Success);
            }
        }
    }
}
