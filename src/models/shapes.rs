use crate::{hittable::*, point, BoundingBox, Interval, Invisible, Material, Point, Ray, Vec3};

use std::{f64::consts::PI, sync::Arc};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
    bounds: BoundingBox,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        let bounds = BoundingBox::from_points(
            center - Vec3(radius, radius, radius),
            center + Vec3(radius, radius, radius),
        );
        Self {
            center,
            radius,
            material,
            bounds,
        }
    }

    pub fn get_uv(&self, p: &Vec3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = Vec3::dot(&ray.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !t_range.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t_range.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let (u, v) = self.get_uv(&normal);

        Some(HitRecord::new(ray, root, point, normal, self.material.clone()).set_uv(u, v))
    }

    fn bound(&self) -> BoundingBox {
        self.bounds
    }
}

pub struct Triangle {
    pub vertex: (Vec3, Vec3, Vec3),
    normal: Vec3,
    pub material: Arc<dyn Material>,
    bounds: BoundingBox,
}

impl Triangle {
    pub fn new(vertex: (Vec3, Vec3, Vec3), material: Arc<dyn Material>) -> Self {
        let normal = Vec3::cross(&(vertex.1 - vertex.0), &(vertex.2 - vertex.0));

        let min_x = vertex.0.x().min(vertex.1.x()).min(vertex.2.x());
        let min_y = vertex.0.y().min(vertex.1.y()).min(vertex.2.y());
        let min_z = vertex.0.z().min(vertex.1.z()).min(vertex.2.z());
        let max_x = vertex.0.x().max(vertex.1.x()).max(vertex.2.x());
        let max_y = vertex.0.y().max(vertex.1.y()).max(vertex.2.y());
        let max_z = vertex.0.z().max(vertex.1.z()).max(vertex.2.z());
        let bounds = BoundingBox::from_points(Vec3(min_x, min_y, min_z), Vec3(max_x, max_y, max_z));
        Self {
            vertex,
            normal,
            material,
            bounds,
        }
    }

    pub fn is_interior(alpha: f64, beta: f64) -> Option<(f64, f64)> {
        if alpha < 0.0 || beta < 0.0 || alpha + beta > 1.0 {
            return None;
        } else {
            Some((alpha, beta))
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, _t_range: Interval) -> Option<HitRecord> {
        let normal = Vec3::cross(
            &(self.vertex.1 - self.vertex.0),
            &(self.vertex.2 - self.vertex.0),
        );

        let p = ray.direction + ray.origin;
        let normal_a = Vec3::cross(&(self.vertex.2 - self.vertex.1), &(p - self.vertex.1));
        let normal_b = Vec3::cross(&(self.vertex.0 - self.vertex.2), &(p - self.vertex.2));
        let normal_c = Vec3::cross(&(self.vertex.1 - self.vertex.0), &(p - self.vertex.0));

        let bary = Vec3(
            Vec3::dot(&normal, &normal_a),
            Vec3::dot(&normal, &normal_b),
            Vec3::dot(&normal, &normal_c),
        ) / normal.length_squared();

        if bary.0 > 0.0 && bary.1 > 0.0 && bary.2 > 0.0 {
            Some(HitRecord::new(ray, 0.0, p, normal, self.material.clone()))
        } else {
            None
        }
    }

    fn bound(&self) -> BoundingBox {
        self.bounds
    }
}

pub struct Parallelogram {
    pub corner: Point,
    pub sides: (Vec3, Vec3),
    normal: Vec3,
    w: Vec3,
    pub material: Arc<dyn Material>,
    bounds: BoundingBox,
}

impl Parallelogram {
    pub fn new(corner: Point, sides: (Vec3, Vec3), material: Arc<dyn Material>) -> Self {
        let n = Vec3::cross(&sides.0, &sides.1);
        let normal = n.unit();
        // let d = Vec3::dot(&normal, &corner);
        let w = n / Vec3::dot(&n, &n);
        let diagonal_bound_1 = BoundingBox::from_points(corner, corner + sides.0 + sides.1);
        let diagonal_bound_2 = BoundingBox::from_points(corner + sides.0, corner + sides.1);
        let bounds = BoundingBox::from_boxes(diagonal_bound_1, diagonal_bound_2);
        Self {
            corner,
            sides,
            normal,
            w,
            material,
            bounds,
        }
    }
    pub fn q(&self) -> Vec3 {
        self.corner
    }
    pub fn u(&self) -> Vec3 {
        self.sides.0
    }
    pub fn v(&self) -> Vec3 {
        self.sides.1
    }

