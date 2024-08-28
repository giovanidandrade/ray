use super::*;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

// Color operations
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

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            data: self.data - rhs.data,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.data -= rhs.data;
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

impl Index<usize> for Color {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl From<Point> for Color {
    #[inline]
    fn from(value: Point) -> Self {
        Self { data: value.data }
    }
}

impl From<Vector> for Color {
    #[inline]
    fn from(value: Vector) -> Self {
        Self { data: value.data }
    }
}

// Point operations
impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            data: self.data + rhs.data,
        }
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.data += rhs.data
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output {
            data: self.data - rhs.data,
        }
    }
}

impl SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        self.data -= rhs.data;
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            data: self.data - rhs.data,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output { data: -self.data }
    }
}

impl Mul<Float> for Point {
    type Output = Point;

    fn mul(self, rhs: Float) -> Self::Output {
        Self::Output {
            data: self.data * rhs,
        }
    }
}

impl Mul<Point> for Float {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Self::Output {
            data: self * rhs.data,
        }
    }
}

impl MulAssign<Float> for Point {
    fn mul_assign(&mut self, rhs: Float) {
        self.data *= rhs;
    }
}

impl Div<Float> for Point {
    type Output = Point;

    fn div(self, rhs: Float) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<Float> for Point {
    fn div_assign(&mut self, rhs: Float) {
        self.data /= rhs;
    }
}

impl Index<usize> for Point {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl From<Color> for Point {
    #[inline]
    fn from(value: Color) -> Self {
        Self { data: value.data }
    }
}

impl From<Vector> for Point {
    #[inline]
    fn from(value: Vector) -> Self {
        Self { data: value.data }
    }
}

// Vector operations
impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            data: self.data + rhs.data,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.data += rhs.data
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output { data: -self.data }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output {
            data: self.data - rhs.data,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.data -= rhs.data;
    }
}

impl Mul<Float> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Float) -> Self::Output {
        Self::Output {
            data: self.data * rhs,
        }
    }
}

impl Mul<Vector> for Float {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output {
            data: self * rhs.data,
        }
    }
}

impl MulAssign<Float> for Vector {
    fn mul_assign(&mut self, rhs: Float) {
        self.data *= rhs;
    }
}

impl Div<Float> for Vector {
    type Output = Vector;

    fn div(self, rhs: Float) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<Float> for Vector {
    fn div_assign(&mut self, rhs: Float) {
        self.data /= rhs;
    }
}

impl Index<usize> for Vector {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl From<Point> for Vector {
    #[inline]
    fn from(value: Point) -> Self {
        Self { data: value.data }
    }
}

impl From<Color> for Vector {
    #[inline]
    fn from(value: Color) -> Self {
        Self { data: value.data }
    }
}
