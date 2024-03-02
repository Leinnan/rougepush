// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams, Sprite3dPlugin};

mod consts;
mod debug;

#[derive(Resource, AssetCollection)]
struct ImageAssets {
    #[asset(path = "ducky.png")]
    duck: Handle<Image>,
    #[asset(path = "colored-transparent_packed.png")]
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

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((Sprite3dPlugin, debug::DebugPlugin))
        .add_systems(OnEnter(MyStates::Next), add_sprite)
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::rgb(0.09, 0.09, 0.13)))
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn add_sprite(mut commands: Commands, assets: Res<ImageAssets>, mut sprite_params: Sprite3dParams) {
    commands
        .spawn(
            Sprite3d {
                image: assets.duck.clone(),
                pixels_per_metre: 100.,
                alpha_mode: AlphaMode::Blend,
                transform: Transform::from_xyz(0., 1., 0.),
                // pivot: Some(Vec2::new(0.5, 0.5)),
                ..default()
            }
            .bundle(&mut sprite_params),
        )
        .insert(FaceCamera);
}

fn generate_world(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {
}
