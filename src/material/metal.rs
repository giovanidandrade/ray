use super::*;
use std::sync::Arc;

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
