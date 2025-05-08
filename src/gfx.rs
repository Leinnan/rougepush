use bevy::prelude::*;
use bevy_sprite3d::Billboard;

use crate::{states::MainGameState, ImageAssets};

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GameBillboards {
    pub billboard: Handle<Billboard>,
    pub billboard_mat: Handle<StandardMaterial>,
    pub transparent_billboard: Handle<Billboard>,
    pub billboard_transparent_mat: Handle<StandardMaterial>,
    pub fire_billboard: Handle<Billboard>,
    pub unlit_mat: Handle<StandardMaterial>,
}

pub struct GfxPlugin;

impl Plugin for GfxPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameBillboards>();
        app.add_systems(OnEnter(MainGameState::Menu), build_billboards);
    }
}

fn build_billboards(
    mut commands: Commands,
    mut billboards: ResMut<Assets<Billboard>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<ImageAssets>,
) {
    let billboard = billboards.add(Billboard::with_texture_atlas(
        assets.image.clone(),
        assets.layout.clone(),
        16.,
        None,
        false,
    ));
    let transparent_billboard = billboards.add(Billboard::with_texture_atlas(
        assets.image_transparent.clone(),
        assets.layout.clone(),
        16.,
        None,
        true,
    ));
    let fire_billboard = billboards.add(Billboard::with_texture_atlas(
        assets.fire.clone(),
        assets.fire_layout.clone(),
        96.,
        None,
        true,
    ));
    let billboard_mat = materials.add(StandardMaterial {
        base_color_texture: assets.image.clone().into(),
        base_color: Color::LinearRgba(LinearRgba::new(0.486, 0.385, 0.223, 1.000)),
        ..bevy_sprite3d::utils::material()
    });
    let billboard_transparent_mat = materials.add(StandardMaterial {
        base_color_texture: assets.image_transparent.clone().into(),
        double_sided: true,
        ..bevy_sprite3d::utils::material()
    });
    let unlit_mat = materials.add(StandardMaterial {
        double_sided: true,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        base_color_texture: assets.fire.clone().into(),
        ..bevy_sprite3d::utils::material()
    });

    commands.insert_resource(GameBillboards {
        billboard,
        transparent_billboard,
        fire_billboard,
        billboard_mat,
        billboard_transparent_mat,
        unlit_mat,
    });
}
