use bevy::prelude::*;

use crate::terrain::{MeshConfig, Terrain};

pub struct YmirPlayerPlugin;
impl Plugin for YmirPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerPositionChangedEvent>()
            .insert_resource(LastRecordedPosition(Vec2::default()))
            .insert_resource(PlayerChunk((0, 0)))
            .add_system(broadcast_player_position)
            .add_system(register_player_chunk);
    }
}

#[derive(Component)]
pub struct YmirPlayer;
#[derive(Resource, Eq, PartialEq)]
pub struct PlayerChunk(pub (i32, i32));
#[derive(Resource)]
pub struct LastRecordedPosition(Vec2);
pub struct PlayerPositionChangedEvent(pub Vec2);

fn register_player_chunk(
    mut player_chunk: ResMut<PlayerChunk>,
    terrain: Query<&Terrain>,
    mesh_config: Res<MeshConfig>,
    mut events: EventReader<PlayerPositionChangedEvent>,
) {
    if let Err(e) = terrain.get_single() {
        bevy::log::warn!("No single Terrain entity found: {}", e);
        return;
    }

    let chunk_size = mesh_config.scale;
    for event in events.iter() {
        let mut pos = event.0;
        pos.x += chunk_size / 2.0;
        pos.y += chunk_size / 2.0;
        pos.x /= chunk_size;
        pos.y /= chunk_size;
        let coord = Vec2::new(pos.x.floor(), pos.y.floor());
        let new_chunk_candidate = PlayerChunk((coord.x as i32, -coord.y as i32));
        if *player_chunk != new_chunk_candidate {
            *player_chunk = new_chunk_candidate;
        }
    }
}

fn broadcast_player_position(
    query: Query<&GlobalTransform, With<YmirPlayer>>,
    last_recorded_position: Res<LastRecordedPosition>,
    mut events: EventWriter<PlayerPositionChangedEvent>,
) {
    let last_pos = last_recorded_position.0;
    let new_pos = match query.get_single() {
        Ok(transform) => Vec2 {
            x: transform.translation().x,
            y: transform.translation().z,
        },
        Err(_) => {
            bevy::log::warn!("No single Ymir Player found.");
            return;
        }
    };

    if new_pos.distance_squared(last_pos) > 200.0 {
        events.send(PlayerPositionChangedEvent(new_pos));
    }
}
