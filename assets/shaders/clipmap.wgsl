#import bevy_pbr::mesh_functions::{ get_model_matrix, mesh_position_local_to_clip}
#import bevy_pbr::utils::coords_to_viewport_uv;
#import bevy_pbr::mesh_view_bindings::view;

struct Vertex {
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

@vertex
fn vertex(
    vertex: Vertex
) -> VertexOutput {
    var out: VertexOutput;
    let position = vertex.position;
    let pos = vec2u(vertex.uv);
    let y = textureLoad(texture, pos, 0).r;
    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(vertex.instance),
        vec4<f32>(position.x, y, position.z,1.0)
    );
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(v: VertexOutput) -> @location(0) vec4<f32> {
    let viewport_uv = coords_to_viewport_uv(v.clip_position.xy, view.viewport);
    return textureSample(texture, texture_sampler, v.uv);
}
