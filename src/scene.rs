use super::*;
use bounding::hierarchy::BoundingHierarchy;
use geometry::parallelogram::Parallelogram;
use material::diffuse::Lambertian;
use render::ThinLens;
use std::sync::Arc;

pub fn make_world(image_dimensions: Dimensions) -> (ThinLens, Arc<BoundingHierarchy>) {
    let red = Lambertian::new(Color::new(1.0, 0.2, 0.2));
    let blue = Lambertian::new(Color::new(0.2, 0.2, 1.0));

    let mut world: Vec<WorldObject> = vec![
        Parallelogram::new(Point::new(-0.5, -0.5, 0.0), Vector::x(), Vector::y(), red),
        Parallelogram::new(Point::new(-0.5, 0.5, 0.0), Vector::z(), Vector::x(), blue),
    ];

    let camera = ThinLens::new(
        image_dimensions,
        Point::z(),
        Point::zeros(),
        Vector::y(),
        80.0,
        10.0,
        0.0,
    );

    (camera, BoundingHierarchy::from_vec(&mut world))
}
