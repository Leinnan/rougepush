use bevy::prelude::*;
use bevy_button_released_plugin::{ButtonReleasedEvent, GameButton};

use crate::consts::{self, MY_ACCENT_COLOR};

use super::{GameObject, MainGameState, PlayerIsDeadEvent};

#[derive(Component, Reflect)]
pub enum DeathScreenButton {
    GoToMenu,
}

pub fn create_death_screen(
    ev: EventReader<PlayerIsDeadEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if ev.is_empty() {
        return;
    }
    commands
        .spawn((
            NodeBundle {
                background_color: Color::hex("2b2b2b").unwrap().into(),
                border_color: MY_ACCENT_COLOR.into(),
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    padding: UiRect::all(Val::Px(50.0)),
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(3.0)),
                    margin: UiRect::horizontal(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            GameObject,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "You are dead :(",
                TextStyle {
                    font: asset_server.load(consts::BASE_FONT),
                    font_size: 35.0,
                    color: MY_ACCENT_COLOR,
                },
            ));

            let btn_text_style = TextStyle {
                font: asset_server.load(consts::BASE_FONT),
                font_size: 25.0,
                color: Color::hex("ECE3CE").unwrap(),
            };
            root.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect {
                            top: Val::Px(15.0),
                            bottom: Val::Px(15.0),
                            ..default()
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },

                    background_color: BackgroundColor::from(Color::hex("4F6F52").unwrap()),
                    ..default()
                },
                Name::new("button".to_string()),
                DeathScreenButton::GoToMenu,
                GameButton::default(),
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Go to menu",
                    btn_text_style.clone(),
                ));
            });
        });
}

pub fn handle_death_menu_buttons(
    mut reader: EventReader<ButtonReleasedEvent>,
    interaction_query: Query<&DeathScreenButton>,
    mut next_state: ResMut<NextState<MainGameState>>,
) {
    for event in reader.read() {
        if interaction_query.get(**event).is_ok() {
            next_state.set(MainGameState::Menu);
        }
    }
}
