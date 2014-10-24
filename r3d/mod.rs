#![macro_escape]
#![feature(import_shadowing)]

pub use std::c_str;
pub use libc::types::os::arch::c95::{c_char,c_int};
pub use libc::types::common::c95::{c_void};
pub use std::collections::RingBuf;
pub use std::collections::Deque;

pub use self::macros::*;
pub use self::to::*;
pub use self::vectypes::*;
pub use self::vecmath::{Vector,Vec2,Vec3,Vec4,Pos,ToVec2,ToVec3,ToVec4};
pub use self::matrix::{Matrix,Matrix34,Matrix44,Matrix43,LookAt};
pub use self::formats::{Pack,UnPack,UnPack16};	
pub use self::debugdraw::*;
pub use self::ut::*;
pub use self::gl::*;
pub use self::glut::*;
pub use self::common::*;
pub use self::shaders::*;
pub use self::geom::*;
pub use self::array3d::*;
pub use self::half::*;
pub use self::render::*;
pub use std::num::*;
//pub use rustwin::WinEvent;

pub mod macros;
pub mod to;
pub mod vectypes;
pub mod vecmath;
pub mod matrix;
pub mod formats;
pub mod debugdraw;
pub mod gl;
pub mod glut;
pub mod gl_constants;
pub mod gl_h_consts;
pub mod glut_h_consts;
pub mod ut;
pub mod array3d;
pub mod half;
pub mod render;
pub mod common;
pub mod shaders;
pub mod vertex;
pub mod geom;

pub trait Render {
	fn render(&self);
}



