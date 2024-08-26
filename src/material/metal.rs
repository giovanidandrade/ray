use super::*;
use std::sync::Arc;

/// Represents a simplified metal model as a mirror with some level of fuzziness
#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzziness: Float,
}

impl Metal {
    /// Will error out if fuzziness is smaller than zero
    pub fn new(albedo: Color, fuzziness: Float) -> Arc<Self> {
        assert! { fuzziness >= 0.0 }
        Arc::new(Self { albedo, fuzziness })
    }

    pub fn polished(albedo: Color) -> Arc<Self> {
        Arc::new(Self {
            albedo,
            fuzziness: 0.0,
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<Scatter> {
        let scattered =
            ray.direction.reflect(collision.normal) + self.fuzziness * random::random_unit_vector();

        Some(Scatter {
            scattered: Ray::new(collision.point, scattered),
            attenuation: self.albedo,
        })
    }
}

/// Represents a metal model where the color changes as reflectance increases
#[derive(Debug, Clone, Copy)]
pub struct SpecularMetal {
    albedo: Color,
    reflective_albedo: Color,
    normal_reflectance: Float,
    fuzziness: Float,
}

impl SpecularMetal {
    /// Will error out if fuzziness is smaller than zero, or if normal_reflectance isn't in [0, 1.0]
    pub fn new(
        albedo: Color,
        reflective_albedo: Color,
        normal_reflectance: Float,
        fuzziness: Float,
    ) -> Arc<Self> {
        assert! { fuzziness >= 0.0 }
        assert! { Range(0.0, 1.0).contains(normal_reflectance) }

        Arc::new(Self {
            albedo,
            reflective_albedo,
            normal_reflectance,
            fuzziness,
        })
    }

    pub fn polished(
        albedo: Color,
        reflective_albedo: Color,
        normal_reflectance: Float,
    ) -> Arc<Self> {
        Arc::new(Self {
            albedo,
            reflective_albedo,
            normal_reflectance,
            fuzziness: 0.0,
        })
    }
}

impl Material for SpecularMetal {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<Scatter> {
        let scattered =
            ray.direction.reflect(collision.normal) + self.fuzziness * random::random_unit_vector();

        let cos_theta = ray.direction.normalize().dot(&collision.normal);
        let reflectance = math::schlick(self.normal_reflectance, cos_theta);

        let attenuation = if random::random_float() < reflectance {
            self.albedo
        } else {
            self.reflective_albedo
        };

        Some(Scatter {
            scattered: Ray::new(collision.point, scattered),
            attenuation,
        })
    }
}
