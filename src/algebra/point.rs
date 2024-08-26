use super::*;
use std::ops::{Deref, DerefMut};

/// A type made to allow Vectors to be accessed via xyz accessors while still leveraging
/// nalgebra for quick and optimized operations
#[repr(C)]
#[derive(PartialEq, Clone, Debug, Copy)]
pub struct XYZ {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Deref for Point {
    type Target = XYZ;

    fn deref(&self) -> &Self::Target {
        // Safety: this is OK because it's essentially how nalgebra does it and it's safe there.
        // The supposed safety coming from the fact Vector3 is guaranteed to be allocated contiguously and
        // the wrapper type has C representation
        unsafe { &*(self.data.as_ptr() as *const Self::Target) }
    }
}

impl DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: this is OK because it's essentially how nalgebra does it and it's safe there.
        // The supposed safety coming from the fact Vector3 is guaranteed to be allocated contiguously and
        // the wrapper type has C representation
        unsafe { &mut *(self.data.as_mut_ptr() as *mut Self::Target) }
    }
}

impl Point {
    #[inline]
    pub const fn new(x: Float, y: Float, z: Float) -> Self {
        Self {
            data: Vector3::new(x, y, z),
        }
    }

    #[inline]
    pub fn zeros() -> Self {
        Self {
            data: Vector3::zeros(),
        }
    }

    #[inline]
    pub fn x() -> Self {
        Self { data: Vector3::x() }
    }

    #[inline]
    pub fn y() -> Self {
        Self { data: Vector3::y() }
    }

    #[inline]
    pub fn z() -> Self {
        Self { data: Vector3::z() }
    }
}
