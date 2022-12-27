use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::{Extent3d, PrimitiveTopology, TextureDimension, TextureFormat};
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::prelude::*;

use super::noise::NoiseMap;
use super::terrain_colors::TerrainColor;

struct MeshData {
    vertices: Vec<[f32; 3]>,
    indices: Vec<u32>,
    uvs: Vec<[f32; 2]>,
    heights: Vec<f32>,
}

#[derive(Resource, Inspectable, Clone)]
pub struct MeshConfig {
    #[inspectable(min = 2, max = 1025)]
    pub grid_size: usize,
    pub scale: f32,
    pub height_multiplier: f32,
    pub render_mode: RenderMode,
    pub texture_mode: TextureMode,
    pub flat_shading: bool,
    pub color_config: ColorConfig,
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            grid_size: 33,
            scale: 256.0,
            height_multiplier: 80.0,
            render_mode: default(),
            texture_mode: default(),
            flat_shading: true,
            color_config: default(),
        }
    }
}

#[derive(Inspectable, Default, Clone)]
pub struct ColorRange {
    pub color: Color,
    pub start_height: f32,
}

#[derive(Inspectable, Clone)]
pub struct ColorConfig {
    pub colors: Vec<ColorRange>,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            colors: vec![
                ColorRange {
                    color: TerrainColor::SNOW,
                    start_height: 0.99,
                },
                ColorRange {
                    color: TerrainColor::MOUNTAIN,
                    start_height: 0.7,
                },
                ColorRange {
                    color: TerrainColor::GRASS,
                    start_height: 0.3,
                },
                ColorRange {
                    color: TerrainColor::SAND,
                    start_height: 0.25,
                },
                ColorRange {
                    color: TerrainColor::SHALLOW_WATER,
                    start_height: 0.2,
                },
                ColorRange {
                    color: TerrainColor::DEEP_WATER,
                    start_height: -1.0,
                },
            ],
        }
    }
}

pub struct MeshImageData {
    pub mesh: Mesh,
    pub image: Image,
    pub collider: Option<Collider>,
}

#[derive(Default, Clone, Copy, Inspectable)]
pub enum RenderMode {
    #[default]
    Mesh,
    Plane,
}
#[derive(Default, Clone, Copy, Inspectable)]
pub enum TextureMode {
    #[default]
    Color,
    HeightMap(Color),
}

pub fn get_mesh(map: &NoiseMap, mesh_config: &MeshConfig) -> MeshImageData {
    match mesh_config.render_mode {
        RenderMode::Plane => generate_plane(map, mesh_config.scale, mesh_config),
        RenderMode::Mesh => generate_mesh(map, mesh_config),
    }
}
fn generate_plane(map: &NoiseMap, scale: f32, mesh_config: &MeshConfig) -> MeshImageData {
    let size = map.size().0 as u32;

    let data = match mesh_config.texture_mode {
        TextureMode::Color => to_color_vec(map, &mesh_config.color_config),
        TextureMode::HeightMap(color) => to_heightmap_vec(map, color),
    };

    let mesh = Mesh::from(shape::Plane { size: scale });

    let image = Image::new_fill(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data.as_slice(),
        TextureFormat::Rgba8UnormSrgb,
    );
    let collider = None;

    MeshImageData {
        mesh,
        image,
        collider,
    }
}

fn generate_mesh(map: &NoiseMap, mesh_config: &MeshConfig) -> MeshImageData {
    let size = map.size().0 as u32;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mesh_data = generate_mesh_data(map, mesh_config);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_data.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_data.uvs);
    mesh.set_indices(Some(Indices::U32(mesh_data.indices)));

    if mesh_config.flat_shading {
        mesh.duplicate_vertices();
        mesh.compute_flat_normals();
    }

    let texture_data = match mesh_config.texture_mode {
        TextureMode::Color => to_color_vec(map, &mesh_config.color_config),
        TextureMode::HeightMap(color) => to_heightmap_vec(map, color),
    };

    let image = Image::new_fill(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        texture_data.as_slice(),
        TextureFormat::Rgba8UnormSrgb,
    );

    let collider = Collider::heightfield(
        mesh_data.heights,
        size as usize,
        size as usize,
        Vec3::ONE * mesh_config.scale,
    );

    MeshImageData {
        mesh,
        image,
        collider: Some(collider),
    }
}

