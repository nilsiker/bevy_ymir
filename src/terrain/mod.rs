mod utils;

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::utils::HashSet;
use bevy_inspector_egui::Inspectable;
use futures_lite::future;
use noise::{Fbm, Perlin};

use super::components::Chunk;
use super::noise::NoiseConfig;
use super::noise::NoiseMap;
use super::player::PlayerChunk;

use self::utils::{get_landscape_data, ColorConfig, LandscapeData, TextureMode};

pub struct YmirTerrainPlugin {
    pub chunk_distance: i32,
}

#[derive(Resource)]
struct ChunkDistance(i32);

impl Plugin for YmirTerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkPool(HashSet::new()))
            .insert_resource(SpawnedChunks(HashSet::new()))
            .add_startup_system(setup)
            .add_system(spawn_tasks)
            .add_system(remove_terrain.label("ymir_cleanup"))
            .add_system(spawn_chunks.after("ymir_cleanup"))
            .add_system(update_chunk_pool)
            .insert_resource(ChunkDistance(self.chunk_distance));
    }
}

#[derive(Resource)]
struct ChunkPool(HashSet<(i32, i32)>);

#[derive(Resource)]
struct SpawnedChunks(HashSet<(i32, i32)>);

#[derive(Component, Default, Inspectable)]
pub struct Terrain;

#[derive(Resource, Inspectable, Clone)]
pub struct MeshConfig {
    #[inspectable(min = 2, max = 1025)]
    pub grid_size: usize,
    pub scale: f32,
    pub height_multiplier: f32,
    pub texture_mode: TextureMode,
    pub flat_shading: bool,
    pub color_config: ColorConfig,
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            grid_size: 33,
            scale: 256.0,
            height_multiplier: 80.0,
            texture_mode: default(),
            flat_shading: true,
            color_config: default(),
        }
    }
}

fn update_chunk_pool(
    player_chunk: Res<PlayerChunk>,
    chunk_distance: Res<ChunkDistance>,
    mut pool: ResMut<ChunkPool>,
) {
    if player_chunk.is_changed() {
        pool.0.clear();

        let dist = chunk_distance.0;
        let (x, y) = player_chunk.0;

        for nx in x - dist..=x + dist {
            for ny in y - dist..=y + dist {
                if !pool.0.contains(&(nx, ny)) {
                    pool.0.insert((nx, ny));
                }
            }
        }
    }
}

fn spawn_chunks(
    mut commands: Commands,
    query: Query<Entity, With<Terrain>>,
    mesh_config: Res<MeshConfig>,
    mut tasks: Query<(Entity, &mut ComputeMeshImageData)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(entity) = query.get_single() else { return;};
    for (task_entity, mut task) in &mut tasks {
        if let Some((
            (x, y),
            LandscapeData {
                mesh,
                image,
                collider,
            },
        )) = futures_lite::future::block_on(future::poll_once(&mut task.0))
        {
            commands.entity(entity).with_children(|children| {
                let material = StandardMaterial {
                    base_color_texture: Some(images.add(image)),
                    unlit: false,
                    metallic: 0.0,
                    reflectance: 0.1,
                    perceptual_roughness: 1.0,
                    ..default()
                };

                let scale = mesh_config.scale;

                let mut mesh = children.spawn(PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(material),
                    transform: Transform::from_xyz(x as f32 * scale, 0.0, y as f32 * -scale),
                    ..default()
                });

                mesh.insert(Name::new(format!("({x},{y})")))
                    .insert(Chunk { x, y });

                match collider {
                    Some(col) => {
                        mesh.with_children(|children| {
                            let mut transform = Transform::from_scale({
                                let mut vec = Vec3::ONE;
                                vec.z = -vec.z;
                                vec.x = -vec.x;
                                vec
                            });
                            transform.rotation =
                                Quat::from_euler(EulerRot::XYZ, 0.0, -FRAC_PI_2, 0.0);

                            children
                                .spawn(TransformBundle {
                                    local: transform,
                                    ..default()
                                })
                                .insert(col);
                        });
                    }
                    None => todo!(),
                };
            });

            commands.entity(task_entity).despawn_recursive();
        }
    }
}

fn spawn_tasks(
    mut commands: Commands,
    query: Query<Entity, With<Terrain>>,
    mesh_config: Res<MeshConfig>,
    noise_config: Res<NoiseConfig>,
    pool: Res<ChunkPool>,
    mut spawned: ResMut<SpawnedChunks>,
) {
    if !pool.is_changed() {
        return;
    }
    let Ok(entity, ) = query.get_single() else { return;};
    let thread_pool = AsyncComputeTaskPool::get();

    let to_spawn = pool
        .0
        .iter()
        .filter(|coord| !spawned.0.contains(*coord))
        .cloned()
        .collect::<Vec<(i32, i32)>>();

    let NoiseConfig {
        seed,
        octaves,
        frequency,
        lacunarity,
        persistence,
        offset,
        falloff,
    } = *noise_config;
    for (x, y) in to_spawn {
        let mesh_config = mesh_config.clone();
        let task = thread_pool.spawn(async move {
            let mut fbm: Fbm<Perlin> = Fbm::new(seed);
            fbm.frequency = frequency;
            fbm.lacunarity = lacunarity;
            fbm.persistence = persistence;
            fbm.octaves = octaves;

            let nm = NoiseMap::new(&fbm, mesh_config.grid_size, (x, y), offset, falloff);
            ((x, y), get_landscape_data(&nm, &mesh_config))
        });
        spawned.0.insert((x, y)); // TODO make a system that removes very distant chunks.
        commands.entity(entity).with_children(|children| {
            children.spawn((ComputeMeshImageData(task),));
        });
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        Name::new("Ymir"),
        Terrain::default(),
    ));
}

fn remove_terrain(
    mut commands: Commands,
    query: Query<Entity, With<Terrain>>,
    mesh_config: Res<MeshConfig>,
    noise_config: Res<NoiseConfig>,
    mut spawned: ResMut<SpawnedChunks>,
) {
    if mesh_config.is_changed() || noise_config.is_changed() {
        for terrain in &query {
            commands.entity(terrain).despawn_descendants();
            spawned.0.clear();
        }
    }
}

#[derive(Component)]
struct ComputeMeshImageData(Task<((i32, i32), LandscapeData)>);
