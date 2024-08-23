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
    let image_dims = Dimensions(400, 225);
    let camera = Camera::sensible_defaults(image_dims);
    let world = make_world();

    let mut handles = Vec::new();
    for (id, (dimensions, offset)) in threads::determine_work(image_dims).into_iter().enumerate() {
        let world = world.clone();
        let handle = std::thread::spawn(move || {
            let canvas = camera
                .render::<fn(&Ray, &World) -> Color>(id, dimensions, offset, &world, ray_color);

            (id, canvas)
        });

        handles.push(handle);
    }

    threads::join_canvases(handles).export("picture.png");
}
