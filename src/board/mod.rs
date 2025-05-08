use std::time::Duration;

use crate::{
    gfx::GameBillboards,
    lights::{LightPattern, Torch},
    states::{self, GameTurnSteps},
    vectors::Vector2Int,
    FaceCamera, ImageAssets,
};
use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_sprite3d::{Sprite3d, Sprite3dBillboard};
use components::*;
use rand::{prelude::SliceRandom, Rng}; // optional if you want movement controls

pub mod components;
pub mod generator;
pub mod renderer;

pub struct BoardPlugin;

pub struct BoardPieceToGen {
    pub tile_type: TileType,
    pub pos: Vector2Int,
    pub neighbours: [(BoardNeighbour, TileType); 4],
}

impl BoardPieceToGen {
    pub fn get_walls_transforms(&self) -> Vec<Transform> {
        match self.tile_type {
            TileType::BaseFloor => self
                .neighbours
                .iter()
                .filter(|e| e.1.eq(&TileType::None))
                .flat_map(|e| [1.0, 0.0].iter().map(|i| self.transform(e.0, i + 0.499)))
                .collect(),
            TileType::Pit => self
                .neighbours
                .iter()
                .filter(|e| e.1.eq(&TileType::BaseFloor))
                .flat_map(|e| [-1.0, -2.0].iter().map(|i| self.transform(e.0, i + 0.499)))
                .collect(),
            TileType::None => vec![],
        }
    }
    pub fn transform(&self, neighbour: BoardNeighbour, y_offset: f32) -> Transform {
        Transform::from_xyz(
            self.pos.x as f32 + neighbour.x_offset(),
            y_offset,
            self.pos.y as f32 + neighbour.z_offset(),
        )
        .with_rotation(neighbour.rotation())
    }
    pub fn from_pos(pos: Vector2Int, board: &CurrentBoard) -> Option<BoardPieceToGen> {
        let tile = board.get(pos.x, pos.y)?;
        if tile.eq(&TileType::None) {
            return None;
        }
        let up = board
            .get(pos.x, pos.y + 1)
            .map_or((BoardNeighbour::Up, TileType::None), |t| {
                (BoardNeighbour::Up, t.clone())
            });
        let down = board
            .get(pos.x, pos.y - 1)
            .map_or((BoardNeighbour::Down, TileType::None), |t| {
                (BoardNeighbour::Down, t.clone())
            });
        let left = board
            .get(pos.x - 1, pos.y)
            .map_or((BoardNeighbour::Left, TileType::None), |t| {
                (BoardNeighbour::Left, t.clone())
            });
        let right = board
            .get(pos.x + 1, pos.y)
            .map_or((BoardNeighbour::Right, TileType::None), |t| {
                (BoardNeighbour::Right, t.clone())
            });

        Some(Self {
            tile_type: tile.clone(),
            pos,
            neighbours: [up, down, left, right],
        })
    }
}

#[derive(Reflect, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum BoardNeighbour {
    Up,
    Down,
    Left,
    Right,
}

impl BoardNeighbour {
    pub fn rotation(&self) -> Quat {
        match self {
            BoardNeighbour::Up => Quat::from_rotation_y(2.0 * std::f32::consts::PI / 2.0),
            BoardNeighbour::Down => Quat::from_rotation_y(0.0 * std::f32::consts::PI / 2.0),
            BoardNeighbour::Left => Quat::from_rotation_y(1.0 * std::f32::consts::PI / 2.0),
            BoardNeighbour::Right => Quat::from_rotation_y(-1.0 * std::f32::consts::PI / 2.0),
        }
    }

    pub fn x_offset(&self) -> f32 {
        match self {
            BoardNeighbour::Left => -0.5,
            BoardNeighbour::Right => 0.5,
            _ => 0.0,
        }
    }

    pub fn z_offset(&self) -> f32 {
        match self {
            BoardNeighbour::Down => -0.5,
            BoardNeighbour::Up => 0.5,
            _ => 0.0,
        }
    }
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TileType>()
            .register_type::<Piece>()
            .register_type::<PiecePos>()
            .register_type::<Health>()
            .register_type::<PlayerControl>()
            .register_type::<PlayerPiece>()
            .register_type::<AiControl>()
            .register_type::<Animation>()
            .register_type::<Melee>()
            .add_systems(
                Update,
                materials_check.run_if(on_timer(Duration::from_secs(5))),
            )
            .add_systems(
                OnEnter(states::MainGameState::Game),
                (
                    generator::create_map,
                    generator::spawn_points,
                    generate_world,
                    start_search_for_agents,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    animate_sprites,
                    renderer::spawn_piece_renderer,
                    renderer::update_piece,
                    renderer::dig_the_grave,
                    renderer::update_tile_visibility,
                )
                    .run_if(in_state(states::MainGameState::Game)),
            )
            .add_systems(
                Update,
                update_animation.run_if(
                    in_state(states::MainGameState::Game)
                        .and(on_timer(Duration::from_secs_f32(0.1))),
                ),
            )
            .add_systems(OnExit(states::MainGameState::Game), remove_map);
    }
}

