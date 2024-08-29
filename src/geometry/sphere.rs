use super::*;
use bounding::BoundingBox;
use material::Material;
use std::sync::Arc;
use transform::Transform;

#[derive(Clone)]
pub struct Ellipsoid {
    transform: Transform,
    material: Arc<dyn Material>,
    bounds: BoundingBox,
}

impl Ellipsoid {
    /// Builds a sphere given a center and a radius
    /// Will error out if the radius is 0
    pub fn sphere(center: Point, radius: Float, material: Arc<dyn Material>) -> Arc<Self> {
        let radius_vec = radius * vector::ONES;
        let transform = Transform::scale_all(1.0 / radius) * Transform::make_origin(center);

        Arc::new(Self {
            transform,
            material,
            bounds: BoundingBox::from_extrema(center - radius_vec, center + radius_vec),
        })
    }

    /// Builds an axis-aligned ellipsoid given a center and a vector of semiaxis lengths.
    /// Will error out if any length is 0
    pub fn new(center: Point, semiaxes: Vector, material: Arc<dyn Material>) -> Arc<Self> {
        let transform = Transform::scale(1.0 / semiaxes.x, 1.0 / semiaxes.y, 1.0 / semiaxes.z)
            * Transform::make_origin(center);

        Arc::new(Self {
            transform,
            material,
            bounds: BoundingBox::from_extrema(center - semiaxes, center + semiaxes),
        })
    }
}

impl Geometry for Ellipsoid {
    fn collide(&self, ray: &Ray, t_range: Range) -> Option<Collision> {
        let (original_ray, ray) = (ray, self.transform * (*ray));
        let center_origin = (-ray.origin).into();

        let a = ray.direction.norm_squared();
        let h = ray.direction.dot(&center_origin);
        let c = center_origin.norm_squared() - 1.0;

        let delta = h * h - a * c;
        if delta < 0.0 {
            return None;
        }

        let delta_sqrt = delta.sqrt();
        let mut root = (h - delta_sqrt) / a;
        if t_range.not_contains(root) {
            root = (h + delta_sqrt) / a;
            if t_range.not_contains(root) {
                return None;
            }
        }

        let glancing_point = ray.at(root);
        let mut collision = Collision {
            point: glancing_point,
            normal: glancing_point.into(),
            t: root,
            is_front_facing: true,
            material: self.material.clone(),
        };
        collision.apply(&original_ray, self.transform);

        Some(collision)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }
}
