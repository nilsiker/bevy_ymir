use bevy::prelude::*;
use bevy_ymir::{clipmap::ClipmapMaterial, YmirPlugin};

#[derive(Component)]
struct Terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(YmirPlugin)
        .add_plugins(MaterialPlugin::<ClipmapMaterial>::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (change_amplitude, move_plane))
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

    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 2.0,
                subdivisions: 2048,
            })),
            transform: Transform::IDENTITY,
            material: materials.add(ClipmapMaterial {
                height_map: asset_server.load("maps/iceland_height.png"),
                amplitude: 0.01,
            }),
            ..default()
        },
        Terrain,
    ));
}

fn change_amplitude(
    mut materials: ResMut<Assets<ClipmapMaterial>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::ArrowUp) {
        materials.iter_mut().for_each(|(_, m)| m.amplitude += 0.1);
    } else if input.just_pressed(KeyCode::ArrowDown) {
        materials.iter_mut().for_each(|(_, m)| m.amplitude -= 0.1);
    }
}

fn move_plane(mut query: Query<&mut Transform, With<Terrain>>, input: Res<ButtonInput<KeyCode>>) {
    let direction: Vec3 = if input.just_pressed(KeyCode::KeyW) {
        Vec3::new(0f32, 0f32, 0.2)
    } else if input.just_pressed(KeyCode::KeyS) {
        Vec3::new(0f32, 0f32, -0.2)
    } else {
        Vec3::ZERO
    };
    query
        .iter_mut()
        .for_each(|mut t| t.translation += direction);
}
