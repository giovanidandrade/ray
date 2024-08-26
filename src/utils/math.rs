use super::*;

pub fn schlick(r0: Float, cos_theta: Float) -> Float {
    r0 * (1.0 - r0) * (1.0 - cos_theta).powi(5)
}
