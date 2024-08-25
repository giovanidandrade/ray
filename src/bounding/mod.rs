use std::cmp::Ordering;

use super::*;
use camera::Ray;

pub mod hierarchy;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    x: Range,
    y: Range,
    z: Range,
}

impl BoundingBox {
    pub fn new(x: Range, y: Range, z: Range) -> Self {
        Self { x, y, z }
    }

    /// Builds a box given two points that represent opposite extrema, i.e.: separated by the diagonal
    /// of the box
    pub fn from_extrema(p: Point, q: Point) -> Self {
        Self {
            x: Range(p.x.min(q.x), p.x.max(q.x)),
            y: Range(p.y.min(q.y), p.y.max(q.y)),
            z: Range(p.z.min(q.z), p.z.max(q.z)),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            x: self.x.union(other.x),
            y: self.y.union(other.y),
            z: self.z.union(other.z),
        }
    }

    fn axes(&self) -> [Range; 3] {
        [self.x, self.y, self.z]
    }

    /// Compares two boxes on a given axis. Will panic if given an axis bigger than 2
    pub fn compare_axis(&self, other: &Self, axis: usize) -> Ordering {
        let (axis_a, axis_b) = match axis {
            0 => (self.x, other.x),
            1 => (self.y, other.y),
            2 => (self.z, other.z),
            _ => panic!("Axis given is not 0 (x), 1 (y) or 2 (z)"),
        };

        axis_a.0.total_cmp(&axis_b.0)
    }

    /// Given a ray, checks whether it intersects the box, returning the range of t values
    /// at which the ray intersects
    pub fn check_intersection(&self, ray: &Ray, range: Range) -> Option<Range> {
        let mut range = range;

        for (index, axis) in self.axes().iter().enumerate() {
            let Range(min, max) = axis;
            let coefficient = 1.0 / ray.direction[index];

            let mut t0 = (min - ray.origin[index]) * coefficient;
            let mut t1 = (max - ray.origin[index]) * coefficient;
            if t1 < t0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            match range.intersect(Range(t0, t1)) {
                Some(r) => range = r,
                None => return None,
            }
        }

        Some(range)
    }
}
