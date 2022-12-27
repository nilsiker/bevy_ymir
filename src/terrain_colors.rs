use bevy::prelude::Color;

pub struct TerrainColor;
impl TerrainColor {
    pub const SNOW: Color = Color::WHITE;
    pub const MOUNTAIN: Color = Color::GRAY;
    pub const GRASS: Color = Color::Rgba {
        red: 49.0 / 255.0,
        green: 108.0 / 255.0,
        blue: 49.0 / 255.0,
        alpha: 1.0,
    };
    pub const SAND: Color = Color::Rgba {
        red: 143.0 / 255.0,
        green: 143.0 / 255.0,
        blue: 102.0 / 255.0,
        alpha: 1.0,
    };
    pub const SHALLOW_WATER: Color = Color::Rgba {
        red: 77.0 / 255.0,
        green: 90.0 / 255.0,
        blue: 145.0 / 255.0,
        alpha: 1.0,
    };
    pub const DEEP_WATER: Color = Color::MIDNIGHT_BLUE;
}
