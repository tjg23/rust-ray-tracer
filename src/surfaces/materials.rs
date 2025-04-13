use std::sync::Arc;

use crate::{color, Color, HitRecord, Ray, SolidColor, Texture, Vec3};

pub trait Material {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        color(0., 0., 0.)
    }
}

pub struct Lambertian {
    pub texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
    pub fn from(albedo: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        let scattered = Ray {
            origin: hit.point,
            direction: scatter_direction,
        };
        let attenuation = self.texture.value(hit.u, hit.v, &hit.point);
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        if fuzz < 1.0 {
            Self { albedo, fuzz }
        } else {
            Self { albedo, fuzz: 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected =
            Vec3::reflect(&ray.direction, &hit.normal).unit() + Vec3::random_unit() * self.fuzz;
        let scattered = Ray {
            origin: hit.point,
            direction: reflected,
        };
        let attenuation = self.albedo;
        // if Vec3::dot(&scattered.direction, &hit.normal) > 0.0 {
        Some((scattered, attenuation))
        // } else {
        // 	None
        // }
    }
}

pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index
    // over the refractive index of the enclosing medium.
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = color(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = Vec3::dot(&-ray.direction, &hit.normal).min(1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random() {
            let reflected = Vec3::reflect(&ray.direction.unit(), &hit.normal);
            let scattered = Ray {
                origin: hit.point,
                direction: reflected,
            };
            Some((scattered, attenuation))
        } else {
            let refracted = Vec3::refract(&ray.direction.unit(), &hit.normal, refraction_ratio);
            let scattered = Ray {
                origin: hit.point,
                direction: refracted,
            };
            Some((scattered, attenuation))
        }
    }
}

pub struct Invisible;

impl Material for Invisible {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

pub struct DiffuseLight {
    pub texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
    pub fn from(color: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
        self.texture.value(u, v, p)
    }
}

pub struct Isotropic {
    pub texture: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
    pub fn from(color: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(color)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let scattered = Ray {
            origin: hit.point,
            direction: Vec3::random_unit(),
        };
        let attenuation = self.texture.value(hit.u, hit.v, &hit.point);
        Some((scattered, attenuation))
    }
}
