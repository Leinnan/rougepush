use bevy::prelude::*;

use crate::{
    board::components::*,
    consts::{BASE_FONT, MY_ACCENT_COLOR},
    states::*,
};

#[derive(Component, Reflect)]
pub struct CurrentActorInfo;

pub struct GameGuiPlugin;

impl Plugin for GameGuiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CurrentActorInfo>()
            .add_systems(OnEnter(MainGameState::Game), add_actor_info)
            .add_systems(
                OnEnter(GameTurnSteps::ActionSelection),
                update_info.after(PreparingActions::TrimWrongMoves),
            );
    }
}

fn add_actor_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            TextBundle::from_section(
                "".to_string(),
                TextStyle {
                    font: asset_server.load(BASE_FONT),
                    font_size: 20.0,
                    color: MY_ACCENT_COLOR,
                },
            )
            .with_text_justify(JustifyText::Left)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            }),
        )
        .insert(CurrentActorInfo);
}

fn update_info(
    mut q: Query<&mut Text, With<CurrentActorInfo>>,
    q2: Query<(&PossibleActions, Option<&PlayerControl>, &Piece), With<CurrentActorToken>>,
) {
    let Ok(mut t) = q.get_single_mut() else {
        return;
    };
    let Ok((possible_actions, player_control, piece)) = q2.get_single() else {
        return;
    };
    let mut value = format!("{:?} turn\n", piece);
    if player_control.is_some() {
        for action in possible_actions.0.iter() {
            value.push_str(&format!(
                "{:?}-> {:?}\n",
                action.get_input().unwrap(),
                action.action_type()
            ));
        }
    }
    t.sections[0].value = value;
}
