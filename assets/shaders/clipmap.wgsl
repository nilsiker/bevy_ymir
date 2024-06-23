#import bevy_pbr::mesh_functions::{ get_model_matrix, mesh_position_local_to_clip, mesh_position_local_to_world}
#import bevy_pbr::utils::coords_to_viewport_uv;
#import bevy_pbr::mesh_view_bindings::view;

struct Vertex {
    @builtin(vertex_index) v_index: u32,
    @builtin(instance_index) instance: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>
};

@group(2) @binding(0) var texture: texture_2d<f32>;
@group(2) @binding(1) var texture_sampler: sampler;
@group(2) @binding(2) var<uniform> amplitude: f32;

@vertex
fn vertex(
    vertex: Vertex
) -> VertexOutput {
    var out: VertexOutput;

    let position = vertex.position;
    var pos = vec2u(vertex.uv* vec2<f32>(textureDimensions(texture, 0)));

    let y = textureLoad(texture, pos, 0).y * amplitude;
    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(vertex.instance),
        vec4<f32>(position.x, y, position.z, 1.0)
    );

    out.uv = vertex.uv;

    return out;
}

@fragment
fn fragment(v: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, v.uv);
}
