use std::{path::Path, sync::Arc};

use crate::{camera::*, core::*, models::*, surfaces::*};

use macroquad::prelude::ImageFormat;
use serde::Deserialize;
use three_d_asset::Geometry;

#[derive(Deserialize)]
pub struct CameraBuilder {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub vfov: f64,
    pub look_from: Point,
    pub look_at: Point,
    pub up: Vec3,
    pub aa_samples: i32,
    pub max_depth: i32,
}
impl CameraBuilder {
    pub fn build(&self) -> Camera {
        Camera::new(
            self.aspect_ratio,
            self.image_width,
            self.vfov,
            self.look_from,
            self.look_at,
            self.up,
            self.aa_samples,
            self.max_depth,
        )
    }
}

pub fn material_spheres() {
    /* === World === */
    let mut world = HittableList::new();

    /* === Materials === */
    let material_ground = Arc::new(Lambertian::from(color(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::from(color(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Arc::new(Metal::new(color(0.8, 0.6, 0.2), 1.0));

    /* === Objects === */
    world.add_arc(Arc::new(Sphere::new(
        point(0., -100.5, -1.),
        100.,
        material_ground,
    )));

    world.add_arc(Arc::new(Sphere::new(
        point(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add_arc(Arc::new(Sphere::new(
        point(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add_arc(Arc::new(Sphere::new(
        point(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add_arc(Arc::new(Sphere::new(
        point(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let world = HittableList::from(Arc::new(BoundNode::from_list(world)));

    Camera::new(
        16.0 / 9.0,
        400,
        90.0,
        point(0.0, 0.0, 0.0),
        point(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        20,
        20,
    )
    .render(&world);
}

pub fn checkered_spheres() {
    /* === World === */
    let mut world = HittableList::new();

    /* === Materials === */
    let checker = Arc::new(CheckerTexture::from(
        0.32,
        color(0.2, 0.3, 0.1),
        color(0.9, 0.9, 0.9),
    ));

    /* === Objects === */
    world.add_arc(Arc::new(Sphere::new(
        point(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    world.add_arc(Arc::new(Sphere::new(
        point(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));

    Camera::new(
        16.0 / 9.0,
        400,
        20.0,
        point(13.0, 2.0, 3.0),
        point(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20,
        20,
    )
    .render(&world);
}

pub fn earthmap() {
    /* === World === */
    let mut world = HittableList::new();

    /* === Materials === */
    let earthmap = Arc::new(ColorTexture::from_file(
        include_bytes!("../resources/earthmap.png"),
        Some(ImageFormat::Png),
    ));

    /* === Objects === */
    world.add_arc(Arc::new(Sphere::new(
        point(0.0, 0.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(earthmap.clone())),
    )));

    Camera::new(
        16.0 / 9.0,
        400,
        20.0,
        point(0., 0., 12.),
        point(0., 0., 0.),
        Vec3(0.0, 1.0, 0.0),
        20,
        20,
    )
    .render(&world);
}

pub fn quads() {
    /* === World === */
    let mut world = HittableList::new();

    /* === Materials === */
    let left_red = Arc::new(Lambertian::from(color(1., 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from(color(0.2, 1., 0.2)));
    let right_blue = Arc::new(Lambertian::from(color(0.2, 0.2, 1.)));
    let top_orange = Arc::new(Lambertian::from(color(1., 0.5, 0.)));
    let bottom_teal = Arc::new(Lambertian::from(color(0.2, 0.8, 0.8)));

    /* === Objects ===  */
    world.add_arc(Arc::new(Parallelogram::new(
        point(-3., -2., 5.),
        (Vec3(0., 0., -4.), Vec3(0., 4., 0.)),
        left_red,
    )));
    world.add_arc(Arc::new(Parallelogram::new(
        point(-2., -2., 0.),
        (Vec3(4., 0., 0.), Vec3(0., 4., 0.)),
        back_green,
    )));
    world.add_arc(Arc::new(Parallelogram::new(
        point(3., -2., 1.),
        (Vec3(0., 0., 4.), Vec3(0., 4., 0.)),
        right_blue,
    )));
    world.add_arc(Arc::new(Parallelogram::new(
        point(-2., 3., 1.),
        (Vec3(4., 0., 0.), Vec3(0., 0., 4.)),
        top_orange,
    )));
    world.add_arc(Arc::new(Parallelogram::new(
        point(-2., -3., 5.),
        (Vec3(4., 0., 0.), Vec3(0., 0., -4.)),
        bottom_teal,
    )));

    Camera::new(
        1.0,
        400,
        80.,
        point(0., 0., 9.),
        point(0., 0., 0.),
        Vec3(0., 1., 0.),
        20,
        20,
    )
    .render(&world);
}

pub fn planars() {
    /* === World === */
    let mut world = HittableList::new();

    /* === Materials === */
    let left_red = Arc::new(Lambertian::from(color(1., 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from(color(0.2, 1., 0.2)));
    let right_blue = Arc::new(Lambertian::from(color(0.2, 0.2, 1.)));
    let top_orange = Arc::new(Lambertian::from(color(1., 0.5, 0.)));
    let bottom_teal = Arc::new(Lambertian::from(color(0.2, 0.8, 0.8)));

    /* === Objects ===  */
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(-2., -2., 0.),
        (Vec3(4., 0., 0.), Vec3(0., 4., 0.)),
        back_green,
    )));
    world.add(Planar::Triangle(Triangle::new(
        (point(-3., -2., 1.), point(-3., -2., 5.), point(-3., 2., 1.)),
        left_red,
    )));
    world.add(Planar::Triangle(Triangle::new(
        (point(3., -2., 1.), point(3., -2., 5.), point(3., 2., 5.)),
        right_blue,
    )));
    world.add(Planar::Triangle(Triangle::new(
        (point(-2., 3., 1.), point(2., 3., 1.), point(0., 3., 5.)),
        top_orange,
    )));
    world.add(Planar::Triangle(Triangle::new(
        (point(-2., -3., 1.), point(2., -3., 1.), point(0., -3., 5.)),
        bottom_teal,
    )));

    Camera::new(
        1.0,
        400,
        80.,
        point(0., 0., 9.),
        point(0., 0., 0.),
        Vec3(0., 1., 0.),
        20,
        20,
    )
    .render(&world);
}

pub fn obj_mesh() {
    let mut world = HittableList::new();

    let material = Arc::new(Lambertian::from(color(0.8, 0.8, 0.8)));

    let model: three_d_asset::Model = three_d_asset::io::load_and_deserialize(Path::new(
        "./resources/SpaceShip-Fighter/SpaceShip-Fighter.obj",
    ))
    .unwrap();

    let mesh = match &model.geometries[0].geometry {
        Geometry::Points(_) => panic!("Expected a triangle mesh"),
        Geometry::Triangles(mesh) => mesh,
    };
    mesh.for_each_triangle(|a, b, c| {
        let va = mesh.positions.to_f64()[a];
        let vb = mesh.positions.to_f64()[b];
        let vc = mesh.positions.to_f64()[c];
        world.add(Planar::Triangle(Triangle::new(
            (
                point(va.x, va.y, va.z),
                point(vb.x, vb.y, vb.z),
                point(vc.x, vc.y, vc.z),
            ),
            material.clone(),
        )));
    });

    let world = HittableList::from(Arc::new(BoundNode::from_list(world)));

    Camera::new(
        1.0,
        400,
        80.,
        point(0., 0., 9.),
        point(0., 0., 0.),
        Vec3(0., 1., 0.),
        20,
        20,
    )
    .render(&world);
}

pub fn simple_light() {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::from(color(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::from(color(0.1, 0.2, 0.5)));
    let diffuse_light = Arc::new(DiffuseLight::from(color(4., 4., 4.)));

    world.add(Sphere::new(point(0., -1000., 0.), 1000., material_ground));
    world.add(Sphere::new(point(0., 2., 0.), 2., material_center));
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(3., 1., -2.),
        (Vec3(2., 0., 0.), Vec3(0., 2., 0.)),
        diffuse_light,
    )));

    Camera::new(
        16.0 / 9.0,
        400,
        20.,
        point(26., 3., 6.),
        point(0., 2., 0.),
        Vec3(0., 1., 0.),
        20,
        20,
    )
    .render(&world);
}

pub fn cornell_box() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from(color(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(color(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(color(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from(color(15., 15., 15.)));

    world.add(Planar::Parallelogram(Parallelogram::new(
        point(555., 0., 0.),
        (Vec3(0., 555., 0.), Vec3(0., 0., 555.)),
        green.clone(),
    )));
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(0., 0., 0.),
        (Vec3(555., 0., 0.), Vec3(0., 0., 555.)),
        red.clone(),
    )));
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(343., 554., 332.),
        (Vec3(-130., 0., 0.), Vec3(0., 0., -105.)),
        light.clone(),
    )));
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(0., 0., 0.),
        (Vec3(555., 0., 0.), Vec3(0., 0., 555.)),
        white.clone(),
    )));
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(555., 555., 555.),
        (Vec3(555., 0., 0.), Vec3(0., 0., 555.)),
        white.clone(),
    )));
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(0., 0., 555.),
        (Vec3(555., 0., 0.), Vec3(0., 555., 0.)),
        white.clone(),
    )));

    let box1 = parallelepiped(Vec3(0., 0., 0.), Vec3(165., 330., 165.), white.clone());
    let box1 = Arc::new(RotateY::new(box1, 15.));
    let box1 = Arc::new(Translation::new(box1, Vec3(265., 0., 295.)));
    world.add_arc(box1);

    let box2 = parallelepiped(Vec3(0., 0., 0.), Vec3(165., 165., 165.), white.clone());
    let box2 = Arc::new(RotateY::new(box2, -18.));
    let box2 = Arc::new(Translation::new(box2, Vec3(130., 0., 65.)));
    world.add_arc(box2);

    Camera::new(
        1.0,
        600,
        40.0,
        point(278., 278., -800.),
        point(278., 278., 0.),
        Vec3(0., 1., 0.),
        50,
        20,
    )
    .render(&world);
}

pub fn cornell_smoke() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from(color(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(color(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(color(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from(color(20., 20., 20.)));

    world.add(Planar::Parallelogram(Parallelogram::new(
        point(555., 0., 0.),
        (Vec3(0., 555., 0.), Vec3(0., 0., 555.)),
        green.clone(),
    )));
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(0., 0., 0.),
        (Vec3(555., 0., 0.), Vec3(0., 0., 555.)),
        red.clone(),
    )));
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(343., 554., 332.),
        (Vec3(-130., 0., 0.), Vec3(0., 0., -105.)),
        light.clone(),
    )));
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(0., 0., 0.),
        (Vec3(555., 0., 0.), Vec3(0., 0., 555.)),
        white.clone(),
    )));
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(555., 555., 555.),
        (Vec3(555., 0., 0.), Vec3(0., 0., 555.)),
        white.clone(),
    )));
    world.add(Planar::Parallelogram(Parallelogram::new(
        point(0., 0., 555.),
        (Vec3(555., 0., 0.), Vec3(0., 555., 0.)),
        white.clone(),
    )));

    let box1 = parallelepiped(Vec3(0., 0., 0.), Vec3(165., 330., 165.), white.clone());
    let box1 = Arc::new(RotateY::new(box1, 15.));
    let box1 = Arc::new(Translation::new(box1, Vec3(265., 0., 295.)));

    let box2 = parallelepiped(Vec3(0., 0., 0.), Vec3(165., 165., 165.), white.clone());
    let box2 = Arc::new(RotateY::new(box2, -18.));
    let box2 = Arc::new(Translation::new(box2, Vec3(130., 0., 65.)));

    world.add(ConstantMedium::from_color(box1, 0.01, color(0., 0., 0.)));
    world.add(ConstantMedium::from_color(box2, 0.01, color(1., 1., 1.)));

    Camera::new(
        1.0,
        900,
        40.0,
        point(278., 278., -800.),
        point(278., 278., 0.),
        Vec3(0., 1., 0.),
        150,
        75,
    )
    .render(&world);
}
