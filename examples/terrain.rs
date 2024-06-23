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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ClipmapMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-1.5, 0.5, 1.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 2.0,
            subdivisions: 256,
        })),
        transform: Transform::IDENTITY,
        material: materials.add(ClipmapMaterial {
            height_map: asset_server.load("maps/iceland_height.png"),
        }),
        ..default()
    });
}
