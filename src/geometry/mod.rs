use super::*;
use bounding::BoundingBox;
use camera::Ray;

pub mod sphere;

#[derive(Clone)]
pub struct Collision {
    pub point: Point,
    pub normal: Vector,
    pub t: Float,
    pub is_front_facing: bool,
    pub material: std::sync::Arc<dyn material::Material>,
}

pub trait Geometry: std::marker::Send + std::marker::Sync {
    fn collide(&self, ray: &Ray, t_range: Range) -> Option<Collision>;
    fn bounding_box(&self) -> BoundingBox;
}

impl Geometry for &[WorldObject] {
    fn collide(&self, ray: &Ray, t_range: Range) -> Option<Collision> {
        let mut closest_t = t_range.1;
        let mut closest_collision = None;

        for geometry in self.iter() {
            if let Some(collision) = geometry.collide(ray, Range(t_range.0, closest_t)) {
                closest_t = collision.t;
                closest_collision = Some(collision);
            }
        }

        closest_collision
    }

    fn bounding_box(&self) -> BoundingBox {
        let empty = Range::union_empty();
        let mut total_box = BoundingBox::new(empty, empty, empty);

        for elem in self.iter() {
            total_box = total_box.union(&elem.bounding_box());
        }

        total_box
    }
}

impl Geometry for &mut [WorldObject] {
    fn collide(&self, ray: &Ray, t_range: Range) -> Option<Collision> {
        let mut closest_t = t_range.1;
        let mut closest_collision = None;

        for geometry in self.iter() {
            if let Some(collision) = geometry.collide(ray, Range(t_range.0, closest_t)) {
                closest_t = collision.t;
                closest_collision = Some(collision);
            }
        }

        closest_collision
    }

    fn bounding_box(&self) -> BoundingBox {
        let empty = Range::union_empty();
        let mut total_box = BoundingBox::new(empty, empty, empty);

        for elem in self.iter() {
            total_box = total_box.union(&elem.bounding_box());
        }

        total_box
    }
}

/// Given a ray and an outward normal (i.e.: that points towards the ray origin), determines if that
/// normal is front facing or not and returns one that is guaranteed to point outwards of the geometry
fn get_face(ray: &Ray, outward_normal: Vector) -> (bool, Vector) {
    if ray.direction.dot(&outward_normal) > 0.0 {
        (false, -outward_normal)
    } else {
        (true, outward_normal)
    }
}
