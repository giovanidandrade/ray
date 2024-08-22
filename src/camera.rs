use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    upper_left_pixel_center: Point,
    center: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
}

impl Camera {
    pub fn new(
        focal_length: Float,
        viewport_height: Float,
        dimensions: Dimensions,
        camera_center: Point,
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
        }
    }

    /// Returns a Camera with the common defaults I use in my renders: unit focal length, origin center and
    /// viewport height of 2
    pub fn sensible_defaults(dimensions: Dimensions) -> Self {
        Self::new(1.0, 2.0, dimensions, Point::zeros())
    }

    pub fn cast(&self, u: Float, v: Float) -> Ray {
        let direction =
            self.upper_left_pixel_center + u * self.pixel_delta_u + v * self.pixel_delta_v
                - self.center;

        Ray::new(self.center, direction)
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
