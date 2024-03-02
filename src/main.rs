// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::time::Duration;

use bevy::{prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams, Sprite3dPlugin};
use bevy_third_person_camera::camera::*;
use bevy_third_person_camera::controller::*; // optional if you want movement controls
use bevy_third_person_camera::*;
use rand::prelude::SliceRandom; // optional for additional camera settings

mod consts;
mod debug;

#[derive(Resource, AssetCollection)]
struct ImageAssets {
    #[asset(path = "colored_packed.png")]
    image: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 49, rows = 22,))]
    layout: Handle<TextureAtlasLayout>,
}

/// Tag entity to make it always face the camera
#[derive(Component)]
struct FaceCamera;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

#[derive(Component)]
struct DelayedStart(pub Timer);

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((Sprite3dPlugin, ThirdPersonCameraPlugin, debug::DebugPlugin))
        .add_systems(OnEnter(MyStates::Next), |mut commands: Commands| {
            commands.spawn(DelayedStart(Timer::new(
                Duration::from_secs(2),
                TimerMode::Once,
            )));
        })
        .add_systems(Update, generate_world.run_if(in_state(MyStates::Next)))
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(consts::BG_COLOR))
        .insert_resource(Msaa::Off)
        .init_state::<MyStates>()
        .add_systems(Update, face_camera)
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<ImageAssets>(),
        )
        .run();
}

fn face_camera(
    cam_query: Query<&Transform, With<Camera>>,
    mut query: Query<&mut Transform, (With<FaceCamera>, Without<Camera>)>,
) {
    let cam_transform = cam_query.single();
    for mut transform in query.iter_mut() {
        let mut delta = cam_transform.translation - transform.translation;
        delta.y = 0.0;
        delta += transform.translation;
        transform.look_at(delta, Vec3::Y);
    }
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Circle::new(4.0)),
    //     material: materials.add(Color::WHITE),
    //     transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    //     ..default()
    // });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn((
        ThirdPersonCamera {
            cursor_lock_key: KeyCode::Space,
            cursor_lock_toggle_enabled: true,
            gamepad_settings: CameraGamepadSettings::default(),
            cursor_lock_active: true,
            mouse_sensitivity: 1.0,
            zoom_enabled: true,
            zoom: Zoom::new(3.5, 5.0),
            zoom_sensitivity: 1.0,
            ..default()
        },
        FogSettings {
            color: consts::BG_COLOR,
            falloff: FogFalloff::ExponentialSquared { density: 0.11 },
            ..default()
        },
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}

