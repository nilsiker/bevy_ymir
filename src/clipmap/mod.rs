use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ClipmapMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub height_map: Handle<Image>,
    // #[texture(2)]
    // #[sampler(3)]
    // normal_map: Option<Handle<Image>>,
}

impl Material for ClipmapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/clipmap.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/clipmap.wgsl".into()
    }
}
