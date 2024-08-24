use super::*;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index_ratio: Float,
}

impl Dielectric {
    pub fn new(refraction_index: Float) -> Arc<Self> {
        Arc::new(Self {
            refraction_index_ratio: refraction_index,
        })
    }

    fn reflect(vector: Vector, normal: Vector) -> Vector {
        vector - 2.0 * vector.dot(&normal) * normal
    }

    fn refract(incoming: Vector, normal: Vector, ri_ratio: Float) -> Vector {
        let cos_theta = incoming.normalize().dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // From some angles, refraction is impossible
        if ri_ratio * sin_theta > 1.0 {
            return Dielectric::reflect(incoming, normal);
        }

        let ray_perpendicular = ri_ratio * (incoming - cos_theta * normal);
        let ray_parallel = normal * (1.0 - ray_perpendicular.norm_squared()).abs().sqrt();

        ray_perpendicular - ray_parallel
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<Scatter> {
        let mut ratio = self.refraction_index_ratio;
        if collision.is_front_facing {
            ratio = 1.0 / ratio;
        }

        let unit_direction = ray.direction.normalize();
        let refracted = Dielectric::refract(unit_direction, collision.normal.normalize(), ratio);

        Some(Scatter {
            scattered: Ray::new(collision.point, refracted.normalize()),
            attenuation: WHITE,
        })
    }
}
