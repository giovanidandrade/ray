use super::*;
use rand::Rng;

pub fn random_vector(min: Float, max: Float) -> Vector {
    let mut rng = rand::thread_rng();

    Vector::new(
        (max - min) * rng.gen::<Float>() + min,
        (max - min) * rng.gen::<Float>() + min,
        (max - min) * rng.gen::<Float>() + min,
    )
}

pub fn random_vector_on_unit_sphere() -> Vector {
    let mut vec = random_vector(-1.0, 1.0);

    // While strictly speaking just normalizing would yield a vector on the unit sphere,
    // this rejection means that we are uniformly sampling *on* the unit sphere.
    //
    // If we just normalized the vector we'd end up with thining at the poles (where the sphere
    // touches the bounding box)
    while vec.norm_squared() > 1.0 {
        vec = random_vector(-1.0, 1.0);
    }

    vec.normalize()
}

/// Given a normal vector, returns a random unit vector in the same orientation as the normal vector
pub fn random_on_hemisphere(normal: &Vector) -> Vector {
    let mut vec = random_vector_on_unit_sphere();
    if normal.dot(&vec) <= 0.0 {
        vec = -vec;
    }

    vec
}
