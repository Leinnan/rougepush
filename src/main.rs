// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams, Sprite3dPlugin};

#[derive(Resource, AssetCollection)]
struct ImageAssets {
    #[asset(path = "ducky.png")]
    duck: Handle<Image>,
    #[asset(path = "colored-transparent_packed.png")]
    image: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 49, rows = 22,))]
    layout: Handle<TextureAtlasLayout>,
}

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
        .add_plugins(Sprite3dPlugin)
        .add_systems(OnEnter(MyStates::Next), add_sprite)
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::rgb(0.09, 0.09, 0.13)))
        .insert_resource(Msaa::Off)
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<ImageAssets>(),
        )
        .run();
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
    commands.spawn(
        Sprite3d {
            image: assets.duck.clone(),

            pixels_per_metre: 150.,

            alpha_mode: AlphaMode::Blend,

            unlit: false,

            transform: Transform::from_xyz(0., 0.5, 0.),
            // pivot: Some(Vec2::new(0.5, 0.5)),
            ..default()
        }
        .bundle(&mut sprite_params),
    );
}