fn generate_world(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
    time: Res<Time>,
    mut q: Query<(&mut DelayedStart, Entity)>,
) {
    let Ok((mut timer, entity)) = q.get_single_mut() else {
        return;
    };
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }
    commands.entity(entity).despawn();
    info!("Start world generate");
    // random floor tile
    let options_f = [685, 734, 774, 775];
    let f = || *options_f.choose(&mut rand::thread_rng()).unwrap();

    let options_d = [830, 831]; // random floor tile
    let d = || *options_d.choose(&mut rand::thread_rng()).unwrap();

    let options_l = [(637)]; // left wall tile
    let l = || *options_l.choose(&mut rand::thread_rng()).unwrap();
    let options_t = [(843)]; // top wall tile
    let t = || *options_t.choose(&mut rand::thread_rng()).unwrap();
    let options_b = [(843)]; // bottom wall tile
    let b = || *options_b.choose(&mut rand::thread_rng()).unwrap();
    let options_r = [(843)]; // right wall tile
    let r = || *options_r.choose(&mut rand::thread_rng()).unwrap();

    let tl = || (843); // top left corner
    let tr = || (843); // top right corner
    let bl = || (843); // bottom left corner
    let br = || (843); // bottom right corner

    let options_tb = [(843)]; // top and bottom wall tile
    let tb = || *options_tb.choose(&mut rand::thread_rng()).unwrap();

    // in reality, you'd probably want to import a map generated by an
    // external tool, or maybe proc-gen it yourself. For this example, a
    // 2d array should suffice.
    #[rustfmt::skip] // irony - we do it to make it more readable 
    let mut map = vec![
        vec![(0), (0), (0), (0), (0), tl(),  t(),   d(),   d(),   d(),   t(),   tr() ],
        vec![(0), (0), (0), (0), (0), l(),   f(),   f(),   f(),   f(),   f(),   r()  ],
        vec![(0), (0), (0), (0), (0), d(),   f(),   d(),   d(),   d(),   f(),   d()  ],
        vec![(0), (0), (0), (0), (0), d(),   f(),   d(),   d(),   d(),   f(),   d()  ],
        vec![(0), (0), (0), (0), (0), d(),   f(),   d(),   d(),   d(),   f(),   d()  ],
        vec![(0), (0), (0), (0), (0), l(),   f(),   f(),   f(),   f(),   f(),   r()  ],
        vec![(0), (0), (0), (0), (0), bl(),  b(), 847,  d(), 847,  b(),   br() ],
        vec![(0), (0), (0), (0), (0), (0), (0), l(),   f(),   r(),   (0), (0)],
        vec![(0), (0), (0), (0), (0), (0), (0), l(),   d(),   r(),   (0), (0)],
        vec![(0), (0), (0), (0), (0), (0), tl(), 847, f(),  847, tr(),  (0)],
        vec![(0), (0), (0), (0), (0), (0), l(),   f(),   d(),   f(),   r(),   (0)],
        vec![(0), (0), (0), (0), (0), (0), l(),   f(),   f(),   f(),   r(),   (0)],
        vec![(0), (0), (0), (0), (0), (0), l(),   f(),   d(),   f(),   r(),   (0)],
        vec![(0), (0), (0), (0), (0), (0), l(),   f(),   f(),   f(),   r(),   (0)],
        vec![tl(),  t(),    tr(), (0), (0), (0), l(),   f(),   f(),   f(),   r(),   (0)],
        vec![l(),   f(),  847,  tb(),  tb(),  tb(), 847,f(),   f(),   f(),   r(),   (0)],
        vec![bl(),  b(),    br(), (0), (0), (0), bl(),  b(),   b(),   b(),   br(),  (0)],
    ];

    // add zero padding to the map
    map.insert(0, vec![0; map[0].len()]);
    map.push(vec![0; map[0].len()]);
    for row in map.iter_mut() {
        row.insert(0, 0);
        row.push(0);
    }

    // might be nice to add built-in support for sprite-merging for tilemaps...
    // though since all the meshes and materials are already cached and reused,
    // I wonder how much of a speedup that'd actually be. Food for thought.

    info!("World generate- floors");
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let index = map[y][x];
            let (x, y) = (
                x as f32 - map[y].len() as f32 / 2.0,
                y as f32 - map.len() as f32 / 2.0,
            );
            if index == 0 {
                continue;
            }

            let atlas = TextureAtlas {
                layout: assets.layout.clone(),
                index: index as usize,
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
                .insert(Name::new(format!("{}x{}", x, y)));
        }
    }
    let atlas_player = TextureAtlas {
        layout: assets.layout.clone(),
        index: 26,
    };

    // Player
    commands.spawn((
        Sprite3d {
            image: assets.image.clone(),
            pixels_per_metre: 16.,
            double_sided: true,
            transform: Transform::from_xyz(2.0, 0.5, 5.0),
            ..default()
        }
        .bundle_with_atlas(&mut sprite_params, atlas_player),
        ThirdPersonCameraTarget,
        ThirdPersonController::default(), // optional if you want movement controls
        Name::new("Player"),
    ));

    // --------------------------- add some walls -------------------------

    // first horizontally, then vertically, scan along the map. If we find
    // a point transitioning from (0) to something else, add a wall there.

    info!("World generate- walls");
    for y in 1..(map.len() - 1) {
        for x in 0..(map[y].len() - 1) {
            if (map[y][x] != (0)) ^ (map[y][x + 1] == (0)) {
                continue;
            }
            let dir = if map[y][x] == (0) { 1.0 } else { -1.0 };

            let (x, y) = (
                x as f32 - map[y].len() as f32 / 2.0,
                y as f32 - map.len() as f32 / 2.0,
            );

            for i in [0, 1] {
                // add bottom and top piece
                let atlas = TextureAtlas {
                    layout: assets.layout.clone(),
                    index: 843,
                };

                commands.spawn(
                    Sprite3d {
                        image: assets.image.clone(),
                        pixels_per_metre: 16.,
                        double_sided: false,
                        transform: Transform::from_xyz(x + 0.5, i as f32 + 0.499, y)
                            .with_rotation(Quat::from_rotation_y(dir * std::f32::consts::PI / 2.0)),
                        ..default()
                    }
                    .bundle_with_atlas(&mut sprite_params, atlas),
                );
            }
        }
    }

    // same thing again, but for the vertical walls
    for x in 1..(map[0].len() - 1) {
        for y in 0..(map.len() - 1) {
            if (map[y][x] != (0)) ^ (map[y + 1][x] == (0)) {
                continue;
            }
            let dir = if map[y][x] == (0) { 1.0 } else { -1.0 };

            let (x, y) = (
                x as f32 - map[y].len() as f32 / 2.0,
                y as f32 - map.len() as f32 / 2.0,
            );

            for i in [0, 1] {
                // add bottom and top piece
                let atlas = TextureAtlas {
                    layout: assets.layout.clone(),
                    index: 843,
                };

                commands.spawn(
                    Sprite3d {
                        image: assets.image.clone(),
                        pixels_per_metre: 16.,
                        double_sided: false,
                        transform: Transform::from_xyz(x, i as f32 + 0.499, y + 0.5).with_rotation(
                            Quat::from_rotation_y((dir - 1.0) * std::f32::consts::PI / 2.0),
                        ),
                        ..default()
                    }
                    .bundle_with_atlas(&mut sprite_params, atlas),
                );
            }
        }
    }
    info!("World generate- completed");
}
