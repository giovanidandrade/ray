use super::*;
use material::Material;
use math::ZERO_TOL;
use std::sync::Arc;

#[derive(Clone)]
pub struct Parallelogram {
    corner: Point,
    bounds: BoundingBox,
    u: Vector,
    v: Vector,
    normal: Vector,
    material: Arc<dyn Material>,
}

impl Parallelogram {
    /// Creates a parallelogram from a corner and two vectors defining its sides.
    /// Will error out if u and v are LD
    pub fn new(corner: Point, u: Vector, v: Vector, material: Arc<dyn Material>) -> Arc<Self> {
        let cos_t = u.dot(&v) / (u.norm() * v.norm());
        assert! { (cos_t.abs() - 1.0).abs() > ZERO_TOL }

        let mut normal = u.cross(&v).normalize();
        normal /= normal.norm_squared();

        Arc::new(Self {
            corner,
            u,
            v,
            normal,
            bounds: Self::get_bounding_box(corner, u, v),
            material,
        })
    }

    /// Creates a parallelogram from three vertices: two of them opposing and one of the adjacent to the opposing vertices.
    pub fn from_vertices(
        corner: Point,
        adjacent: Point,
        opposing: Point,
        material: Arc<dyn Material>,
    ) -> Arc<Self> {
        let u = adjacent - corner;
        let v = opposing - corner - u;
        let normal = u.cross(&v).normalize();

        Arc::new(Self {
            corner,
            u,
            v,
            normal,
            bounds: Self::get_bounding_box(corner, u, v),
            material,
        })
    }

    fn get_bounding_box(corner: Point, u: Vector, v: Vector) -> BoundingBox {
        let bounds_a = BoundingBox::from_extrema(corner, corner + u + v);
        let bounds_b = BoundingBox::from_extrema(corner + u, corner + v);

        bounds_a.union(&bounds_b)
    }
}

impl Geometry for Parallelogram {
    fn collide(&self, ray: &Ray, t_range: Range) -> Option<Collision> {
        let denominator = self.normal.dot(&ray.direction);

        if denominator.abs() < ZERO_TOL {
            return None;
        }

        let d = self.normal.dot(&self.corner.into());
        let t = (d - self.normal.dot(&ray.origin.into())) / denominator;
        if t_range.not_contains(t) {
            return None;
        }

        let glancing_point = ray.at(t);

        let in_q_coords = glancing_point - self.corner;
        let alpha = self.normal.dot(&in_q_coords.cross(&self.v));
        let beta = self.normal.dot(&self.u.cross(&in_q_coords));

        let param_range = Range(0.0, 1.0);
        if param_range.not_contains(alpha) || param_range.not_contains(beta) {
            return None;
        }

        let (is_front_facing, normal) = get_face(ray, self.normal);

        Some(Collision {
            point: glancing_point,
            normal,
            t,
            is_front_facing,
            material: self.material.clone(),
        })
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }
}
