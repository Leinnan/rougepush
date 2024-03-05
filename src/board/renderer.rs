use crate::{FaceCamera, ImageAssets};
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams};

use super::{GameObject, Piece, PiecePos};

pub fn spawn_piece_renderer(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
    query: Query<(Entity, &PiecePos, &Piece), Added<Piece>>,
) {
    for (entity, pos, piece) in query.iter() {
        let atlas = TextureAtlas {
            layout: assets.layout.clone(),
            index: if piece == &Piece::Player { 26 } else { 125 },
        };
        let mut entity_cmd = commands.entity(entity);
        entity_cmd.insert((
            Sprite3d {
                image: assets.image_transparent.clone(),
                pixels_per_metre: 16.,
                double_sided: true,
                transform: Transform::from_xyz(pos.0.x as f32, 0.5, pos.0.y as f32),
                ..default()
            }
            .bundle_with_atlas(&mut sprite_params, atlas),
            Name::new(format!("{:?}", &piece)),
            FaceCamera,
            GameObject
        ));
        if piece == &Piece::Player {
            entity_cmd.insert(bevy_third_person_camera::ThirdPersonCameraTarget);
        }
    }
}

pub fn update_piece(mut query: Query<(&PiecePos, &mut Transform), Changed<PiecePos>>) {
    for (pos, mut transofrm) in query.iter_mut() {
        transofrm.translation = Vec3::new(pos.x as f32, 0.5, pos.y as f32);
    }
}
