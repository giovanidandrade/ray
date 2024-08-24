use super::*;
use geometry::Geometry;
use io::PngTile;

#[derive(Debug, Clone, Copy)]
pub struct CameraBuilder {
    focal_length: Float,
    vertical_field_of_view: Float,
    camera_center: Point,
}

pub type BuilderResult = Result<(), BuilderError>;

pub enum BuilderError {
    NonPositiveFocalLength,
    FovOutOfRange,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            focal_length: 1.0,
            vertical_field_of_view: std::f32::consts::PI / 2.0,
            camera_center: Point::zeros(),
        }
    }
}

impl CameraBuilder {
    /// Creates a default camera with the common defaults I use in my renders: unit focal length, origin center and
    /// viewport height of 2
    pub fn new() -> Self {
        Self::default()
    }

    /// Will error out if the focal length is nonpositive.
    pub fn set_focal_length(&mut self, length: Float) -> BuilderResult {
        if length <= 0.0 {
            return Err(BuilderError::NonPositiveFocalLength);
        }

        self.focal_length = length;
        Ok(())
    }

    /// Will error out if the vertical field of view isn't an angle between 0 and 180 degrees.
    pub fn set_vertical_field_of_view(&mut self, angle: Float) -> BuilderResult {
        if Range(0.0, 180.0).not_contains(angle) {
            return Err(BuilderError::FovOutOfRange);
        }

        self.vertical_field_of_view = angle * std::f32::consts::PI / 360.0;
        Ok(())
    }

    pub fn set_camera_center(&mut self, center: Point) {
        self.camera_center = center;
    }

    pub fn build(&self, dimensions: Dimensions, samples_per_pixel: usize) -> Camera {
        Camera::new(
            self.focal_length,
            self.vertical_field_of_view,
            dimensions,
            self.camera_center,
            samples_per_pixel,
        )
    }
}

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
        vertical_field_of_view: Float,
        dimensions: Dimensions,
        camera_center: Point,
        samples_per_pixel: usize,
    ) -> Self {
        let Dimensions(image_width, image_height) = dimensions;

        let h = (vertical_field_of_view / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
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

    pub fn render(
        &self,
        id: usize,
        dimensions: Dimensions,
        offset: TileCorner,
        world: &World,
    ) -> PngTile {
        let mut canvas = PngTile::with_offset(dimensions, offset);

        for (index, j) in (offset.1..(offset.1 + dimensions.1)).enumerate() {
            eprintln!("Thread {id}: {index} / {} scanlines", dimensions.1);

            for i in offset.0..(offset.0 + dimensions.0) {
                let mut color = Color::default();

                for (du, dv) in self.jitter_batch().iter() {
                    let ray = self.cast(i as Float + du, j as Float + dv);
                    color += ray_color(&ray, world, 10);
                }

                color /= self.samples_per_pixel as Float;
                canvas.set(i, j, color);
            }
        }

        canvas
    }
}

fn ray_color(ray: &Ray, world: &World, depth: usize) -> Color {
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

            WHITE.lerp(&Color::new(0.5, 0.7, 1.0), t)
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
