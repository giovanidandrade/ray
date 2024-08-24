use camera::CameraBuilder;
use engine::Dimensions;
use engine::*;

fn main() {
    let image_dimensions = Dimensions(400, 225);
    let camera = CameraBuilder::new().build(image_dimensions, 100);
    let world = scene::make_world();

    threads::render_parallel(image_dimensions, camera, &world).export("picture.png");
}
