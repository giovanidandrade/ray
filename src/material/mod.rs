use super::*;
use camera::Ray;
use geometry::Collision;

#[derive(Debug, Clone, Copy)]
pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Color,
}

pub trait Material: std::marker::Send + std::marker::Sync {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<Scatter>;
}

pub mod lambertian;
pub mod metal;
