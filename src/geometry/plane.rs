use super::*;
use material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub struct Plane {
    normal: Vector,
    point: Point,
    material: Arc<dyn Material>,
}

impl Plane {
    pub fn new(normal: Vector, point: Point, material: Arc<dyn Material>) -> Self {
        Self {
            normal: normal.normalize(),
            point,
            material,
        }
    }
}

impl Geometry for Plane {
    fn collide(&self, ray: &Ray, t_range: Range) -> Option<Collision> {
        let a = self.normal.dot(&ray.direction);
        if a == 0.0 {
            return None;
        }

        let b = (self.point - ray.origin).dot(&self.normal);
        let root = b / a;
        if !t_range.contains(root) {
            return None;
        }

        let glancing_point = ray.at(root);
        let is_front_facing = a > 0.0;

        let mut normal = self.normal;
        if !is_front_facing {
            normal = -normal;
        };

        Some(Collision {
            t: root,
            point: glancing_point,
            normal,
            is_front_facing,
            material: self.material.clone(),
        })
    }
}