    pub fn is_interior(alpha: f64, beta: f64) -> Option<(f64, f64)> {
        if !Interval::new(0., 1.).contains(alpha) || !Interval::new(0., 1.).contains(beta) {
            None
        } else {
            Some((alpha, beta))
        }
    }
}

impl Hittable for Parallelogram {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<HitRecord> {
        if let Some(record) = Plane::new(self.corner, self.normal).hit(ray, t_range) {
            let p = record.point - self.corner;
            let alpha = Vec3::dot(&self.w, &Vec3::cross(&p, &self.sides.1));
            let beta = Vec3::dot(&self.w, &Vec3::cross(&self.sides.0, &p));
            if let Some((u, v)) = Self::is_interior(alpha, beta) {
                Some(
                    HitRecord::new(
                        ray,
                        record.t,
                        record.point,
                        self.normal,
                        self.material.clone(),
                    )
                    .set_uv(u, v),
                )
            } else {
                None
            }
        } else {
            None
        }
    }
    fn bound(&self) -> BoundingBox {
        self.bounds
    }
}

pub fn parallelepiped(a: Point, b: Point, material: Arc<dyn Material>) -> Arc<HittableList> {
    let mut sides = HittableList::new();

    let min = Vec3(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Vec3(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3(0.0, 0.0, max.z() - min.z());

    sides.add(Parallelogram::new(
        point(min.x(), min.y(), max.z()),
        (dx, dy),
        material.clone(),
    ));
    sides.add(Parallelogram::new(
        point(max.x(), min.y(), max.z()),
        (-dz, dy),
        material.clone(),
    ));
    sides.add(Parallelogram::new(
        point(max.x(), min.y(), min.z()),
        (-dx, dy),
        material.clone(),
    ));
    sides.add(Parallelogram::new(
        point(min.x(), min.y(), min.z()),
        (dz, dy),
        material.clone(),
    ));
    sides.add(Parallelogram::new(
        point(min.x(), max.y(), max.z()),
        (dx, -dz),
        material.clone(),
    ));
    sides.add(Parallelogram::new(
        point(min.x(), min.y(), min.z()),
        (dx, dz),
        material.clone(),
    ));

    Arc::new(sides)
}

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<HitRecord> {
        let d = Vec3::dot(&self.point, &self.normal);

        let denominator = Vec3::dot(&ray.direction, &self.normal);
        if denominator.abs() < 1e-8 {
            return None;
        }
        let t = (d - Vec3::dot(&ray.origin, &self.normal)) / denominator;
        if !t_range.contains(t) {
            return None;
        }
        Some(HitRecord::new(
            ray,
            t,
            ray.at(t),
            self.normal,
            Arc::new(Invisible),
        ))
    }
    fn bound(&self) -> BoundingBox {
        BoundingBox::empty()
    }
}

pub enum Planar {
    Triangle(Triangle),
    Parallelogram(Parallelogram),
}

impl Hittable for Planar {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<HitRecord> {
        let (point, normal, material) = match self {
            Planar::Triangle(triangle) => (
                triangle.vertex.0,
                triangle.normal,
                triangle.material.clone(),
            ),
            Planar::Parallelogram(parallelogram) => (
                parallelogram.corner,
                parallelogram.normal,
                parallelogram.material.clone(),
            ),
        };
        if let Some(record) = Plane::new(point, normal).hit(ray, t_range) {
            let p = record.point - point;
            let w = normal / Vec3::dot(&normal, &normal);
            if let Some((u, v)) = match self {
                Planar::Triangle(triangle) => {
                    let u = triangle.vertex.1 - triangle.vertex.0;
                    let v = triangle.vertex.2 - triangle.vertex.0;
                    let alpha = Vec3::dot(&w, &Vec3::cross(&p, &v));
                    let beta = Vec3::dot(&w, &Vec3::cross(&u, &p));
                    Triangle::is_interior(alpha, beta)
                }
                Planar::Parallelogram(quad) => {
                    let alpha = Vec3::dot(&quad.w, &Vec3::cross(&p, &quad.sides.1));
                    let beta = Vec3::dot(&quad.w, &Vec3::cross(&quad.sides.0, &p));
                    Parallelogram::is_interior(alpha, beta)
                }
            } {
                Some(HitRecord::new(ray, record.t, record.point, normal, material).set_uv(u, v))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bound(&self) -> BoundingBox {
        match self {
            Planar::Triangle(triangle) => triangle.bound(),
            Planar::Parallelogram(quad) => quad.bound(),
        }
    }
}
