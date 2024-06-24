#import bevy_pbr::mesh_functions::{ get_model_matrix, mesh_position_local_to_clip, mesh_position_local_to_world}
#import bevy_pbr::mesh_view_bindings;

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

@group(2) @binding(0) var heightmap: texture_2d<f32>;
@group(2) @binding(1) var heightmap_sampler: sampler;
@group(2) @binding(2) var normal_map: texture_2d<f32>;
@group(2) @binding(3) var normal_map_sampler: sampler;
@group(2) @binding(4) var<uniform> amplitude: f32;

@vertex
fn vertex(
    vertex: Vertex
) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = get_model_matrix(vertex.instance);
    let world_position = mesh_position_local_to_world(model_matrix, vec4<f32>(vertex.position, 1.0)).xyz;

    let heightmap_coords = vec2u(fract(world_position.xz) * vec2<f32>(textureDimensions(heightmap, 0)));
    let y = textureLoad(heightmap, heightmap_coords, 0).y * amplitude;
    let deformed_position = vec3<f32>(vertex.position.x, y, vertex.position.z);

    out.clip_position = mesh_position_local_to_clip(
        model_matrix,
        vec4<f32>(deformed_position, 1.0)
    );

    out.uv = fract(world_position.xz);

    return out;
}

@fragment
fn fragment(v: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the color texture
    let color = textureSample(heightmap, heightmap_sampler, v.uv);

    // Sample the normal map
    let normal_sample = textureSample(normal_map, normal_map_sampler, v.uv);
    let normal = normalize(normal_sample.xyz * 2.0 - 1.0);

    // Use standard material lighting
    let view_light_dir = normalize(vec3<f32>(1.0, -1.0, 0.0)); // Example light direction
    let diffuse = max(dot(normal, view_light_dir), 0.0);
        
    return vec4<f32>(color.rgb * diffuse, color.a);
}