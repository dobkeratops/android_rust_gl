#![macro_escape]
use std::c_str;
use libc::c_char;

pub use self::vecmath::{Vec3,Vec4};
//pub use self::matrix::{Matrix3,Matrix4,SRT,identity};
pub use self::formats::{Pack,UnPack};	
pub use self::macros::*;

pub mod macros;
pub mod to;
pub mod vecmath;
pub mod matrix;
pub mod formats;
pub mod gl;
pub mod glut;
pub mod gl_constants;
pub mod gl_h_consts;
pub mod glut_h_consts;
pub mod ut;
pub mod array3d;


pub unsafe fn c_str(s:&str)->*const c_char {
	s.to_c_str().unwrap()
}
