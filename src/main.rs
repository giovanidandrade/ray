use camera::{Camera, Ray};
use engine::*;
use io::PngTile;

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    Color::new(1.0, 1.0, 1.0).lerp(&Color::new(0.5, 0.7, 1.0), t)
}

fn main() {
    let image_dims = Dimensions(400, 225);
    let camera = Camera::sensible_defaults(image_dims);

    let num_cores = threads::estimate_cores();
    let work = threads::determine_work(image_dims, num_cores);

    let mut handles = Vec::new();
    for (id, (dims, offset)) in work.into_iter().enumerate() {
        let handle = std::thread::spawn(move || {
            let mut canvas = PngTile::with_offset(dims, offset);

            for j in offset.1..(offset.1 + dims.1) {
                eprintln!("Thread {id}: {j} / {} scanlines", dims.1);

                for i in offset.0..(offset.0 + dims.0) {
                    let ray = camera.cast(i as Float, j as Float);
                    let color = ray_color(&ray);

                    canvas.set(i, j, color);
                }
            }

            (offset, canvas)
        });

        handles.push(handle);
    }

    threads::join_canvases(handles).export("picture.png");
}
