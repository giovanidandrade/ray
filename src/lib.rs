pub mod camera;
pub mod geometry;
pub mod io;
pub mod material;
pub mod random;
pub mod scene;
pub mod threads;

// Useful Aliases
pub type Float = f32;

pub type Point = nalgebra::Vector3<Float>;
pub type Vector = nalgebra::Vector3<Float>;
pub type Color = nalgebra::Vector3<Float>;

pub type World = Vec<std::sync::Arc<dyn geometry::Geometry>>;

// Useful constants
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

// Convenience Types
#[derive(Debug, Clone, Copy)]
pub struct Dimensions(pub usize, pub usize);

/// A wrapper type for the upper left corner of the PNG Tile
#[derive(Debug, Clone, Copy, Default)]
pub struct TileCorner(pub usize, pub usize);

#[derive(Debug, Clone, Copy)]
/// Represents a closed range, i.e.: both endpoints are considered to be part of the range
pub struct Range(pub Float, pub Float);

impl Range {
    pub fn contains(&self, t: Float) -> bool {
        let Range(min, max) = *self;
        t >= min && t <= max
    }

    pub fn not_contains(&self, t: Float) -> bool {
        let Range(min, max) = *self;
        t < min || t > max
    }
}

pub fn preprocess_color(color: Color) -> [u8; 3] {
    [
        (255.999 * color.x.powf(1.0 / 2.2)).trunc() as u8,
        (255.999 * color.y.powf(1.0 / 2.2)).trunc() as u8,
        (255.999 * color.z.powf(1.0 / 2.2)).trunc() as u8,
    ]
}
