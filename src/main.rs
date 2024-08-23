use camera::Camera;
use engine::Dimensions;
use engine::*;

fn main() {
    let image_dimensions = Dimensions(400, 225);
    let camera = Camera::sensible_defaults(image_dimensions);
    let world = scene::make_world();

    threads::render_parallel(image_dimensions, camera, &world).export("picture.png");
}
