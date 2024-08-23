use super::*;
use io::PngTile;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    upper_left_pixel_center: Point,
    center: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
    samples_per_pixel: usize,
}

impl Camera {
    pub fn new(
        focal_length: Float,
        viewport_height: Float,
        dimensions: Dimensions,
        camera_center: Point,
        samples_per_pixel: usize,
    ) -> Self {
        let Dimensions(image_width, image_height) = dimensions;

        let viewport_width = viewport_height * (image_width as Float) / (image_height as Float);

        let viewport_u = Vector::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (image_width as Float);
        let pixel_delta_v = viewport_v / (image_height as Float);

        let upper_left_corner = camera_center
            - Vector::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let upper_left_pixel_center = upper_left_corner + (pixel_delta_u + pixel_delta_v) / 2.0;

        Self {
            upper_left_pixel_center,
            center: camera_center,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
        }
    }

    /// Returns a Camera with the common defaults I use in my renders: unit focal length, origin center and
    /// viewport height of 2 and 30 AA filters
    pub fn sensible_defaults(dimensions: Dimensions) -> Self {
        Self::new(1.0, 2.0, dimensions, Point::zeros(), 30)
    }

    pub fn cast(&self, u: Float, v: Float) -> Ray {
        let direction =
            self.upper_left_pixel_center + u * self.pixel_delta_u + v * self.pixel_delta_v
                - self.center;

        Ray::new(self.center, direction)
    }

    fn jitter_batch(&self) -> Vec<(Float, Float)> {
        use rand::Rng;

        let mut jitter = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..self.samples_per_pixel {
            let du = rng.gen::<Float>() - 0.5;
            let dv = rng.gen::<Float>() - 0.5;

            jitter.push((du, dv));
        }

        jitter
    }

    pub fn render<F>(
        &self,
        id: usize,
        dimensions: Dimensions,
        offset: TileCorner,
        world: &World,
        render_fn: impl Fn(&Ray, &World) -> Color,
    ) -> PngTile {
        let mut canvas = PngTile::with_offset(dimensions, offset);

        for j in offset.1..(offset.1 + dimensions.1) {
            eprintln!("Thread {id}: {j} / {} scanlines", dimensions.1);

            for i in offset.0..(offset.0 + dimensions.0) {
                let mut color = Color::default();

                for (du, dv) in self.jitter_batch().iter() {
                    let ray = self.cast(i as Float + du, j as Float + dv);
                    color += render_fn(&ray, world);
                }

                color /= self.samples_per_pixel as Float;
                canvas.set(i, j, color);
            }
        }

        canvas
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
        self.origin.lerp(&self.direction, t)
    }
}
