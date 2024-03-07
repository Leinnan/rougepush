use crate::{consts, FaceCamera, ImageAssets};
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use rand::prelude::SliceRandom;

use super::{GameObject, Piece, PiecePos, PlayerControl};

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
            GameObject,
        ));
        if piece == &Piece::Player {
            entity_cmd.insert(bevy_third_person_camera::ThirdPersonCameraTarget);
        }
    }
}

pub fn update_piece(
    mut query: Query<
        (&PiecePos, &mut Transform),
        (Changed<PiecePos>, Without<crate::board::MapTile>),
    >,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(pos.x as f32, 0.5, pos.y as f32);
    }
}

pub fn update_tile_visibility(
    player_query: Query<(&PiecePos, &PlayerControl), Changed<PiecePos>>,
    mut q: Query<(&mut Visibility, &PiecePos)>,
) {
    let Ok((player_pos, _)) = player_query.get_single() else {
        return;
    };
    let max_distance = 8;

    for (mut visibility, pos) in q.iter_mut() {
        *visibility = if pos.manhattan(**player_pos) <= max_distance {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

pub fn dig_the_grave(
    mut removed: RemovedComponents<Piece>,
    mut query: Query<(&mut TextureAtlas, &mut Transform)>,
) {
    for e in removed.read() {
        let Ok((mut atlas, mut transform)) = query.get_mut(e) else {
            return;
        };
        atlas.index = *consts::GRAVES.choose(&mut rand::thread_rng()).unwrap();
        transform.translation += Vec3::NEG_Y * 0.2;
    }
}
