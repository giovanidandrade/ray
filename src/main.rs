use engine::Dimensions;
use engine::*;
use render::{Renderer, ThinLens};

fn main() {
    let image_dimensions = Dimensions(1200, 675);

    let camera = ThinLens::new(
        image_dimensions,
        Point::new(13.0, 2.0, 3.0),
        Point::zeros(),
        Vector::y(),
        20.0,
        10.0,
        0.6,
    );

    let renderer = Renderer::new(camera, 500, 50);
    let world = scene::make_world();

    parallelization::render(image_dimensions, renderer, &world, 1).export("picture.png");
}
