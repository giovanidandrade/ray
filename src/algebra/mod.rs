use super::*;
use nalgebra::Vector3;

pub mod color;
pub mod ops;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    data: Vector3<Float>,
}
