use std::marker::{Send, Sync};
use std::sync::Arc;

pub mod camera;
pub mod geometry;
pub mod io;
pub mod threads;

// Useful Aliases
pub type Float = f32;

pub type Point = nalgebra::Vector3<Float>;
pub type Vector = nalgebra::Vector3<Float>;
pub type Color = nalgebra::Vector3<Float>;

pub type World = Vec<Arc<dyn geometry::Geometry + Send + Sync>>;

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
        t >= self.0 && t <= self.1
    }
}
