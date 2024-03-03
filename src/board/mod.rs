use crate::{states, FaceCamera, ImageAssets};
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use bevy_third_person_camera::controller::*;
use components::{CurrentBoard, TileType};
use rand::prelude::SliceRandom; // optional if you want movement controls

pub mod components;
pub mod generator;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TileType>()
            .add_systems(OnEnter(states::MainGameState::Game), generator::create_map)
            .add_systems(
                Update,
                (generate_world).run_if(in_state(states::MainGameState::Game)),
            );
    }
}

fn generate_world(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
    map: Option<Res<CurrentBoard>>,
) {
    let Some(board) = map else {
        return;
    };
    if !board.is_added() {
        return;
    }
    let map = &board.tiles;
    // random floor tile
    let options_f = [685, 734, 774, 775, 830, 831];
    let f = || *options_f.choose(&mut rand::thread_rng()).unwrap();

    let wall_atlas = TextureAtlas {
        layout: assets.layout.clone(),
        index: 843,
    };

    info!("World generate- floors");
    for (pos, tile_type) in map.iter() {
        let (x, y) = (pos.x as f32, pos.y as f32);

        let surounding_elements = [
            (
                board.get(pos.x, pos.y - 1),
                Quat::from_rotation_y(0.0 * std::f32::consts::PI / 2.0),
                0.0,
                -0.5,
            ),
            (
                board.get(pos.x, pos.y + 1),
                Quat::from_rotation_y(2.0 * std::f32::consts::PI / 2.0),
                0.0,
                0.5,
            ),
            (
                board.get(pos.x - 1, pos.y),
                Quat::from_rotation_y(1.0 * std::f32::consts::PI / 2.0),
                -0.5,
                0.0,
            ),
            (
                board.get(pos.x + 1, pos.y),
                Quat::from_rotation_y(-1.0 * std::f32::consts::PI / 2.0),
                0.5,
                0.0,
            ),
        ];
        if tile_type == &TileType::Pit {
            for el in surounding_elements
                .iter()
                .filter(|e| e.0.is_some() && e.0.unwrap() == &TileType::BaseFloor)
            {
                let (x, y) = (x, y);

                for i in [-1, -2] {
                    commands
                        .spawn(
                            Sprite3d {
                                image: assets.image.clone(),
                                pixels_per_metre: 16.,
                                double_sided: false,
                                transform: Transform::from_xyz(
                                    x + el.2,
                                    i as f32 + 0.499,
                                    y + el.3,
                                )
                                .with_rotation(el.1),
                                ..default()
                            }
                            .bundle_with_atlas(&mut sprite_params, wall_atlas.clone()),
                        )
                        .insert(Name::new(format!("PitWall{}x{}[{}]", pos.x, pos.y, i)));
                }
            }
            continue;
        }

        let atlas = TextureAtlas {
            layout: assets.layout.clone(),
            index: f(),
        };

        commands
            .spawn(
                Sprite3d {
                    image: assets.image.clone(),
                    pixels_per_metre: 16.,
                    double_sided: false,
                    transform: Transform::from_xyz(x, 0.0, y)
                        .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
                    ..default()
                }
                .bundle_with_atlas(&mut sprite_params, atlas),
            )
            .insert(Name::new(format!("Tile{}x{}", x, y)));

        for el in surounding_elements.iter().filter(|e| e.0.is_none()) {
            let (x, y) = (x, y);

            for i in [0, 1] {
                commands
                    .spawn(
                        Sprite3d {
                            image: assets.image.clone(),
                            pixels_per_metre: 16.,
                            double_sided: false,
                            transform: Transform::from_xyz(x + el.2, i as f32 + 0.499, y + el.3)
                                .with_rotation(el.1),
                            ..default()
                        }
                        .bundle_with_atlas(&mut sprite_params, wall_atlas.clone()),
                    )
                    .insert(Name::new(format!("Wall{}x{}[{}]", pos.x, pos.y, i)));
            }
        }
    }
    let atlas_player = TextureAtlas {
        layout: assets.layout.clone(),
        index: 26,
    };

    // Player
    commands.spawn((
        Sprite3d {
            image: assets.image_transparent.clone(),
            pixels_per_metre: 16.,
            double_sided: true,
            transform: Transform::from_xyz(9.0, 0.5, 3.0),
            ..default()
        }
        .bundle_with_atlas(&mut sprite_params, atlas_player),
        bevy_third_person_camera::ThirdPersonCameraTarget,
        ThirdPersonController::default(), // optional if you want movement controls
        Name::new("Player"),
        FaceCamera,
    ));
}
