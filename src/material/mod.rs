use super::*;
use geometry::Collision;
use render::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Color,
}

pub trait Material: std::marker::Send + std::marker::Sync {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<Scatter>;
}

pub mod dielectric;
pub mod diffuse;
pub mod metal;
