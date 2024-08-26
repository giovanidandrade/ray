use super::*;
use bounding::hierarchy::BoundingHierarchy;
use geometry::sphere::Sphere;
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use random::random_float;

pub fn make_world() -> std::sync::Arc<BoundingHierarchy> {
    let mut world: World = Vec::new();

    let ground = Lambertian::new(0.5 * color::WHITE);

    let glass = Dielectric::new(1.5);
    let steel = Metal::polished(Color::new(0.7, 0.6, 0.5));
    let brown = Lambertian::new(Color::new(0.4, 0.2, 0.1));

    let big_objects: [WorldObject; 4] = [
        Sphere::new(Point::y(), 1.0, glass.clone()),
        Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, brown),
        Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, steel),
        Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground),
    ];
    world.extend(big_objects);

    for a in -11..11 {
        for b in -11..11 {
            let center = Point::new(
                a as Float + 0.9 * random_float(),
                0.2,
                b as Float + 0.9 * random_float(),
            );
            let test_point = Point::new(4.0, 0.2, 0.0);

            if (center - test_point).norm_squared() > 0.81 {
                let p = random::random_float();
                if p < 0.8 {
                    let albedo = random::random_color();
                    let material = Lambertian::new(albedo);

                    world.push(Sphere::new(center, 0.2, material));
                } else if p < 0.95 {
                    let albedo = random::random_color();
                    let fuzziness = random::random_float() / 2.0;
                    let metal = Metal::new(albedo, fuzziness);

                    world.push(Sphere::new(center, 0.2, metal));
                } else {
                    world.push(Sphere::new(center, 0.2, glass.clone()));
                }
            }
        }
    }

    BoundingHierarchy::from_vec(&mut world)
}
