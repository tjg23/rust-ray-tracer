use crate::{hittable::*, vec3::*, Interval, Point, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn background(&self) -> Color {
        // let unit_direction = self.direction.unit();
        // let t = 0.5 * (unit_direction.y() + 1.0);
        // Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
        color(0., 0., 0.)
    }

    pub fn hit<T: Hittable>(&self, object: &T, t: Interval) -> Option<HitRecord> {
        object.hit(self, t)
    }

    pub fn send(&self, world: &HittableList, depth: i32) -> Color {
        if depth <= 0 {
            return color(0.0, 0.0, 0.0);
        }
        if let Some(record) = self.hit(world, Interval::from_range(0.0001..f64::INFINITY)) {
            let emitted = record.material.emitted(record.u, record.v, &record.point);
            if let Some((scattered, attenuation)) = record.material.scatter(self, &record) {
                emitted + attenuation * scattered.send(world, depth - 1)
            } else {
                emitted
            }
        } else {
            self.background()
        }
    }
}
