use super::*;
use geometry::Geometry;
use io::PngTile;

pub mod pinhole;
pub mod thin_lens;

// Reexporting useful types
pub use pinhole::Pinhole;
pub use thin_lens::ThinLens;

pub trait Camera: std::marker::Send + std::marker::Sync {
    fn cast(&self, u: Float, v: Float) -> Ray;
}

#[derive(Debug, Clone, Copy)]
pub struct Renderer<C>
where
    C: Camera,
{
    camera: C,
    samples_per_pixel: usize,
    max_depth: usize,
}

impl<C> Renderer<C>
where
    C: Camera,
{
    pub fn new(camera: C, samples_per_pixel: usize, max_depth: usize) -> Self {
        Self {
            camera,
            samples_per_pixel,
            max_depth,
        }
    }

    fn jitter_batch(&self) -> Vec<(Float, Float)> {
        let mut jitter = Vec::new();

        for _ in 0..self.samples_per_pixel {
            let random = random::random_vector(-0.5, 0.5);
            jitter.push((random.x, random.y));
        }

        jitter
    }

    pub fn render(
        &self,
        id: usize,
        dimensions: Dimensions,
        offset: TileCorner,
        geometry: &dyn Geometry,
    ) -> PngTile {
        let mut canvas = PngTile::with_offset(dimensions, offset);

        for (index, j) in (offset.1..(offset.1 + dimensions.1)).enumerate() {
            eprintln!("Thread {id}: {index} / {} scanlines", dimensions.1);

            for i in offset.0..(offset.0 + dimensions.0) {
                let mut color = Color::default();

                for (du, dv) in self.jitter_batch().iter() {
                    let ray = self.camera.cast(i as Float + du, j as Float + dv);
                    color += ray_color(&ray, geometry, self.max_depth);
                }

                color /= self.samples_per_pixel as Float;
                canvas.set(i, j, color);
            }
        }

        canvas
    }
}

fn ray_color(ray: &Ray, world: &dyn Geometry, depth: usize) -> Color {
    if depth == 0 {
        return Color::default();
    }

    match world.collide(ray, Range(0.001, Float::INFINITY)) {
        Some(collision) => match collision.material.scatter(ray, &collision) {
            Some(scatter) => {
                ray_color(&scatter.scattered, world, depth - 1).component_mul(&scatter.attenuation)
            }
            None => Color::default(),
        },
        _ => {
            let unit_direction = ray.direction.normalize();
            let t = (unit_direction.y + 1.0) / 2.0;

            color::WHITE.lerp(&Color::new(0.5, 0.7, 1.0), t)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: Float) -> Point {
        self.origin + t * self.direction
    }
}
