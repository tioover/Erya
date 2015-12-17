extern crate nalgebra;
extern crate time;
extern crate rand;
extern crate image;
extern crate num;
#[macro_use]
extern crate glium;


#[macro_export]
macro_rules! na {
    ($x: expr) =>
        ($crate::na::Vec1::new($x));
    ($x: expr, $y: expr) =>
        ($crate::na::Vec2::new($x, $y));
    ($x: expr, $y: expr, $z: expr) =>
        ($crate::na::Vec3::new($x, $y, $z));
    ($x: expr, $y: expr, $z: expr, $w: expr) =>
        ($crate::na::Vec4::new($x, $y, $z, $w));
}


macro_rules! from {
    ($x: expr) => ($crate::num::NumCast::from($x).unwrap())
}


pub use nalgebra as na;
pub use glium::Display;


pub mod renderer;
pub mod mesh;
pub mod resource;
pub mod texture;
pub mod id;
pub mod math;
pub mod rect;
pub mod timer;
pub mod camera;
pub mod transform;
pub mod sprite;
pub mod shader;
pub mod utils;


use std::string::ToString;
use glium::glutin::WindowBuilder;
use glium::DisplayBuild;


pub fn build_display<T>(title: T, (width, height): (u32, u32)) -> Display
    where T: ToString
{
    WindowBuilder::new()
        .with_title(title.to_string())
        .with_dimensions(width, height)
        .with_vsync()
        .build_glium()
        .unwrap()
}
