#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}

struct Vertex {
    @builtin(instance_index) instance: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vertex(
    vertex: Vertex
) -> VertexOutput {
    var out: VertexOutput;
    let position = vertex.position;
    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(vertex.instance),
        vec4<f32>(position,1.0)
    );
    return out;
}

@fragment
fn fragment(v: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(v.clip_position.x,v.clip_position.y,v.clip_position.z,1);
}
