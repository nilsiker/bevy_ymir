use bevy::prelude::*;
use bevy_ymir::{clipmap::ClipmapMaterial, YmirPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(YmirPlugin)
        .add_plugins(MaterialPlugin::<ClipmapMaterial>::default())
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ClipmapMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-4., 2., 4.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(Plane3d {
            normal: Direction3d::Y,
        })),
        transform: Transform::IDENTITY,
        material: materials.add(ClipmapMaterial { color: Color::RED }),
        ..default()
    });
}
