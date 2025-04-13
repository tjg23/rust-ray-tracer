pub mod camera;
pub mod core;
pub mod models;
pub mod surfaces;

pub mod scenes;

pub use camera::*;
pub use core::*;
pub use models::*;
pub use surfaces::*;

use std::panic;

fn main() {
    match 8 {
        0 => scenes::material_spheres(),
        1 => scenes::checkered_spheres(),
        2 => scenes::earthmap(),
        3 => scenes::quads(),
        4 => scenes::planars(),
        5 => scenes::obj_mesh(),
        6 => scenes::simple_light(),
        7 => scenes::cornell_box(),
        8 => scenes::cornell_smoke(),
        _ => panic!("Invalid scene number"),
    }
}
