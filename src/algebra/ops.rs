use super::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

// Color
impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            data: self.data + rhs.data,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.data += rhs.data
    }
}

impl Mul<Float> for Color {
    type Output = Color;

    fn mul(self, rhs: Float) -> Self::Output {
        Self::Output {
            data: self.data * rhs,
        }
    }
}

impl Mul<Color> for Float {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Self::Output {
            data: self * rhs.data,
        }
    }
}

impl MulAssign<Float> for Color {
    fn mul_assign(&mut self, rhs: Float) {
        self.data *= rhs;
    }
}

impl Div<Float> for Color {
    type Output = Color;

    fn div(self, rhs: Float) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<Float> for Color {
    fn div_assign(&mut self, rhs: Float) {
        self.data /= rhs;
    }
}
