use bevy::prelude::*;
use bevy_ymir::{clipmap::ClipmapMaterial, YmirPlugin};

#[derive(Component)]
struct Terrain;

#[derive(Component)]
struct Spectator;

#[derive(Component)]
struct Speed(pub f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(YmirPlugin)
        .add_plugins(MaterialPlugin::<ClipmapMaterial>::default())
        .add_systems(Startup, (setup_clipmap, setup_spectator))
        .add_systems(Update, (change_amplitude, move_spectator, move_clipmap))
        .run();
}

fn setup_clipmap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ClipmapMaterial>>,
) {
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 10.0,
                subdivisions: 2048,
            })),
            transform: Transform::IDENTITY,
            material: materials.add(ClipmapMaterial {
                height_map: asset_server.load("maps/iceland_height.png"),
                normal_map: asset_server.load("maps/iceland_normal.png"),
                amplitude: 0.2,
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

fn move_spectator(
    mut query: Query<(&mut Transform, &Speed), With<Spectator>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0f32, 0f32, -0.2);
    }
    if input.pressed(KeyCode::KeyS) {
        direction += Vec3::new(0f32, 0f32, 0.2);
    }
    if input.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-0.2, 0., 0.);
    }
    if input.pressed(KeyCode::KeyD) {
        direction += Vec3::new(0.2, 0., 0.);
    }
    if direction.length_squared() > f32::EPSILON {
        query.iter_mut().for_each(|(mut t, s)| {
            t.translation += direction.normalize() * time.delta_seconds() * s.0
        });
    }
}

fn move_clipmap(
    spectator_q: Query<&Transform, With<Spectator>>,
    mut clipmap_q: Query<&mut Transform, (With<Terrain>, Without<Spectator>)>,
) {
    let Ok(spectator) = spectator_q.get_single() else {
        return;
    };

    clipmap_q.iter_mut().for_each(|mut t| {
        t.translation.x = spectator.translation.x;
        t.translation.z = spectator.translation.z;
    });
}

fn setup_spectator(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 1.5),
            ..default()
        },
        Spectator,
        Speed(0.2),
    ));
}