fn generate_mesh_data(map: &NoiseMap, mesh_config: &MeshConfig) -> MeshData {
    let (width, height) = map.size();

    let scale = mesh_config.scale;

    let top_left_x = (width - 1) as f32 / -2.0;
    let top_left_z = (height - 1) as f32 / 2.0;

    let mut heights = vec![0.0; height * width];
    let mut vertices = vec![[0.0; 3]; height * width];

    let mut indices = vec![0; (height - 1) * (width - 1) * 6];

    let mut uvs = vec![[0.0; 2]; height * width];

    let mut vertex_index = 0;
    let mut triangle_index = 0;

    let mut add_triangle = |a: usize, b: usize, c: usize| {
        indices[triangle_index] = a as u32;
        indices[triangle_index + 1] = b as u32;
        indices[triangle_index + 2] = c as u32;
        triangle_index += 3;
    };

    for y in 0..height {
        for x in 0..width {
            let xf = x as f32;
            let zf = y as f32;
            let height_value = map.get_value(x, y) * mesh_config.height_multiplier;
            heights[vertex_index] = map.get_value(x, y) / mesh_config.scale * mesh_config.height_multiplier;
            vertices[vertex_index] = [
                (top_left_x + xf) / (width - 1) as f32 * scale,
                height_value,
                (top_left_z - zf) / (height - 1) as f32 * scale,
            ]; 
            uvs[vertex_index] = [
                x as f32 / (width - 1) as f32,
                y as f32 / (height - 1) as f32,
            ];

            if x < width - 1 && y < height - 1 {
                add_triangle(vertex_index, vertex_index + width + 1, vertex_index + width);
                add_triangle(vertex_index + width + 1, vertex_index, vertex_index + 1);
            }

            vertex_index += 1;
        }
    }

    MeshData {
        vertices,
        indices,
        uvs,
        heights, // used for heightfield collision!
                 // TODO add normals when flat shading won't cut it no more!
    }
}

fn to_heightmap_vec(map: &NoiseMap, base_color: Color) -> Vec<u8> {
    let size = map.size().0;
    let mut data: Vec<u8> = Vec::with_capacity(size * size);

    for i in map.values() {
        let i_normalized = (i * 0.5 + 0.5).clamp(0.0, 1.0);
        let i_u8 = (i_normalized * 255.0) as u8;
        data.push(((i_u8 as f32) * base_color.r()) as u8); //r
        data.push(((i_u8 as f32) * base_color.g()) as u8); //g
        data.push(((i_u8 as f32) * base_color.b()) as u8); //b
        data.push(255); //a
    }

    data
}

fn to_color_vec(map: &NoiseMap, config: &ColorConfig) -> Vec<u8> {
    let size = map.size().0;
    let mut data: Vec<u8> = Vec::with_capacity(size * size);

    for value in map.values() {
        let value = (value * 0.5 + 0.5).clamp(0.0, 1.0);

        let mut colors = config.colors.clone();
        colors.sort_by(|a, b| b.start_height.total_cmp(&a.start_height));

        let color_range = colors
            .iter()
            .find(|color| value > color.start_height as f64);

        let color = match color_range {
            Some(color_range) => color_range.color,
            None => Color::rgb_u8(255, 0, 255),
        };

        data.push((color.r() * 255.0) as u8); //r
        data.push((color.g() * 255.0) as u8); //g
        data.push((color.b() * 255.0) as u8); // b
        data.push(255); //a
    }

    data
}
