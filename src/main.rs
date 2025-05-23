// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_sprite3d::Sprite3dPlugin;
use bevy_third_person_camera::*;

mod actions;
mod board;
mod consts;
#[cfg(not(target_arch = "wasm32"))]
mod debug;
mod dungeon;
mod gfx;
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
    #[asset(texture_atlas(tile_size_x = 16, tile_size_y = 16, columns = 49, rows = 22,))]
    layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas(tile_size_x = 58, tile_size_y = 72, columns = 3, rows = 2,))]
    fire_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "fire.png")]
    fire: Handle<Image>,
}

/// Tag entity to make it always face the camera
#[derive(Component)]
struct FaceCamera;

pub trait ObserverExtension {
    fn observe_in_child<E: Event, B: Bundle, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self;
}

impl ObserverExtension for EntityCommands<'_> {
    fn observe_in_child<E: Event, B: Bundle, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self {
        self.with_children(|p| {
            let entity = p.target_entity();
            p.spawn(Observer::new(system).with_entity(entity));
        })
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            gfx::GfxPlugin,
            Sprite3dPlugin,
            ThirdPersonCameraPlugin,
            board::BoardPlugin,
            input::InputPlugin,
            lights::LightsPlugin,
            states::GameStatesPlugin,
            gui::GameGuiPlugin,
        ))
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(consts::BG_COLOR))
        .add_systems(FixedUpdate, face_camera)
        .add_loading_state(
            LoadingState::new(states::MainGameState::AssetLoading)
                .continue_to_state(states::MainGameState::Menu)
                .load_collection::<ImageAssets>(),
        );

    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Startup, set_window_icon)
        .add_plugins(debug::plugin);
    app.run();
}

fn face_camera(
    cam_transform: Single<&Transform, With<Camera>>,
    mut query: Query<&mut Transform, (With<FaceCamera>, Without<Camera>)>,
) {
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
        commands.entity(e).despawn();
    }
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn((
        ThirdPersonCamera {
            cursor_lock_key: KeyCode::Space,
            cursor_lock_toggle_enabled: true,
            cursor_lock_active: false,
            zoom_enabled: true,
            zoom: Zoom::new(4.5, 7.0),
            zoom_sensitivity: 1.0,
            ..default()
        },
        DistanceFog {
            color: consts::BG_COLOR,
            falloff: FogFalloff::Linear {
                start: 5.0,
                end: 11.5,
            },
            ..default()
        },
        Msaa::Off,
        Transform::from_xyz(0.0, 15.9, -33.2).looking_at(Vec3::ZERO, Vec3::Y),
        bevy::core_pipeline::tonemapping::Tonemapping::AgX,
        Camera3d::default(),
    ));
}

#[cfg(not(target_arch = "wasm32"))]
fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<bevy::winit::WinitWindows>,
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
    let icon = winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
