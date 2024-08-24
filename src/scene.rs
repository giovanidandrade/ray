use super::*;
use geometry::sphere::Sphere;
use material::lambertian::Lambertian;

pub fn make_world() -> World {
    let blue = Lambertian::new(Color::z());
    let red = Lambertian::new(Color::x());

    let radius = (0.5 as Float).sqrt(); // cos(pi / 4)

    vec![
        Sphere::new(Point::new(-radius, 0.0, -1.0), radius, blue),
        Sphere::new(Point::new(radius, 0.0, -1.0), radius, red),
    ]
}
