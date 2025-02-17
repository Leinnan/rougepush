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
            BackgroundColor(Srgba::hex("2b2b2b").unwrap().into()),
            BorderColor(MY_ACCENT_COLOR),
            Node {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                padding: UiRect::all(Val::Px(50.0)),
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                margin: UiRect::horizontal(Val::Auto),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            GameObject,
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("You are dead :("),
                TextFont {
                    font: asset_server.load(consts::BASE_FONT),
                    font_size: 35.0,
                    ..default()
                },
                TextColor(MY_ACCENT_COLOR),
            ));

            let btn_text_style = TextFont {
                font: asset_server.load(consts::BASE_FONT),
                font_size: 25.0,
                ..default()
            };
            let btn_txt_clr = TextColor(Srgba::hex("ECE3CE").unwrap().into());
            root.spawn((
                Node {
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
                Button,
                BackgroundColor::from(Srgba::hex("4F6F52").unwrap()),
                Name::new("button".to_string()),
                DeathScreenButton::GoToMenu,
                GameButton::default(),
            ))
            .with_child((
                Text::new("Go to menu"),
                btn_txt_clr,
                btn_text_style.clone(),
            ));
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
