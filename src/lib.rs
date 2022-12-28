mod components;
pub mod noise;
pub mod player;
pub mod object_placement;
pub mod terrain;

use bevy::prelude::*;
use bevy_inspector_egui::InspectorPlugin;
use player::YmirPlayerPlugin;
use terrain::YmirTerrainPlugin;

use self::{noise::NoiseConfig, object_placement::ProcSpawnPlugin, terrain::MeshConfig};

#[derive(Resource)]
struct ObjectDistance(i32);

#[derive(Default)]
pub struct YmirPlugin {
    pub chunk_distance: i32,
    pub object_distance: i32,
    pub mesh_config: MeshConfig,
    pub noise_config: NoiseConfig,
    pub inspectors: bool,
}

impl Plugin for YmirPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.mesh_config.clone())
            .insert_resource(self.noise_config.clone())
            .insert_resource(ObjectDistance(self.object_distance))
            .add_plugin(YmirTerrainPlugin {
                chunk_distance: 3,
            })
            .add_plugin(YmirPlayerPlugin)
            .add_plugin(ProcSpawnPlugin);

        if self.inspectors {
            app.add_plugin(InspectorPlugin::<MeshConfig>::new_insert_manually());
            app.add_plugin(InspectorPlugin::<NoiseConfig>::new_insert_manually());
        }
    }
}
