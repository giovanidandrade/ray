use engine::Dimensions;
use engine::*;
use render::Renderer;

fn main() {
    let image_dimensions = Dimensions(400, 225);
    let (camera, world) = scene::make_world2(image_dimensions);
    let renderer = Renderer::new(camera, 500, 50);

    parallelization::render(image_dimensions, renderer, &world, 1).export("picture.png");
}
