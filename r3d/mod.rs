#![macro_escape]
use std::c_str;
use libc::{c_char,c_int,c_void};

pub use self::vectypes::*;
pub use self::vecmath::*;
pub use self::matrix::*;
pub use self::formats::*;	
pub use self::macros::*;
pub use self::debugdraw::*;
pub use self::half::*;
pub use self::render::*;
pub use self::ut::*;
pub use self::gl::*;
pub use self::glut::*;
pub use self::common::*;
pub use self::debugdraw::*;
pub use self::shaders::*;
pub use self::geom::*;
pub use self::array3d::*;

pub mod macros;
pub mod to;
pub mod vectypes;
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
pub mod debugdraw;
pub mod half;
pub mod render;
pub mod common;
pub mod shaders;
pub mod vertex;
pub mod geom;

pub trait Render {
	fn render(&self);
}

pub enum WinEvent {
	KeyDown(int),
	KeyUp(int),
	MouseMove((int,int)),
	MouseDown((int,int),int),
	MouseUp((int,int),int),
	WinResize(int,int),
	WinMove(int,int)
}
pub enum NextScreen<APP> {
	Continue,
	Push(Box<Screen<APP>>),
	Replace(Box<Screen<APP>>),
	Pop
}
 
pub trait Screen<APP> {
	fn display_create(&mut self){}
	fn render(&self,app:&mut APP)	{}
	fn update(&mut self,app:&mut APP)->NextScreen<APP>	{Continue}
	fn event(&mut self, ev:WinEvent,&mut APP){}
}

