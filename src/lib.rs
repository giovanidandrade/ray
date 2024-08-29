pub mod algebra;
pub mod bounding;
pub mod geometry;
pub mod io;
pub mod material;
pub mod render;
pub mod scene;
pub mod transform;
pub mod utils;

// Reexporting some useful and common things
pub use algebra::*;
pub use utils::types::*;
pub use utils::*;

// Useful Aliases
pub type Float = f32;

pub type WorldObject = std::sync::Arc<dyn geometry::Geometry>;
pub type World = Vec<WorldObject>;
