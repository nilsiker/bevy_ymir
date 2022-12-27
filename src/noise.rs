use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use noise::{Fbm, NoiseFn, Perlin};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub struct NoiseMap {
    size: usize,
    values: Vec<f64>,
}

#[derive(Resource, Component, Inspectable, Clone)]
pub struct NoiseConfig {
    pub seed: u32,
    #[inspectable(min = 0, max = 6)]
    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
    pub offset: Vec2,
    pub falloff: bool,
}

// NOTE found this to be a nice default!
impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            octaves: 6,
            frequency: 0.5,
            lacunarity: 4.0,
            persistence: 0.3,
            offset: Vec2::new(0.0, 0.0),
            falloff: false,
        }
    }
}

impl NoiseMap {
    pub fn new(
        fbm: &Fbm<Perlin>,
        size: usize,
        coord: (i32, i32),
        offset: Vec2,
        use_falloff: bool,  // TODO this causes seam tearing between chunks!
    ) -> NoiseMap {
        let mut values = vec![0.0; size * size];

        values.par_iter_mut().enumerate().for_each(|(i, value)| {
            let x = i % size;
            let y = i / size;
            let chunk_offset = Vec2::new(coord.0 as f32, coord.1 as f32);
            let xf = x as f32 / (size - 1) as f32 + chunk_offset.x + offset.x;
            let yf = y as f32 / (size - 1) as f32 + chunk_offset.y + offset.y;

            *value = fbm.get([xf as f64, yf as f64]);

            if use_falloff {
                let x = i % size;
                let y = i / size;

                let x_val = x as f64 / size as f64 * 2.0 - 1.0;
                let y_val = y as f64 / size as f64 * 2.0 - 1.0;

                let falloff_val = f64::max(f64::abs(x_val), f64::abs(y_val));

                let a = 3.0;
                let b = 2.2;

                let evaluated_value =
                    falloff_val.powf(a) / (falloff_val.powf(a) + (b - b * falloff_val).powf(a));

                *value -= evaluated_value * 2.0;
            }
        });

        NoiseMap { size, values }
    }

    pub fn get_value(&self, x: usize, y: usize) -> f32 {
        self.values[y * self.size + x] as f32
    }

    pub fn size(&self) -> (usize, usize) {
        (self.size, self.size)
    }

    pub fn values(&self) -> Vec<f64> {
        self.values.clone()
    }
}
