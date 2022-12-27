use bevy::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorParams, WorldInspectorPlugin};
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_ymir::{PlayerPositionChangedEvent, TerragenPlugin};
use rustpg::{
    core::{camera::CameraPlugin, spectator::SpectatorPlugin},
    nycthemeron::{time_of_day::TimeOfDay, NycthemeronPlugin},
};

fn main() {
    let inspectors = match std::env::args().collect::<Vec<String>>().get(1) {
        Some(inspectors) => inspectors == "true",
        None => false,
    };

    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb_u8(85, 156, 215)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(SpectatorPlugin)
        .add_plugin(NycthemeronPlugin {
            time_of_day: TimeOfDay::new(12.0, 0.0, 0.0, 0.0),
            inspectors:false,
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(TerragenPlugin {
            chunk_distance: 5,
            object_distance: 1,
            inspectors,
            ..default()
        })
        .add_system(send_player_pos_events);

    if inspectors {
        app.add_plugin(InspectorPlugin::<AmbientLight>::default())
            .add_plugin(WorldInspectorPlugin::default())
            .insert_resource(WorldInspectorParams {
                sort_components: true,
                despawnable_entities: true,
                ..default()
            });
    }

    app.run();
}

fn send_player_pos_events(
    query: Query<&Transform, With<Camera>>,
    mut events: EventWriter<PlayerPositionChangedEvent>,
) {
    if let Ok(transform) = query.get_single() {
        events.send(PlayerPositionChangedEvent(transform.translation));
    }
}
