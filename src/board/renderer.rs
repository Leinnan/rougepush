use crate::{consts, gfx::GameBillboards, vectors::Vector2Int, FaceCamera, ImageAssets};
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dBillboard};
use rand::prelude::SliceRandom;

use super::{GameObject, Piece, PiecePos, PlayerControl};
const RENDER_DISTANCE: i32 = 10;

pub fn spawn_piece_renderer(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    billboards: Res<GameBillboards>,
    player_query: Query<&PiecePos, With<PlayerControl>>,
    query: Query<(Entity, &PiecePos, &Piece), Added<Piece>>,
) {
    let player_pos = player_query
        .single()
        .map_or(Vector2Int::default(), |e| e.0.to_owned());

    for (entity, pos, piece) in query.iter() {
        let vis = if pos.manhattan(player_pos) <= RENDER_DISTANCE {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        let atlas = TextureAtlas {
            layout: assets.layout.clone(),
            index: if piece == &Piece::Player { 26 } else { 125 },
        };
        let mut entity_cmd = commands.entity(entity);
        entity_cmd.insert((
            Transform::from_xyz(pos.0.x as f32, 0.5, pos.0.y as f32),
            Sprite3d::from(atlas),
            Sprite3dBillboard::new(billboards.transparent_billboard.clone()),
            MeshMaterial3d(billboards.billboard_transparent_mat.clone()),
            Name::new(format!("{:?}", &piece)),
            FaceCamera,
            GameObject,
            vis,
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
    let Ok((player_pos, _)) = player_query.single() else {
        return;
    };
    let player = **player_pos;
    q.par_iter_mut().for_each(|(mut visibility, pos)| {
        let new_vis = if pos.manhattan(player) <= RENDER_DISTANCE {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        visibility.set_if_neq(new_vis);
    });
}

pub fn dig_the_grave(
    mut removed: RemovedComponents<Piece>,
    mut query: Query<(&mut Sprite3d, &mut Transform)>,
) {
    for e in removed.read() {
        let Ok((mut sprite, mut transform)) = query.get_mut(e) else {
            return;
        };
        let atlas = sprite.texture_atlas.as_mut().unwrap();
        atlas.index = *consts::GRAVES.choose(&mut rand::thread_rng()).unwrap();
        transform.translation += Vec3::NEG_Y * 0.2;
    }
}
