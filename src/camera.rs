use crate::{HittableList, Point, Ray, Vec3};

pub struct Camera {
    /* Image Dimensions */
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    center: Point,
    pixel_00: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    /* Point of View */
    pub vfov: f64,
    pub look_from: Point,
    pub look_at: Point,
    pub up: Vec3,

    /* Anti-Aliasing */
    pub aa_samples: i32,
    aa_scale: f64,

    /* Ray Behavior */
    pub max_depth: i32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        vfov: f64,
        look_from: Point,
        look_at: Point,
        up: Vec3,
        aa_samples: i32,
        max_depth: i32,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height >= 1 { image_height } else { 1 };

        // let look_from = Vec3(0.0, 0.0, 0.0);
        // let look_at = Vec3(0.0, 0.0, -1.0);
        // let up = Vec3(0.0, 1.0, 0.0);
        let center = look_from;

        let focal_length = (look_from - look_at).length();
        let theta = vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (look_from - look_at).unit();
        let u = Vec3::cross(&up, &w).unit();
        let v = Vec3::cross(&w, &u);
        // let basis = (u, v, w);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - (w * focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel_00 = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) / 2.0);

        // let aa_samples = 10;
        let aa_scale = 1.0 / aa_samples as f64;

        // let max_depth = 10;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel_00,
            pixel_delta_u,
            pixel_delta_v,
            vfov,
            look_from,
            look_at,
            up,
            // basis,
            aa_samples,
            aa_scale,
            max_depth,
        }
    }

    pub fn set_aa_samples(&mut self, aa_samples: i32) -> &mut Self {
        self.aa_samples = aa_samples;
        self.aa_scale = 1.0 / aa_samples as f64;
        self
    }

    pub fn set_max_depth(&mut self, max_depth: i32) -> &mut Self {
        self.max_depth = max_depth;
        self
    }

    pub fn move_camera(&mut self, look_from: Point, look_at: Point, up: Vec3) -> &mut Self {
        self.look_from = look_from;
        self.look_at = look_at;
        self.up = up;

        self.center = look_from;

        let focal_length = (look_from - look_at).length();
        let theta = self.vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let w = (look_from - look_at).unit();
        let u = Vec3::cross(&up, &w).unit();
        let v = Vec3::cross(&w, &u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (w * focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel_00 = viewport_upper_left + ((self.pixel_delta_u + self.pixel_delta_v) / 2.0);

        self
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                // let pixel_center = self.pixel_00
                //     + (self.pixel_delta_u * x as f64)
                //     + (self.pixel_delta_v * y as f64);
                // let ray = Ray {
                //     origin: self.center,
                //     direction: pixel_center - self.center,
                // };
                let mut color = Vec3(0.0, 0.0, 0.0);
                for _ in 0..self.aa_samples {
                    let ray = self.sample_ray(x, y);
                    color += ray.send(world, self.max_depth);
                }
                // ray.send(world).write_color();
                // write_color(&ray.send(world));
                (color * self.aa_scale).to_gamma().write_color();
            }
        }
    }

    pub fn sample_ray(&self, x: i32, y: i32) -> Ray {
        let offset = Vec3::sample_square();
        let pixel_sample = self.pixel_00
            + (self.pixel_delta_u * (x as f64 + offset.0))
            + (self.pixel_delta_v * (y as f64 + offset.1));
        Ray {
            origin: self.center,
            direction: pixel_sample - self.center,
        }
    }
}
