use super::*;
use bounding::hierarchy::BoundingHierarchy;
use geometry::{flat::Parallelogram, sphere::Ellipsoid};
use material::{dielectric::Dielectric, diffuse::Lambertian, metal::Metal};
use render::{Pinhole, ThinLens};
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

pub fn make_world2(image_dimensions: Dimensions) -> (Pinhole, Arc<BoundingHierarchy>) {
    let ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));

    let center = Lambertian::new(Color::new(0.1, 0.2, 0.5));

    let glass = Dielectric::new(1.5);
    let bubble = Dielectric::new(1.0 / 1.5);

    let metal = Metal::polished(Color::new(0.8, 0.6, 0.2));

    let mut world: Vec<WorldObject> = vec![
        Ellipsoid::sphere(Point::new(0.0, -100.5, -1.0), 100.0, ground),
        Ellipsoid::sphere(Point::new(-1.0, 0.0, -1.0), 0.5, glass),
        Ellipsoid::sphere(Point::new(-1.0, 0.0, -1.0), 0.4, bubble),
        Ellipsoid::sphere(Point::new(0.0, 0.0, -1.2), 0.5, center),
        Ellipsoid::sphere(Point::new(1.0, 0.0, -1.0), 0.5, metal.clone()),
        Parallelogram::new(
            Point::new(-1.0, 0.6, -1.0),
            2.0 * Vector::x(),
            Vector::new(0.0, 1.0, 1.0),
            metal,
        ),
    ];

    (
        Pinhole::sensible_defaults(image_dimensions),
        BoundingHierarchy::from_vec(&mut world),
    )
}
