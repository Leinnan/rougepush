// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_sprite3d::Sprite3dPlugin;
use bevy_third_person_camera::camera::*;
use bevy_third_person_camera::*;

use bevy::winit::WinitWindows;
use winit::window::Icon;

mod actions;
mod board;
mod consts;
mod debug;
mod dungeon;
mod gui;
mod input;
mod lights;
mod states;
mod vectors;

#[derive(Resource, AssetCollection)]
struct ImageAssets {
    #[asset(path = "colored_packed.png")]
    image: Handle<Image>,
    #[asset(path = "colored-transparent_packed.png")]
    image_transparent: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 49, rows = 22,))]
    layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas(tile_size_x = 58., tile_size_y = 72., columns = 3, rows = 2,))]
    fire_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "fire.png")]
    fire: Handle<Image>,
}

/// Tag entity to make it always face the camera
#[derive(Component)]
struct FaceCamera;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            Sprite3dPlugin,
            ThirdPersonCameraPlugin,
            debug::DebugPlugin,
            board::BoardPlugin,
            input::InputPlugin,
            lights::LightsPlugin,
            states::GameStatesPlugin,
            gui::GameGuiPlugin,
        ))
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(consts::BG_COLOR))
        .insert_resource(Msaa::Off)
        .add_systems(FixedUpdate, face_camera)
        .add_loading_state(
            LoadingState::new(states::MainGameState::AssetLoading)
                .continue_to_state(states::MainGameState::Menu)
                .load_collection::<ImageAssets>(),
        );

    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Startup, set_window_icon);
    app.run();
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

pub fn despawn_recursive_by_component<T: bevy::prelude::Component>(
    q: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn((
        ThirdPersonCamera {
            cursor_lock_key: KeyCode::Space,
            cursor_lock_toggle_enabled: true,
            gamepad_settings: CameraGamepadSettings::default(),
            cursor_lock_active: false,
            mouse_sensitivity: 1.0,
            zoom_enabled: true,
            zoom: Zoom::new(4.5, 7.0),
            zoom_sensitivity: 1.0,
            ..default()
        },
        FogSettings {
            color: consts::BG_COLOR,
            falloff: FogFalloff::Linear {
                start: 5.0,
                end: 11.5,
            },
            ..default()
        },
        Camera3dBundle {
            transform: Transform::from_xyz(9.0, 2.9, -2.2).looking_at(Vec3::ZERO, Vec3::Y),
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::AgX,
            ..default()
        },
    ));
}

#[cfg(not(target_arch = "wasm32"))]
fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/game_icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
