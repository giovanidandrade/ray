use super::*;

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
