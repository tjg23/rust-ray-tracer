use crate::{core::*, models::*, surfaces::*};

use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, texture: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(texture)),
        }
    }
    pub fn from_color(boundary: Arc<dyn Hittable>, density: f64, color: Color) -> Self {
        Self::new(boundary, density, Arc::new(SolidColor::new(color)))
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(ray, Interval::universe()) {
            if let Some(mut rec2) = self.boundary.hit(
                ray,
                Interval::from_range(rec1.t + 0.0001..std::f64::INFINITY),
            ) {
                rec1.t = rec1.t.max(t.start);
                rec2.t = rec2.t.min(t.end);
                if rec1.t >= rec2.t {
                    return None;
                }
                rec1.t = rec1.t.max(0.0);
                let ray_length = ray.direction.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = rec1.t + hit_distance / ray_length;
                let point = ray.at(t);
                Some(HitRecord::new(
                    ray,
                    t,
                    point,
                    Vec3(1.0, 0.0, 0.0),
                    self.phase_function.clone(),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bound(&self) -> BoundingBox {
        self.boundary.bound()
    }
}
