use super::*;
use math::ZERO_TOL;
use std::sync::Arc;

/// Idealization of matte surfaces: always scatters light randomly when hit
#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Arc<Self> {
        Arc::new(Self { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, collision: &Collision) -> Option<Scatter> {
        let mut scatter_direction = collision.normal + random::random_unit_vector();
        if scatter_direction.norm_squared() < ZERO_TOL {
            scatter_direction = collision.normal;
        }

        Some(Scatter {
            scattered: Ray::new(collision.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

/// Behaves more like glossy surfaces, will reflect light rays at glancing angles.
#[derive(Debug, Clone, Copy)]
pub struct Glossy {
    albedo: Color,
}

impl Glossy {
    pub fn new(albedo: Color) -> Arc<Self> {
        Arc::new(Self { albedo })
    }
}

impl Material for Glossy {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<Scatter> {
        let reflection_probability = 1.0 - ray.direction.normalize().dot(&collision.normal).abs();

        let mut scatter_direction = if random::random_float() < reflection_probability {
            ray.direction.reflect(collision.normal)
        } else {
            collision.normal + random::random_unit_vector()
        };

        if scatter_direction.norm_squared() < ZERO_TOL {
            scatter_direction = collision.normal;
        }

        Some(Scatter {
            scattered: Ray::new(collision.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
