//pub use r3d::landscape::*;
//pub use r3d::mesh::*;

#![feature(globs)]
#![feature(macro_rules)]
#![allow(unused_attribute)]
#![feature(default_type_params)]
#![allow(unused_imports)]
#![allow(unused_variable)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_unsafe)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![macro_escape]

extern crate debug;
extern crate libc;
extern crate collections;

use shadertest::*;
use r3d::*;
use rw=rustwin;

pub mod r3d;
pub mod shadertest;
pub mod bsp;
pub mod bsprender;
pub mod rustwin;

// framework can be: Android, Glut, (iOS,.o.)
// 'app_*' are hooks called from platform specific stub
// these entrypoints implement screenflow system

#[cfg(target_os = "android")]
extern { fn android_log_print(lvl:c_int,  s:*const c_char);}

mod common {
	pub use std::vec;
	pub use libc::{c_int,c_char};
	pub use r3d::vecmath::*;
	pub use r3d::gl::*;
	pub use r3d::glut::*;
}



#[cfg(target_os = "android")]
fn log_print(level:int, s:&str) {
	unsafe {
		::android_log_print(level as c_int, c_str(s));
	}
}

#[cfg(not(target_os = "android"))]
fn log_print(level:int, s:&str) {
	std::io::println(s);
}

// (setq mouse-autoselect-window t)
static MAX_TOUCH_POINTERS:u32=12;

// TODO: These can be renamed away from android
// Its' really to be a superset of Android,iOS,Windows8,game-consoles,PC
struct AndroidInputSub {
	pointers:[((f32,f32,f32),u32),..12],
	accelerometer:(f32,f32,f32),
}

struct AndroidInput {
	// TODO: gamepad, keyboard... they do exist
	curr:AndroidInputSub,
	prev:AndroidInputSub
}
extern { fn android_get_inputs()->AndroidInput; }

// Desktop glut main loop, uses the app_create/display_create/render/display_destroy/destroy hooks; on android (iOS..) these same functions are just called by platform specific app loops
// It might be nice to make a rust trait object for all this, 
// however this is language independant. One can glue any other framework specifics ontop.

extern "C" fn null_func() {
}


#[cfg(not(target_os = "android"))]
pub fn main()
{

	unsafe {
		let mut argc:c_int=0;
		glutInit((&mut argc) as *mut c_int,0 as *const *const c_char );

		glutInitDisplayMode(GLUT_DEPTH | GLUT_DOUBLE | GLUT_RGBA);
		glutInitWindowSize(1280,800);
		let win=verify!(glutCreateWindow(c_str("Rust ShaderTest")) isnt 0);

		let mut app = app_create(0,0 as *const *const c_char,1280,800);
		app_display_create(&mut *app);
		glDrawBuffer(GL_BACK);

		glutIdleFunc(null_func as *const u8);
        glutDisplayFunc(null_func as *const u8); // osx impl requires some callback, even though we render manually here. for cleaner window handling, we should ensure this shows the backbuffer without redraw?
		glutReshapeWindow(1024,1024);
		glEnable(GL_DEPTH_TEST);
		dump!(argc);



		loop {
			glutMainLoopEvent();
			app_render(&mut *app);
			glFlush();
			glutSwapBuffers();
		}
		app_display_destroy(&mut *app);
	}
}


/// render a load of meshes in a lissajous curve
#[no_mangle]
pub extern "C" fn	app_render(s:&mut AppScreens) 
{
	let mut screens=&mut s.screens;
	screens.last().unwrap().render(&mut s.app);

	//todo:fps timer	
	let next_screen={
		screens.mut_last().unwrap().update(&mut s.app)
	};
	unsafe {
		match next_screen {
			Pop=>{screens.pop();},
			Push(x)=>{screens.push(x);},
			Replace(x)=>{screens.pop();screens.push(x);},
			_=>{}
		}
	}
}

pub static mut g_resources_init:bool=false;

struct AppScreens{
	app:App,
	screens:Vec<Box<Screen<App>>>
}

#[no_mangle]
pub extern "C" fn app_display_create(s:&mut AppScreens) {
	s.app.display_create();
	for  screen in s.screens.mut_iter(){
		screen.display_create();
	}
}
#[no_mangle]
pub extern "C" fn app_display_destroy(_:&mut AppScreens) {
	unsafe {
		g_resources_init=false;
	}
}

#[no_mangle]
pub extern "C" fn app_destroy(_:Box<AppScreens>) {
}


#[no_mangle]
pub extern "C" fn app_create(argc:c_int, argv:*const *const c_char, w:c_int,h:c_int)->Box<AppScreens> {
	box AppScreens{
		app:shadertest::App::new(),
		screens:vec![box shadertest::ShaderTest as Box<Screen<App>>]
	}
}


pub fn render_clear()
{
	unsafe {
//		glClearColor(g_fog_color.x()+(g_angle*2.0).sin(),g_fog_color.y(),g_fog_color.z(),g_fog_color.w());
		glClearColor(0.5f32,0.5f32,0.5f32,1.0f32);

		glClearDepthf(1.0f32);
		glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);
		glEnable(GL_DEPTH_TEST);
		glDepthMask(GL_TRUE);
		glDepthFunc(GL_LEQUAL);

		glEnable(GL_CULL_FACE);
	}
}









