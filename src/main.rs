use camera::{Camera, Ray};
use engine::*;
use geometry::{sphere::Sphere, Geometry};
use io::PngTile;
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
    for (id, (dims, offset)) in threads::determine_work(image_dims).into_iter().enumerate() {
        let world = world.clone();
        let handle = std::thread::spawn(move || {
            let mut canvas = PngTile::with_offset(dims, offset);

            for j in offset.1..(offset.1 + dims.1) {
                eprintln!("Thread {id}: {j} / {} scanlines", dims.1);

                for i in offset.0..(offset.0 + dims.0) {
                    let ray = camera.cast(i as Float, j as Float);
                    let color = ray_color(&ray, &world);

                    canvas.set(i, j, color);
                }
            }

            (id, canvas)
        });

        handles.push(handle);
    }

    threads::join_canvases(handles).export("picture.png");
}
