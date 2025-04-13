use crate::Interval;

use rand::{random, thread_rng, Rng};
use serde::Deserialize;
use std::ops::{Add, AddAssign, Div, Index, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    /* == Dimensions == */
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    /* == Operations == */
    pub fn cross(v: &Vec3, w: &Vec3) -> Vec3 {
        Vec3(
            v.1 * w.2 - v.2 * w.1,
            v.2 * w.0 - v.0 * w.2,
            v.0 * w.1 - v.1 * w.0,
        )
    }
    pub fn dot(v: &Vec3, w: &Vec3) -> f64 {
        v.0 * w.0 + v.1 * w.1 + v.2 * w.2
    }
    pub fn sub(v: &Vec3, w: &Vec3) -> Vec3 {
        Vec3(v.0 - w.0, v.1 - w.1, v.2 - w.2)
    }
    pub fn add(v: &Vec3, w: &Vec3) -> Vec3 {
        Vec3(v.0 + w.0, v.1 + w.1, v.2 + w.2)
    }
    pub fn scale(v: &Vec3, s: f64) -> Vec3 {
        Vec3(v.0 * s, v.1 * s, v.2 * s)
    }

    /* -- Length -- */
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }
    /* -- Reflect & Refract -- */
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - *normal * 2.0 * Vec3::dot(self, normal)
    }
    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&-*self, normal).min(1.0);
        let r_out_perp = (*self + *normal * cos_theta) * etai_over_etat;
        let r_out_parallel = *normal * -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared()));
        r_out_perp + r_out_parallel
    }

    /* -- Debug -- */
    pub fn print(&self) {
        println!("{} {} {}", self.0, self.1, self.2);
    }

    /* -- Color -- */
    pub fn to_gamma(&self) -> Vec3 {
        let r = if self.0 > 0.0 { self.0.sqrt() } else { 0.0 };
        let g = if self.1 > 0.0 { self.1.sqrt() } else { 0.0 };
        let b = if self.2 > 0.0 { self.2.sqrt() } else { 0.0 };
        Vec3(r, g, b)
    }

    pub fn write_color(&self) {
        let intensity = Interval::new(0.0, 0.999);
        println!(
            "{} {} {}",
            (256.0 * intensity.clamp(self.0)) as i32,
            (256.0 * intensity.clamp(self.1)) as i32,
            (256.0 * intensity.clamp(self.2)) as i32,
        );
    }

    /* -- Random -- */
    pub fn random() -> Vec3 {
        Vec3(random(), random(), random())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = thread_rng();
        Vec3(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn sample_square() -> Vec3 {
        let mut rng = thread_rng();
        Vec3(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0)
    }

    pub fn random_unit() -> Vec3 {
        loop {
            let v = Vec3::random_range(-1.0, 1.0);
            let l = v.length_squared();
            if l < 1.0 && l > 1e-60 {
                return v / f64::sqrt(l);
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit();
        if Vec3::dot(&on_unit_sphere, &normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}
impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

pub type Point = Vec3;
pub type Color = Vec3;

pub fn point(x: f64, y: f64, z: f64) -> Point {
    Vec3(x, y, z)
}
pub fn color(r: f64, g: f64, b: f64) -> Color {
    Vec3(r, g, b)
}
