use super::*;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Arc<Self> {
        Arc::new(Self { albedo })
    }
}

const NEAR_ZERO_TOLERANCE: Float = 1e-8;

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, collision: &Collision) -> Option<Scatter> {
        let scatter_direction = collision.normal + random::random_vector_on_unit_sphere();
        if scatter_direction.norm_squared() < NEAR_ZERO_TOLERANCE {
            return None;
        }

        Some(Scatter {
            scattered: Ray::new(collision.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

#[derive(Debug, Clone, Copy)]
/// A Lambertian model of diffuse materials that scatters probabilistically
pub struct LambertianProb {
    albedo: Color,
    probability: Float,
}

impl LambertianProb {
    /// Will error out if the probability is smaller than 0 or bigger than 100%
    pub fn new(albedo: Color, probability: Float) -> Arc<Self> {
        assert! { Range(0.0, 1.0).contains(probability) }
        Arc::new(Self {
            albedo,
            probability,
        })
    }
}

impl Material for LambertianProb {
    fn scatter(&self, _ray: &Ray, collision: &Collision) -> Option<Scatter> {
        use rand::Rng;

        let sample: Float = rand::thread_rng().gen();
        if sample < self.probability {
            return None;
        }

        let scatter_direction = collision.normal + random::random_vector_on_unit_sphere();
        if scatter_direction.norm_squared() < NEAR_ZERO_TOLERANCE {
            return None;
        }

        Some(Scatter {
            scattered: Ray::new(collision.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LambertianApproximation {
    albedo: Color,
}

impl LambertianApproximation {
    pub fn new(albedo: Color) -> Arc<Self> {
        Arc::new(Self { albedo })
    }
}

impl Material for LambertianApproximation {
    fn scatter(&self, _ray: &Ray, collision: &Collision) -> Option<Scatter> {
        let scatter_direction = collision.normal + random::random_on_hemisphere(&collision.normal);

        Some(Scatter {
            scattered: Ray::new(collision.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}