fn remove_map(mut commands: Commands, mut next: ResMut<NextState<GameTurnSteps>>) {
    commands.remove_resource::<CurrentBoard>();
    next.set(GameTurnSteps::SearchForAgents);
}

fn start_search_for_agents(mut next: ResMut<NextState<GameTurnSteps>>) {
    next.set(GameTurnSteps::SearchForAgents);
}

fn generate_world(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    billboards: Res<GameBillboards>,
    board: Res<CurrentBoard>,
) {
    use std::time::Instant;

    let start = Instant::now();
    let map = &board.tiles;
    // random floor tile
    let options_f = [685, 734, 774, 775, 830, 831];
    let floor = || *options_f.choose(&mut rand::thread_rng()).unwrap();
    let floor_atlas_gen = || TextureAtlas {
        layout: assets.layout.clone(),
        index: floor(),
    };

    let wall_atlas = TextureAtlas {
        layout: assets.layout.clone(),
        index: 843,
    };
    let wall_sprite = Sprite3d::from(wall_atlas);

    info!("World generate- floors");
    for (pos, tile_type) in map.iter() {
        let Some(surounding_elements) = BoardPieceToGen::from_pos(*pos, &board) else {
            continue;
        };
        let walls: Vec<(
            MeshMaterial3d<StandardMaterial>,
            Sprite3dBillboard,
            Sprite3d,
            Transform,
            MapTile,
            PiecePos,
        )> = surounding_elements
            .get_walls_transforms()
            .iter()
            .map(|t| {
                (
                    MeshMaterial3d(billboards.billboard_mat.clone()),
                    Sprite3dBillboard::new(billboards.billboard.clone()),
                    wall_sprite.clone(),
                    *t,
                    crate::board::MapTile,
                    PiecePos(*pos),
                )
            })
            .collect();
        commands.spawn_batch(walls);
        let (x, y) = (pos.x as f32, pos.y as f32);

        if tile_type == &TileType::Pit {
            continue;
        }

        commands
            .spawn((
                MeshMaterial3d(billboards.billboard_mat.clone()),
                Sprite3dBillboard::new(billboards.billboard.clone()),
                Sprite3d::from(floor_atlas_gen()),
                Transform::from_xyz(x, 0.0, y)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
            ))
            .insert(Name::new(format!("Tile{}x{}", x, y)))
            .insert(crate::board::MapTile)
            .insert(PiecePos(*pos));
        let mut rng = rand::thread_rng();

        for el in surounding_elements
            .neighbours
            .iter()
            .filter(|e| e.1.eq(&TileType::None))
        {
            if (pos.x + pos.y) % 4 == 0 && rng.gen_bool(0.5) {
                let interval_counter = rng.gen_range(15..20);
                let cur_index = rng.gen_range(0..15);
                let min_intensity = rng.gen_range(25_000.0..40_000.0);
                let flame_index = rng.gen_range(0..5);
                let atlas = TextureAtlas {
                    layout: assets.fire_layout.clone(),
                    index: flame_index,
                };
                commands
                    .spawn((
                        MeshMaterial3d(billboards.unlit_mat.clone()),
                        Sprite3dBillboard::new(billboards.fire_billboard.clone()),
                        Sprite3d::from(atlas),
                        Transform::from_xyz(
                            x + (el.0.x_offset() * 0.8),
                            1.499,
                            y + (el.0.z_offset() * 0.8),
                        )
                        .with_rotation(el.0.rotation()),
                        PiecePos(*pos),
                        Animation::new_with_index(5, flame_index),
                        FaceCamera,
                        Name::new("TORCH"),
                        LightPattern::from_chars(
                            &"mmmmmaaaaammmmmaaaaaabcdefgabcdefg"
                                .chars()
                                .collect::<Vec<char>>(),
                        ),
                        Torch {
                            cur_index,
                            intensity_variation: 10_000.0,
                            min_intensity,
                            target_intensity: min_intensity,
                            interval_counter,
                        },
                    ))
                    .insert(crate::board::MapTile);
            }
        }
    }
    let duration = start.elapsed();
    error!("World generation: {}", duration.as_micros());
}

fn update_animation(mut query: Query<&mut Animation>) {
    query.par_iter_mut().for_each(|mut a| {
        a.bump_index();
    });
}

fn materials_check(meshes: Res<Assets<Mesh>>, standard_materials: Res<Assets<StandardMaterial>>) {
    warn!("There are {} meshes", meshes.len());
    warn!("There are {} materials", standard_materials.len());
}

fn animate_sprites(mut query: Query<(&Animation, &mut Sprite3d), Changed<Animation>>) {
    for (animation, mut sprite) in query.iter_mut() {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = animation.current();
        }
    }
}
