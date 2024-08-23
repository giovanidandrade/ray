use super::*;
use geometry::sphere::Sphere;
use material::{lambertian::Lambertian, metal::Metal};

pub fn make_world() -> World {
    let ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));

    let blue = Lambertian::new(Color::new(0.1, 0.2, 0.5));

    let steel = Metal::new(0.8 * WHITE);
    let gold = Metal::new(Color::new(0.8, 0.6, 0.2));

    vec![
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground),
        Sphere::new(Point::new(0.0, 0.0, -1.2), 0.5, blue),
        Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, steel),
        Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, gold),
    ]
}
