use macroquad::{prelude::ImageFormat, texture::Image};

use crate::{color, Color, Interval, Point};

use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    pub inv_scale: f64,
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            odd,
            even,
        }
    }
    pub fn from(scale: f64, odd: Color, even: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(odd)),
            Arc::new(SolidColor::new(even)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let (x, y, z) = (
            (self.inv_scale * p.x()).floor() as i32,
            (self.inv_scale * p.y()).floor() as i32,
            (self.inv_scale * p.z()).floor() as i32,
        );
        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ColorTexture {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl ColorTexture {
    pub fn new(data: Vec<Color>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }
    pub fn from_image(image: Image) -> Self {
        Self {
            width: image.width as usize,
            height: image.height as usize,
            data: image
                .get_image_data()
                .iter()
                .map(|&c| color(c[0] as f64 / 255., c[1] as f64 / 255., c[2] as f64 / 255.))
                .collect(),
        }
    }
    pub fn from_file(file: &[u8], format: Option<ImageFormat>) -> Self {
        let image = Image::from_file_with_format(file, format).unwrap();
        Self::from_image(image)
    }
}

impl Texture for ColorTexture {
    fn value(&self, u: f64, v: f64, _p: &Point) -> Color {
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1. - Interval::new(0., 1.).clamp(v);
        let x = (u * self.width as f64) as usize;
        let y = (v * self.height as f64) as usize;
        self.data[y * self.width + x]
    }
}

pub struct ImageTexture {
    pub image: Image,
}

impl ImageTexture {
    pub fn new(image: Image) -> Self {
        Self { image }
    }
    pub fn from_file(file: &[u8], format: Option<ImageFormat>) -> Self {
        Self {
            image: Image::from_file_with_format(file, format).unwrap(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point) -> Color {
        let u = Interval::new(0., 1.).clamp(u);
        let v = 1. - Interval::new(0., 1.).clamp(v);
        let x = (u * self.image.width as f64)
            .min(self.image.width as f64 - 1.0)
            .max(0.0) as usize;
        let y = (v * self.image.height as f64)
            .min(self.image.height as f64 - 1.0)
            .max(0.0) as usize;
        let c = self.image.get_pixel(x as u32, y as u32);
        color(c.r as f64, c.g as f64, c.b as f64)
    }
}
