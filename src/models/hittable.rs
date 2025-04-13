use crate::{vec3::*, BoundingBox, Interval, Material, Point, Ray};

use std::sync::Arc;

pub use transformation::*;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
    pub material: Arc<dyn Material>,
    pub emitted: Color,
}

impl HitRecord {
    pub fn new(ray: &Ray, t: f64, point: Point, normal: Vec3, material: Arc<dyn Material>) -> Self {
        let front_face = Vec3::dot(&ray.direction, &normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        Self {
            point,
            normal,
            t,
            front_face,
            u: 0.0,
            v: 0.0,
            material,
            emitted: color(0., 0., 0.),
        }
    }
    pub fn set_uv(&mut self, u: f64, v: f64) -> Self {
        self.u = u;
        self.v = v;
        self.clone()
    }
    pub fn set_material(&mut self, material: Arc<dyn Material>) -> Self {
        self.material = material;
        self.clone()
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord>;

    fn bound(&self) -> BoundingBox;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bounds: BoundingBox,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bounds: BoundingBox::empty(),
        }
    }
    pub fn from(object: Arc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add_arc(object);
        list
    }

    pub fn add_arc(&mut self, object: Arc<dyn Hittable>) {
        self.bounds = BoundingBox::from_boxes(self.bounds, object.bound());
        self.objects.push(object);
    }
    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.add_arc(Arc::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = t.end;
        let mut hit_record = None;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, Interval::from_range(t.start..closest_so_far)) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
    fn bound(&self) -> BoundingBox {
        self.bounds
    }
}

pub mod transformation {
    use super::*;

    pub struct Translation {
        pub object: Arc<dyn Hittable>,
        pub offset: Vec3,
        bounds: BoundingBox,
    }

    impl Translation {
        pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
            let bounds = object.bound() + offset;
            Self {
                object,
                offset,
                bounds,
            }
        }
    }

    impl Hittable for Translation {
        fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
            let moved_ray = Ray {
                origin: ray.origin - self.offset,
                direction: ray.direction,
            };
            if let Some(mut record) = self.object.hit(&moved_ray, t) {
                record.point += self.offset;
                Some(record)
            } else {
                None
            }
        }
        fn bound(&self) -> BoundingBox {
            self.bounds
        }
    }

    pub struct RotateY {
        object: Arc<dyn Hittable>,
        sin_theta: f64,
        cos_theta: f64,
        bounds: BoundingBox,
    }

    impl RotateY {
        pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
            let radians = angle.to_radians();
            let sin_theta = radians.sin();
            let cos_theta = radians.cos();
            let mut bounds = object.bound();

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * bounds.intervals[0].end
                            + (1 - i) as f64 * bounds.intervals[0].start;
                        let y = j as f64 * bounds.intervals[1].end
                            + (1 - j) as f64 * bounds.intervals[1].start;
                        let z = k as f64 * bounds.intervals[2].end
                            + (1 - k) as f64 * bounds.intervals[2].start;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester = Vec3(new_x, y, new_z);

                        for c in 0..3 {
                            bounds.intervals[c] = Interval::from_pair(
                                bounds.intervals[c],
                                Interval::new(tester[c], tester[c]),
                            );
                        }
                    }
                }
            }

            Self {
                object,
                sin_theta,
                cos_theta,
                bounds,
            }
        }
    }

    impl Hittable for RotateY {
        fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
            let mut origin = ray.origin;
            let mut direction = ray.direction;

            origin.0 = self.cos_theta * ray.origin.0 - self.sin_theta * ray.origin.2;
            origin.2 = self.sin_theta * ray.origin.0 + self.cos_theta * ray.origin.2;

            direction.0 = self.cos_theta * ray.direction.0 - self.sin_theta * ray.direction.2;
            direction.2 = self.sin_theta * ray.direction.0 + self.cos_theta * ray.direction.2;

            let rotated_ray = Ray { origin, direction };

            if let Some(mut record) = self.object.hit(&rotated_ray, t) {
                let mut point = record.point;
                let mut normal = record.normal;

                point.0 = self.cos_theta * record.point.0 + self.sin_theta * record.point.2;
                point.2 = -self.sin_theta * record.point.0 + self.cos_theta * record.point.2;

                normal.0 = self.cos_theta * record.normal.0 + self.sin_theta * record.normal.2;
                normal.2 = -self.sin_theta * record.normal.0 + self.cos_theta * record.normal.2;

                record.point = point;
                record.normal = normal;

                Some(record)
            } else {
                None
            }
        }

        fn bound(&self) -> BoundingBox {
            self.bounds
        }
    }
}
