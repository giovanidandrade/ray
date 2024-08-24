use super::*;
use geometry::Geometry;
use io::PngTile;

#[derive(Debug, Clone, Copy)]
pub struct CameraBuilder {
    vertical_field_of_view: Float,
    look_from: Point,
    look_at: Point,
    v_up: Vector,
    max_depth: usize,
}

pub type BuilderResult = Result<(), BuilderError>;

#[derive(Debug, Clone, Copy)]
pub enum BuilderError {
    FovOutOfRange,
    LookingAtCenter,
    ZeroVup,
    ZeroDepth,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            vertical_field_of_view: std::f32::consts::PI / 2.0,
            look_from: Point::zeros(),
            look_at: -Point::z(),
            v_up: Vector::y(),
            max_depth: 10,
        }
    }
}

impl CameraBuilder {
    /// Creates a default camera with the common defaults I use in my renders: unit focal length, origin center and
    /// viewport height of 2
    pub fn new() -> Self {
        Self::default()
    }

    /// Will error out if the vertical field of view isn't an angle between 0 and 180 degrees.
    pub fn set_vertical_field_of_view(&mut self, angle: Float) -> BuilderResult {
        if Range(0.0, 180.0).not_contains(angle) {
            return Err(BuilderError::FovOutOfRange);
        }

        self.vertical_field_of_view = angle * std::f32::consts::PI / 180.0;
        Ok(())
    }

    /// Will error out if look_from is the same point as look_at
    pub fn set_look_from(&mut self, look_from: Point) -> BuilderResult {
        if look_from == self.look_at {
            return Err(BuilderError::LookingAtCenter);
        }

        self.look_from = look_from;
        Ok(())
    }

    /// Will error out if look_at is the same point as look_from
    pub fn set_look_at(&mut self, look_at: Point) -> BuilderResult {
        if self.look_from == look_at {
            return Err(BuilderError::LookingAtCenter);
        }

        self.look_at = look_at;
        Ok(())
    }

    /// Normalizes v_up before storing it.
    /// Will error out if v_up has a norm sufficiently close to 0
    pub fn set_vup(&mut self, v_up: Vector) -> BuilderResult {
        if v_up.norm_squared() < 1e-6 {
            return Err(BuilderError::ZeroVup);
        }

        self.v_up = v_up.normalize();
        Ok(())
    }

    /// Will error out if the depth is 0
    pub fn set_max_depth(&mut self, depth: usize) -> BuilderResult {
        if depth == 0 {
            return Err(BuilderError::ZeroDepth);
        }

        self.max_depth = depth;
        Ok(())
    }

    pub fn build(&self, dimensions: Dimensions, samples_per_pixel: usize) -> Camera {
        Camera::new(
            self.vertical_field_of_view,
            dimensions,
            self.look_from,
            self.look_at,
            self.v_up,
            samples_per_pixel,
            self.max_depth,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    upper_left_pixel_center: Point,
    look_from: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
    samples_per_pixel: usize,
    max_depth: usize,
}

impl Camera {
    pub fn new(
        vertical_field_of_view: Float,
        dimensions: Dimensions,
        look_from: Point,
        look_at: Point,
        v_up: Vector,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        let Dimensions(image_width, image_height) = dimensions;

        let focal_length = (look_from - look_at).norm();

        let h = (vertical_field_of_view / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as Float) / (image_height as Float);

        // Creating the orthonormal basis for the camera
        let camera_w = (look_from - look_at) / focal_length;
        let camera_u = v_up.cross(&camera_w);
        let camera_v = camera_w.cross(&camera_u);

        let viewport_u = viewport_width * camera_u;
        let viewport_v = viewport_height * -camera_v;

        let pixel_delta_u = viewport_u / (image_width as Float);
        let pixel_delta_v = viewport_v / (image_height as Float);

        let upper_left_corner =
            look_from - (focal_length * camera_w) - viewport_u / 2.0 - viewport_v / 2.0;

        let upper_left_pixel_center = upper_left_corner + (pixel_delta_u + pixel_delta_v) / 2.0;

        Self {
            upper_left_pixel_center,
            look_from,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn cast(&self, u: Float, v: Float) -> Ray {
        let direction =
            self.upper_left_pixel_center + u * self.pixel_delta_u + v * self.pixel_delta_v
                - self.look_from;

        Ray::new(self.look_from, direction)
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
                    color += ray_color(&ray, world, self.max_depth);
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
