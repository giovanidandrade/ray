use super::*;

pub const PI: Float = std::f32::consts::PI;

#[inline]
pub fn schlick(r0: Float, cos_theta: Float) -> Float {
    r0 * (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

#[inline]
pub fn degrees_to_radians(theta: Float) -> Float {
    theta * PI / 180.0
}
