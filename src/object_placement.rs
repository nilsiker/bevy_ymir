
use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use rand::Rng;

use crate::{terrain::MeshConfig, player::PlayerChunk, ObjectDistance, components::Chunk};

#[derive(Default)]
pub struct ProcSpawnPlugin;

impl Plugin for ProcSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(_dummy_spawn_points); // TODO this needs a severe revamp
    }
}

#[derive(Component)]
struct SpawnedObjects;

fn _dummy_spawn_points(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    mesh_config: Res<MeshConfig>, // handle this differently, needs some sort of heightbased generation condition!
    assets: Res<AssetServer>,
    player_chunk: Res<PlayerChunk>,
    object_distance: Res<ObjectDistance>,
    query: Query<(Entity, &Handle<Mesh>, &Chunk), (With<Chunk>, Without<SpawnedObjects>)>,
) {
    if !player_chunk.is_changed() {
        return;
    }

    let chunks: Vec<(Entity, &Handle<Mesh>, &Chunk)> = query
        .into_iter()
        .filter(|(_, _, chunk)| {
            let (x, y) = player_chunk.0;

            (x - object_distance.0..=x + object_distance.0).contains(&chunk.x)
                && (y - object_distance.0..=y + object_distance.0).contains(&chunk.y)
        })
        .collect();

    for (entity, mesh_handle, _) in chunks {
        let positions = match meshes.get(mesh_handle) {
            Some(mesh) => match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                Some(VertexAttributeValues::Float32x3(positions)) => Some(positions.clone()),
                _ => None,
            },
            None => None,
        };

        if let Some(positions) = positions {
            let mut vec = vec![];
            for pos in positions {
                if pos[1] > 0.35 * mesh_config.height_multiplier
                    || pos[1] < -0.1 * mesh_config.height_multiplier
                {
                    continue;
                }
                let v: Vec3 = pos.into();
                if !vec.contains(&v) {
                    vec.push(v);

                    let mut transform = Transform::from_translation(v);
                    transform.scale = Vec3::new(
                        rand::thread_rng().gen_range(1.4..=2.4),
                        rand::thread_rng().gen_range(1.4..=2.4),
                        rand::thread_rng().gen_range(1.4..=2.4),
                    );
                    transform.rotation = Quat::from_euler(
                        EulerRot::XYZ,
                        0.0,
                        rand::thread_rng().gen_range(0.0..=360.0),
                        0.0,
                    );
                    let pos_offset = Vec3::new(
                        rand::thread_rng().gen_range(-1.0..1.0) * 10.0,
                        0.0,
                        rand::thread_rng().gen_range(-1.0..1.0) * 10.0,
                    );
                    transform.translation += pos_offset;

                    commands.entity(entity).with_children(|children| {
                        children.spawn(SceneBundle {
                            scene: assets.load("models/tree.glb#Scene0"),
                            transform,
                            ..default()
                        });
                    });
                }
            }
        }
        commands.entity(entity).insert(SpawnedObjects);
    }
}
