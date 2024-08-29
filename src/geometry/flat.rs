use super::*;
use material::Material;
use math::ZERO_TOL;
use std::sync::Arc;
use transform::Transform;

#[derive(Clone)]
pub struct Parallelogram {
    transform: Transform,
    bounds: BoundingBox,
    material: Arc<dyn Material>,
}

impl Parallelogram {
    /// Creates a parallelogram from a corner and two vectors defining its sides.
    /// Will error out if u and v are LD
    pub fn new(corner: Point, u: Vector, v: Vector, material: Arc<dyn Material>) -> Arc<Self> {
        let transform = Transform::make_xy(u, v) * Transform::make_origin(corner);

        Arc::new(Self {
            transform,
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
        let transform = Transform::make_xy(u, v) * Transform::make_origin(corner);

        Arc::new(Self {
            transform,
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
        let (original_ray, ray) = (ray, self.transform * (*ray));

        if ray.direction.z.abs() <= ZERO_TOL {
            return None;
        }

        let t = -ray.origin.z / ray.direction.z;
        if t_range.not_contains(t) {
            return None;
        }

        let glancing_point = ray.at(t);
        let param_range = Range(0.0, 1.0);
        if param_range.not_contains(glancing_point.x) || param_range.not_contains(glancing_point.y)
        {
            return None;
        }

        let normal = Vector::z() * ray.direction.z.signum();
        let (is_front_facing, normal) = get_face(&ray, normal);
        let mut collision = Collision {
            point: glancing_point,
            normal,
            t,
            is_front_facing,
            material: self.material.clone(),
        };
        collision.apply(&original_ray, self.transform);

        Some(collision)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }
}
