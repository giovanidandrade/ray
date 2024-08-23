use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    normal: Vector,
    point: Point,
}

impl Plane {
    pub fn new(normal: Vector, point: Point) -> Self {
        Self {
            normal: normal.normalize(),
            point,
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
        })
    }
}
