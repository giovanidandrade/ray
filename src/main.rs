use camera::CameraBuilder;
use engine::Dimensions;
use engine::*;

fn main() {
    let image_dimensions = Dimensions(1200, 675);

    let mut builder = CameraBuilder::new();
    builder.set_look_from(Point::new(13.0, 2.0, 3.0)).unwrap();
    builder.set_look_at(Point::zeros()).unwrap();
    builder.set_vertical_field_of_view(20.0).unwrap();
    builder.set_defocus_angle(0.6).unwrap();
    builder.set_focus_distance(10.0).unwrap();
    builder.set_max_depth(50).unwrap();

    let camera = builder.build(image_dimensions, 500);
    let world = scene::make_world();

    parallelization::render(image_dimensions, camera, &world).export("picture.png");
}
