use super::*;
use rand::Rng;

pub fn random_float() -> Float {
    let mut rng = rand::thread_rng();
    rng.gen::<Float>()
}

pub fn random_vector(min: Float, max: Float) -> Vector {
    let mut rng = rand::thread_rng();

    Vector::new(
        (max - min) * rng.gen::<Float>() + min,
        (max - min) * rng.gen::<Float>() + min,
        (max - min) * rng.gen::<Float>() + min,
    )
}

/// Samples a random unit vector uniformly on the unit sphere
pub fn random_unit_vector() -> Vector {
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

/// Samples a random vector uniformly in the unit disk
pub fn random_in_unit_disk() -> Vector {
    let mut vec = random_vector(-1.0, 1.0);
    while vec.norm_squared() > 1.0 {
        vec = random_vector(-1.0, 1.0);
    }

    vec
}

/// Given a normal vector, returns a random unit vector in the same orientation as the normal vector
pub fn random_on_hemisphere(normal: &Vector) -> Vector {
    let mut vec = random_unit_vector();
    if normal.dot(&vec) <= 0.0 {
        vec = -vec;
    }

    vec
}
