use camera::{Camera, Ray};
use engine::*;
use geometry::{sphere::Sphere, Geometry};
use std::sync::Arc;

fn make_world() -> World {
    vec![
        Arc::new(Sphere::new(-Point::z(), 0.5)),
        Arc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)),
    ]
}

fn ray_color(ray: &Ray, world: &World) -> Color {
    match world.collide(ray, Range(0.0, Float::INFINITY)) {
        Some(collision) => (WHITE + collision.normal) / 2.0,
        _ => {
            let unit_direction = ray.direction.normalize();
            let t = (unit_direction.y + 1.0) / 2.0;

            WHITE.lerp(&Color::new(0.5, 0.7, 1.0), t)
        }
    }
}

fn main() {
    let image_dimensions = Dimensions(400, 225);
    let camera = Camera::sensible_defaults(image_dimensions);
    let world = make_world();

    threads::render_parallel(image_dimensions, camera, &world, ray_color).export("picture.png");
}
