use super::*;
use math::ZERO_TOL;
use render::Ray;

type Matrix = nalgebra::Matrix4<Float>;
type Vec4 = nalgebra::Vector4<Float>;

pub mod ops;

/// Represents an invertible, affine transform
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    matrix: Matrix,
    inverse: Matrix,
}

impl Default for Transform {
    fn default() -> Self {
        let id = Matrix::identity();

        Self {
            matrix: id,
            inverse: id,
        }
    }
}

impl Transform {
    /// Makes a transform from its matrix.
    /// Will error out if the matrix isn't invertible.
    pub fn from_matrix(matrix: Matrix) -> Self {
        Self {
            matrix,
            inverse: matrix.try_inverse().unwrap(),
        }
    }

    /// Makes a transform from the matrix and inverse.
    /// Assumes that:
    /// - the matrix given is invertible
    /// - and that the inverse is indeed the inverse
    pub fn from_matrix_inverse(matrix: Matrix, inverse: Matrix) -> Self {
        Self { matrix, inverse }
    }

    pub fn identity() -> Self {
        let id = Matrix::identity();
        Self {
            matrix: id,
            inverse: id,
        }
    }

    pub fn translate(offset: Vector) -> Self {
        let id = Matrix::identity();

        let mut matrix = id;
        matrix.m14 = offset.x;
        matrix.m24 = offset.y;
        matrix.m34 = offset.z;

        let mut inverse = id;
        inverse.m14 = -offset.x;
        inverse.m24 = -offset.y;
        inverse.m34 = -offset.z;

        Self { matrix, inverse }
    }

    /// Returns the transform that translates `point` to the origin
    #[inline]
    pub fn make_origin(point: Point) -> Self {
        Self::translate((-point).into())
    }

    /// Makes a uniform scaling matrix.
    /// Will error out if lambda is 0
    pub fn scale_all(lambda: Float) -> Self {
        assert! { lambda.abs() > ZERO_TOL }
        let mut matrix = Matrix::from_diagonal_element(lambda);
        matrix.m44 = 1.0;

        let mut inverse = Matrix::from_diagonal_element(1.0 / lambda);
        inverse.m44 = 1.0;

        Self { matrix, inverse }
    }

    /// Makes a scaling matrix.
    /// Will error out if any of the arguments is 0
    pub fn scale(x: Float, y: Float, z: Float) -> Self {
        assert! { (x * y * z).abs() > ZERO_TOL }
        let matrix = Matrix::from_diagonal(&Vec4::new(x, y, z, 1.0));
        let inverse = Matrix::from_diagonal(&Vec4::new(1.0 / x, 1.0 / y, 1.0 / z, 1.0));

        Self { matrix, inverse }
    }
}
