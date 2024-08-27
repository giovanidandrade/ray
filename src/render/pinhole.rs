use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Pinhole {
    center: Point,
    upper_left_pixel: Point,
    pixel_du: Vector,
    pixel_dv: Vector,
}

impl Pinhole {
    /// Builds a pinhole camera with 90º of vfov, looking at (0, 0, -1) from the origin, with (0, 1, 0) as up
    pub fn sensible_defaults(dimensions: Dimensions) -> Self {
        Self::new(dimensions, Point::zeros(), -Point::z(), Vector::y(), 90.0)
    }

    pub fn new(
        dimensions: Dimensions,
        look_from: Point,
        look_at: Point,
        v_up: Vector,
        vertical_fov: Float,
    ) -> Self {
        let Dimensions(width, height) = dimensions;
        let center = look_from;

        let focal_length = (look_from - look_at).norm();
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (width as Float) / (height as Float);

        let w = (look_from - look_at) / focal_length;
        let u = v_up.cross(&w);
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_du = viewport_u / (width as Float);
        let pixel_dv = viewport_v / (height as Float);

        let viewport_upper_left = center - (focal_length * w) - (viewport_u + viewport_v) / 2.0;
        let upper_left_pixel = viewport_upper_left + (pixel_du + pixel_dv) / 2.0;

        Self {
            center,
            upper_left_pixel,
            pixel_du,
            pixel_dv,
        }
    }
}

impl Camera for Pinhole {
    fn cast(&self, u: Float, v: Float) -> Ray {
        let sample = self.upper_left_pixel + u * self.pixel_du + v * self.pixel_dv;
        let origin = self.center;

        Ray::new(origin, sample - origin)
    }
}
