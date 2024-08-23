use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Point,
    radius: Float,
}

impl Sphere {
    pub fn new(center: Point, radius: Float) -> Self {
        Self { center, radius }
    }
}

impl Geometry for Sphere {
    fn collide(&self, ray: &Ray, t_range: Range) -> Option<Collision> {
        let center_vector = self.center - ray.origin;

        let c = center_vector.norm_squared() - self.radius * self.radius;
        let minus_half_b = ray.direction.dot(&center_vector);
        let a = ray.direction.norm_squared();

        let delta = minus_half_b * minus_half_b - a * c;
        if delta < 0.0 {
            return None;
        }

        let delta_sqrt = delta.sqrt();
        let mut root = (minus_half_b - delta_sqrt) / a;
        if !t_range.contains(root) {
            root = (minus_half_b + delta_sqrt) / a;
            if !t_range.contains(root) {
                return None;
            }
        }

        let glancing_point = ray.at(root);
        let mut normal = (glancing_point - self.center) / self.radius;

        let is_front_facing = is_front_face(ray, normal);
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
