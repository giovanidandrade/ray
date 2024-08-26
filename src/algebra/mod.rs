use super::*;
use nalgebra::Vector3;

pub mod color;
pub mod ops;
pub mod point;
pub mod vector;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Color {
    data: Vector3<Float>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point {
    data: Vector3<Float>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector {
    data: Vector3<Float>,
}
