#![macro_escape]
#![feature(import_shadowing)]

pub use self::macros::*;
pub use self::to::*;
pub use self::vecmath::*;
pub use self::matrix::*;
pub use self::formats::*;
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
//pub use rustwin::WinEvent;

pub mod macros;
pub mod to;
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
pub mod shaders;
pub mod vertex;
pub mod geom;

/// External context brought in by all submodules
/// created to avoid needing circular super::*
mod common {
	pub use libc::{c_void,c_char,c_int};
	pub use std::{num,vec,mem,cmp,io,ptr,c_str};
	pub use std::num::*;
	pub use std::f32::*;
	pub use std::str::raw::*;
	pub use std::ops::Deref;
	pub use std::c_str::CString;
	pub use std::intrinsics::{size_of,offset};
	pub use std::collections::{RingBuf,Deque,HashMap,HashSet,DList};

}




pub trait Render {
	fn render(&self);
}



