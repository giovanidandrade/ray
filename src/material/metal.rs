use super::*;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Arc<Self> {
        Arc::new(Self { albedo })
    }

    fn reflect(vector: Vector, normal: Vector) -> Vector {
        vector - 2.0 * vector.dot(&normal) * normal
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<Scatter> {
        let scattered = Metal::reflect(ray.direction, collision.normal);

        Some(Scatter {
            scattered: Ray::new(collision.point, scattered),
            attenuation: self.albedo,
        })
    }
}
