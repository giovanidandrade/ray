pub mod bounding;
pub mod camera;
pub mod geometry;
pub mod io;
pub mod material;
pub mod scene;
pub mod utils;

// Reexporting some useful and common things
pub use utils::types::*;
pub use utils::*;

// Useful Aliases
pub type Float = f32;

pub type Point = nalgebra::Vector3<Float>;
pub type Vector = nalgebra::Vector3<Float>;
pub type Color = nalgebra::Vector3<Float>;

pub type WorldObject = std::sync::Arc<dyn geometry::Geometry>;
pub type World = Vec<WorldObject>;

// Useful constants
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

pub fn preprocess_color(color: Color) -> [u8; 3] {
    [
        (255.999 * color.x.powf(1.0 / 2.2)).trunc() as u8,
        (255.999 * color.y.powf(1.0 / 2.2)).trunc() as u8,
        (255.999 * color.z.powf(1.0 / 2.2)).trunc() as u8,
    ]
}
