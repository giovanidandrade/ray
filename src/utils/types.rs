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
    /// Returns a reversed range that operates as the empty operand of union
    pub fn union_empty() -> Self {
        Range(Float::INFINITY, -Float::INFINITY)
    }

    // Returns the intersecting range if there is one, and None if the ranges are disjoint
    pub fn intersect(&self, other: Range) -> Option<Self> {
        let a = self.0.max(other.0);
        let b = self.1.min(other.1);

        if b < a {
            None
        } else {
            Some(Self(a, b))
        }
    }

    pub fn length(&self) -> Float {
        self.1 - self.0
    }

    pub fn union(&self, other: Range) -> Self {
        let a = self.0.min(other.0);
        let b = self.1.max(other.1);

        Self(a, b)
    }

    pub fn contains(&self, t: Float) -> bool {
        let Range(min, max) = *self;
        t >= min && t <= max
    }

    pub fn not_contains(&self, t: Float) -> bool {
        let Range(min, max) = *self;
        t < min || t > max
    }
}
