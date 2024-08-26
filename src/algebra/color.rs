use std::ops::{Deref, DerefMut};

use super::*;

/// A type made to allow Colors to be accessed via rgb accessors while still leveraging
/// nalgebra for quick and optimized operations
#[repr(C)]
#[derive(PartialEq, Clone, Debug, Copy)]
pub struct RGB {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl Deref for Color {
    type Target = RGB;

    fn deref(&self) -> &Self::Target {
        // Safety: this is OK because it's essentially how nalgebra does it and it's safe there.
        // The supposed safety coming from the fact Vector3 is guaranteed to be allocated contiguously and
        // the wrapper type has C representation
        unsafe { &*(self.data.as_ptr() as *const Self::Target) }
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: this is OK because it's essentially how nalgebra does it and it's safe there.
        // The supposed safety coming from the fact Vector3 is guaranteed to be allocated contiguously and
        // the wrapper type has C representation
        unsafe { &mut *(self.data.as_mut_ptr() as *mut Self::Target) }
    }
}

impl Color {
    #[inline]
    pub const fn new(r: Float, g: Float, b: Float) -> Self {
        Self {
            data: Vector3::new(r, g, b),
        }
    }

    #[inline]
    pub fn component_mul(&self, other: &Self) -> Self {
        Self {
            data: self.data.component_mul(&other.data),
        }
    }

    #[inline]
    pub fn lerp(&self, other: &Self, t: Float) -> Self {
        Self {
            data: self.data.lerp(&other.data, t),
        }
    }

    /// Converts the color from [0, 1] colorspace to 2.2 gamma corrected 8 bit RGB
    pub fn to_gamma_corrected_bytes(&self) -> [u8; 3] {
        let buffer = self.data;

        [
            (255.999 * buffer.x.powf(1.0 / 2.2)).trunc() as u8,
            (255.999 * buffer.y.powf(1.0 / 2.2)).trunc() as u8,
            (255.999 * buffer.z.powf(1.0 / 2.2)).trunc() as u8,
        ]
    }
}
