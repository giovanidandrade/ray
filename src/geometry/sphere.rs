use super::*;
use material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    center: Point,
    radius: Float,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: Float, material: Arc<dyn Material>) -> Arc<Self> {
        Arc::new(Self {
            center,
            radius,
            material,
        })
    }
}

impl Geometry for Sphere {
    fn collide(&self, ray: &Ray, t_range: Range) -> Option<Collision> {
        let center_origin = self.center - ray.origin;

        let a = ray.direction.norm_squared();
        let h = ray.direction.dot(&center_origin);
        let c = center_origin.norm_squared() - self.radius * self.radius;

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
        let (is_front_facing, outward_normal) =
            get_face(ray, (glancing_point - self.center) / self.radius);

        Some(Collision {
            point: glancing_point,
            normal: outward_normal,
            t: root,
            is_front_facing,
            material: self.material.clone(),
        })
    }
}
