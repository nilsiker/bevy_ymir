use bevy::{ecs::system::EntityCommands, prelude::*, render::mesh::VertexAttributeValues};
use bevy_rapier3d::prelude::{Collider, QueryFilter, RapierContext, Real, RigidBody};
use rand::Rng;

use crate::{
    player::{PlayerChunk, YmirPlayer},
    terrain::{Chunk, MeshConfig},
};

#[derive(Resource)]
struct ObjectDistance(i32);

#[derive(Default)]
pub struct YmirSpawnpointPlugin {
    pub object_distance: i32,
}

impl Plugin for YmirSpawnpointPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObjectDistance(self.object_distance))
            .add_system(generate_spawnpoints)
            .add_system(dummy_spawn_balls);
    }
}

#[derive(Component)]
struct SpawnPointGrid {
    positions: Vec<Vec3>,
    width: usize,
}
impl SpawnPointGrid {
    fn get(&self, x: usize, y: usize) -> Option<&Vec3> {
        self.positions.get(y * self.width + x)
    }
}

fn generate_spawnpoints(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    rapier: Res<RapierContext>,
    mesh_config: Res<MeshConfig>,
    player_chunk: Res<PlayerChunk>,
    object_distance: Res<ObjectDistance>,
    query: Query<(Entity, &Handle<Mesh>, &Chunk, &Transform), Without<SpawnPointGrid>>,
) {
    // let chunks: Vec<(Entity, &Handle<Mesh>, &Chunk)> = query
    //     .into_iter()
    //     .filter(|(_, _, chunk)| {
    //         let (x, y) = player_chunk.0;

    //         (x - object_distance.0..=x + object_distance.0).contains(&chunk.x)
    //             && (y - object_distance.0..=y + object_distance.0).contains(&chunk.y)
    //     })
    //     .collect();

    let mut rand = rand::thread_rng();
    let step = mesh_config.scale / mesh_config.grid_size as f32;

    for (entity, _, chunk, _) in &query {
        let width = (chunk.unique_vertices.len() as f32).sqrt() as usize;
        let mut positions = vec![];
        for (i, pos) in chunk.unique_vertices.iter().enumerate() {
            let mut pos: Vec3 = pos.clone().into();

            if i % width == width - 1 || i >= (width * width) - width {
                continue; // ignore last row and column of vertices (these positions are shared between neighbouring chunks)
            }

            let offset_x = rand.gen_range(0.0..step);
            let offset_y = rand.gen_range(0.0..step);

            let ray_origin = Vec3::new(
                mesh_config.scale * chunk.x as f32 + pos.x + offset_x,
                1000.0,
                -mesh_config.scale * chunk.y as f32 + pos.z - offset_y,
            );
            let ray_dir = Vec3::NEG_Y;

            match rapier.cast_ray(ray_origin, ray_dir, Real::MAX, true, QueryFilter::new()) {
                Some((_, dist)) => {
                    pos.x += offset_x;
                    pos.z -= offset_y;
                    pos.y = 1000.0 - dist
                }
                None => (),
            };
            positions.push(pos);
        }

        commands
            .entity(entity)
            .insert(SpawnPointGrid { positions, width });
    }
}

fn dummy_spawn_balls(
    mut commands: Commands,
    query: Query<&Transform, With<YmirPlayer>>,
    input: Res<Input<KeyCode>>,
    rapier: Res<RapierContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for transform in &query {
            let ray_origin = transform.translation;
            let ray_direction = transform.forward();

            let point = match rapier.cast_ray(
                ray_origin,
                ray_direction,
                Real::MAX,
                true,
                QueryFilter::new(),
            ) {
                Some((entity, distance)) => ray_origin + ray_direction * distance,
                None => ray_origin,
            };

            commands.spawn(PbrBundle {
                mesh: meshes.add(
                    shape::Icosphere {
                        radius: 1.0,
                        subdivisions: 0,
                    }
                    .into(),
                ),
                material: materials.add(Color::BLACK.into()),
                transform: Transform::from_translation(point),
                ..Default::default()
            });
        }
    }
}
