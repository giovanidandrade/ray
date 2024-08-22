pub mod io;

#[derive(Debug, Clone, Copy)]
pub struct Dimensions(pub usize, pub usize);

// Useful Alias
pub type Float = f32;

pub type Point = nalgebra::Vector3<Float>;
pub type Vector = nalgebra::Vector3<Float>;
pub type Color = nalgebra::Vector3<Float>;
