use math::degrees_to_radians;

use super::*;

#[derive(Debug, Copy, Clone)]
pub struct ThinLens {
    center: Point,
    upper_left_pixel: Point,
    pixel_du: Vector,
    pixel_dv: Vector,
    defocus_du: Vector,
    defocus_dv: Vector,
}

impl ThinLens {
    pub fn sensible_defaults(dimensions: Dimensions) -> Self {
        Self::new(
            dimensions,
            Point::zeros(),
            -Point::z(),
            Vector::y(),
            90.0,
            1.0,
            10.0,
        )
    }

    pub fn new(
        dimensions: Dimensions,
        look_from: Point,
        look_at: Point,
        v_up: Vector,
        vertical_fov: Float,
        focal_distance: Float,
        defocus_angle: Float,
    ) -> Self {
        let Dimensions(width, height) = dimensions;
        let center = look_from;

        let theta = math::degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focal_distance;
        let viewport_width = viewport_height * (width as Float) / (height as Float);

        let w = (look_from - look_at).normalize();
        let u = v_up.cross(&w);
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_du = viewport_u / (width as Float);
        let pixel_dv = viewport_v / (height as Float);

        let viewport_upper_left = center - (focal_distance * w) - (viewport_u + viewport_v) / 2.0;
        let upper_left_pixel = viewport_upper_left + (pixel_du + pixel_dv) / 2.0;

        let defocus_radius = focal_distance * degrees_to_radians(defocus_angle);
        let defocus_du = u * defocus_radius;
        let defocus_dv = v * defocus_radius;

        Self {
            center,
            upper_left_pixel,
            pixel_du,
            pixel_dv,
            defocus_du,
            defocus_dv,
        }
    }

    fn sample_defocus_disk(&self) -> Point {
        let p = random::random_in_unit_disk();
        self.center + p.x * self.defocus_du + p.y * self.defocus_dv
    }
}

impl Camera for ThinLens {
    fn cast(&self, u: Float, v: Float) -> Ray {
        let sample = self.upper_left_pixel + u * self.pixel_du + v * self.pixel_dv;
        let origin = self.sample_defocus_disk();

        Ray::new(origin, sample - origin)
    }
}
