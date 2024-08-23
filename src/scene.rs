use geometry::{plane::Plane, sphere::Sphere};
use std::sync::Arc;

use super::*;
use crate::{material::lambertian::LambertianAlwaysScatters, World};

pub fn make_world() -> World {
    let blue = Arc::new(LambertianAlwaysScatters::new(Color::new(0.1, 0.2, 0.5)));
    let bg = Arc::new(LambertianAlwaysScatters::new(Color::new(0.5, 0.7, 1.0)));

    vec![
        Arc::new(Sphere::new(-Point::z(), 0.5, blue.clone())),
        Arc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, blue)),
        Arc::new(Plane::new(
            Vector::new(0.0, 0.0, 1.0),
            Point::new(0.0, 0.0, -10.0),
            bg,
        )),
    ]
}
