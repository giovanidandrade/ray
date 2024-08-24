use camera::CameraBuilder;
use engine::Dimensions;
use engine::*;

fn main() {
    let image_dimensions = Dimensions(400, 225);

    let mut builder = CameraBuilder::new();
    builder.set_look_from(Point::new(-2.0, 2.0, 1.0)).unwrap();
    builder.set_vertical_field_of_view(20.0).unwrap();
    builder.set_max_depth(50).unwrap();

    let camera = builder.build(image_dimensions, 1000);
    let world = scene::make_world();

    threads::render_parallel(image_dimensions, camera, &world).export("picture.png");
}
