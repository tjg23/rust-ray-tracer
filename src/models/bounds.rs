use std::{
    ops::{Add, Range},
    sync::Arc,
};

use crate::{hittable::*, Interval, Point, Ray, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    pub intervals: [Interval; 3],
}
impl Add<Vec3> for BoundingBox {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        Self {
            intervals: [
                self.intervals[0] + rhs.x(),
                self.intervals[1] + rhs.y(),
                self.intervals[2] + rhs.z(),
            ],
        }
    }
}

impl BoundingBox {
    pub fn empty() -> Self {
        Self {
            intervals: [Interval::empty(); 3],
        }
    }
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut b = Self {
            intervals: [x, y, z],
        };
        b.pad_min()
    }
    pub fn from_points(a: Point, b: Point) -> Self {
        let x = Interval::new(a.x().min(b.x()), a.x().max(b.x()));
        let y = Interval::new(a.y().min(b.y()), a.y().max(b.y()));
        let z = Interval::new(a.z().min(b.z()), a.z().max(b.z()));
        Self::new(x, y, z)
    }
    pub fn from_boxes(a: BoundingBox, b: BoundingBox) -> Self {
        let x = Interval::from_pair(a.intervals[0], b.intervals[0]);
        let y = Interval::from_pair(a.intervals[1], b.intervals[1]);
        let z = Interval::from_pair(a.intervals[2], b.intervals[2]);
        Self::new(x, y, z)
    }

    pub fn longest_axis(&self) -> usize {
        if self.intervals[0].size() > self.intervals[1].size() {
            if self.intervals[0].size() > self.intervals[2].size() {
                0
            } else {
                2
            }
        } else {
            if self.intervals[1].size() > self.intervals[2].size() {
                1
            } else {
                2
            }
        }
    }

    fn pad_min(&mut self) -> Self {
        let delta = 0.0001;
        if self.intervals[0].size() < delta {
            self.intervals[0] = self.intervals[0].expand(delta);
        }
        if self.intervals[1].size() < delta {
            self.intervals[1] = self.intervals[1].expand(delta);
        }
        if self.intervals[2].size() < delta {
            self.intervals[2] = self.intervals[2].expand(delta);
        }
        self.clone()
    }
}

pub trait Bounds {
    fn hit(&self, ray: &Ray, t: Interval) -> bool;
}

impl Bounds for BoundingBox {
    fn hit(&self, ray: &Ray, t: Interval) -> bool {
        for i in 0..3 {
            let ax = self.intervals[i];
            let adinv = 1.0 / ray.direction[i];

            let t0 = (ax.start - ray.origin[i]) * adinv;
            let t1 = (ax.end - ray.origin[i]) * adinv;

            let (t0, t1) = (t0.min(t1), t0.max(t1));
            let t = Interval::new(t.start.max(t0), t.end.min(t1));
            if t.size() <= 0.0 {
                return false;
            }
        }
        true
    }
}

pub struct BoundNode {
    bounds: BoundingBox,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl BoundNode {
    // pub fn new(bounds: BoundingBox) -> Self {
    //     Self {
    //         bounds,
    //         left: None,
    //         right: None,
    //     }
    // }
    pub fn from_objects(objects: &Vec<Arc<dyn Hittable>>, range: Range<usize>) -> Self {
        let mut bounds = BoundingBox::empty();
        for i in range.clone() {
            bounds = BoundingBox::from_boxes(bounds, objects[i].bound());
        }
        let axis = bounds.longest_axis();

        let span = range.len();
        match span {
            0 => panic!("No objects in range"),
            1 => Self {
                bounds,
                left: objects[range.start].clone(),
                right: objects[range.start].clone(),
            },
            2 => Self {
                bounds,
                left: objects[range.start].clone(),
                right: objects[range.start + 1].clone(),
            },
            _ => {
                let mut objects = objects.clone();
                objects[range.clone()].sort_by(|a, b| {
                    let a = a.bound().intervals[axis].start;
                    let b = b.bound().intervals[axis].start;
                    a.partial_cmp(&b).unwrap()
                });
                let mid = range.start + span / 2;
                let left = Self::from_objects(&objects, range.start..mid);
                let right = Self::from_objects(&objects, mid..range.end);
                Self {
                    bounds,
                    left: Arc::new(left),
                    right: Arc::new(right),
                }
            }
        }
    }
    pub fn from_list(list: HittableList) -> Self {
        let objects = list.objects.clone();
        let len = objects.len();
        Self::from_objects(&objects, 0..len)
    }
}

impl Hittable for BoundNode {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        if !self.bounds.hit(ray, t) {
            return None;
        }
        match (self.left.hit(ray, t), self.right.hit(ray, t)) {
            (Some(a), Some(b)) => {
                if a.t < b.t {
                    Some(a)
                } else {
                    Some(b)
                }
            }
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    fn bound(&self) -> BoundingBox {
        self.bounds
    }
}
