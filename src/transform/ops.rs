use super::*;
use std::ops::{Div, Mul};

impl Mul<Point> for Transform {
    type Output = Point;

    #[inline]
    fn mul(self, rhs: Point) -> Self::Output {
        Self::Output {
            data: (self.matrix * rhs.unified()).xyz(),
        }
    }
}

impl Mul<Vector> for Transform {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output {
            data: (self.matrix * rhs.unified()).xyz(),
        }
    }
}

impl Mul<Ray> for Transform {
    type Output = Ray;

    #[inline]
    fn mul(self, rhs: Ray) -> Self::Output {
        Ray::new(self * rhs.origin, self * rhs.direction)
    }
}

impl Mul for Transform {
    type Output = Transform;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            matrix: self.matrix * rhs.matrix,
            inverse: rhs.inverse * self.inverse,
        }
    }
}

// Multiplication is allowed here because we cache the inverse
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Point> for Transform {
    type Output = Point;

    #[inline]
    fn div(self, rhs: Point) -> Self::Output {
        Self::Output {
            data: (self.inverse * rhs.unified()).xyz(),
        }
    }
}

// Multiplication is allowed here because we cache the inverse
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Vector> for Transform {
    type Output = Vector;

    #[inline]
    fn div(self, rhs: Vector) -> Self::Output {
        Self::Output {
            data: (self.inverse * rhs.unified()).xyz(),
        }
    }
}

impl Div<Ray> for Transform {
    type Output = Ray;

    #[inline]
    fn div(self, rhs: Ray) -> Self::Output {
        Ray::new(self / rhs.origin, self / rhs.direction)
    }
}

// Multiplication is allowed here because we cache the inverse
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Transform {
    type Output = Transform;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            matrix: self.inverse * rhs.matrix,
            inverse: rhs.inverse * self.matrix,
        }
    }
}
