use crate::{lights::Torch, states, FaceCamera, ImageAssets};
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use components::*;
use rand::{prelude::SliceRandom, Rng}; // optional if you want movement controls

pub mod components;
pub mod generator;
pub mod renderer;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TileType>()
            .register_type::<Piece>()
            .register_type::<PiecePos>()
            .register_type::<Health>()
            .register_type::<PlayerControl>()
            .register_type::<AiControl>()
            .register_type::<Animation>()
            .register_type::<Melee>()
            .add_systems(
                OnEnter(states::MainGameState::Game),
                (generator::create_map, generate_world).chain(),
            )
            .add_systems(
                Update,
                (
                    renderer::set_materials_colors,
                    animate_sprites,
                    renderer::spawn_piece_renderer,
                    renderer::update_piece,
                    renderer::dig_the_grave,
                    renderer::update_tile_visibility,
                )
                    .run_if(in_state(states::MainGameState::Game)),
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
                        .insert(Name::new(format!("PitWall{}x{}[{}]", pos.x, pos.y, i)))
                        .insert(crate::board::MapTile)
                        .insert(PiecePos(*pos));
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
            .insert(Name::new(format!("Tile{}x{}", x, y)))
            .insert(crate::board::MapTile)
            .insert(PiecePos(*pos));
        let mut rng = rand::thread_rng();

        for el in surounding_elements.iter().filter(|e| e.0.is_none()) {
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
                    .insert(Name::new(format!("Wall{}x{}[{}]", pos.x, pos.y, i)))
                    .insert(crate::board::MapTile)
                    .insert(PiecePos(*pos));
            }

            if (pos.x + pos.y) % 4 == 0 && rng.gen_bool(0.5) {
                let interval_counter = rng.gen_range(15..20);
                let cur_index = rng.gen_range(0..15);
                let max_intensity = rng.gen_range(38_000.0..51_000.0);
                let min_intensity = max_intensity - 11_000.0;
                let flame_index = rng.gen_range(0..5);
                let atlas = TextureAtlas {
                    layout: assets.fire_layout.clone(),
                    index: flame_index,
                };
                commands
                    .spawn((
                        Sprite3d {
                            image: assets.fire.clone(),
                            pixels_per_metre: 196.,
                            double_sided: true,
                            unlit: true,
                            transform: Transform::from_xyz(
                                x + (el.2 * 0.8),
                                1.499,
                                y + (el.3 * 0.8),
                            )
                            .with_rotation(el.1),
                            ..default()
                        }
                        .bundle_with_atlas(&mut sprite_params, atlas.clone()),
                        PiecePos(*pos),
                        Animation {
                            frames: vec![0, 1, 2, 3, 4, 5],
                            current: flame_index,
                            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                        },
                        FaceCamera,
                        Name::new("TORCH"),
                        Torch {
                            cur_index,
                            interval_counter,
                            pattern: "mmmmmaaaaammmmmaaaaaabcdefgabcdefg".chars().collect(),
                            max_intensity,
                            min_intensity,
                        },
                    ))
                    .insert(crate::board::MapTile);
            }
        }
    }
}

fn animate_sprites(time: Res<Time>, mut query: Query<(&mut Animation, &mut TextureAtlas)>) {
    for (mut animation, mut atlas) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            atlas.index = animation.frames[animation.current];
            animation.current += 1;
            animation.current %= animation.frames.len();
        }
    }
}
