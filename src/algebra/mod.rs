use super::*;
use nalgebra::Vector3;

pub mod color;
pub mod ops;
pub mod point;
pub mod vector;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Color {
    pub data: Vector3<Float>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point {
    pub data: Vector3<Float>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector {
    pub data: Vector3<Float>,
}
