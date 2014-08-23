//pub use r3d::landscape::*;
//pub use r3d::mesh::*;

#![feature(globs)]
#![feature(import_shadowing)]
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

pub use shadertest::*;
pub use r3d::*;
pub use rustwin::*;

pub mod r3d;
pub mod shadertest;
pub mod bsp;
pub mod bsprender;
pub mod rustwin;
pub mod flymode;
pub mod everywhere;

// framework can be: Android, Glut, (iOS,.o.)
// 'app_*' are hooks called from platform specific stub
// these entrypoints implement screenflow system

#[cfg(target_os = "android")]
extern { fn android_log_print(lvl:c_int,  s:*const c_char);}
/*
mod common {

	pub use std::vec;
	pub use libc::{c_int,c_char};
	pub use r3d::vecmath::*;
	pub use r3d::gl::*;
	pub use r3d::glut::*;
}
*/


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


// rustic mainloop possible on desktop OS.
// this is replaced with C/objC stub on android/iOS
#[cfg(not(target_os = "android"))]
pub fn main()
{

	unsafe {
		let mut argc:c_int=0;
		render_glut_init(argc,0 as *const *const c_char);


		let mut app = app_create(0,0 as *const *const c_char,1280,800);
		app_display_create(&mut *app);

// osx impl requires some callback, even though we render manually here. for cleaner window handling, we should ensure this shows the backbuffer without redraw?
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
	if rustwin::get_key_state('z')!=0 {
		dump!(rustwin::get_key_state('z'));
	}
	let mut screens=&mut s.screens;
	screens.back().unwrap().render();

	while {
		let ev=get_event();
		match ev {
			EventNone=>false,
			_ => {let chg=screens.back_mut().unwrap().win_event(ev); change_screen(screens,chg); true},
		}
	} {}

	//todo:fps timer	
	let next_screen={
		screens.back_mut().unwrap().update()
	};

	change_screen(screens,next_screen);
}
fn change_screen<S:Deque<Box<Screen>>>(screens:&mut S,ns:ScreenChange) {
	unsafe {
		match ns {
			ScContinue=>{},
			_=>{
				screens.back_mut().unwrap().on_deselect();
				match ns {
					ScRoot(x)=>{ while screens.len()>0 {
							screens.back_mut().unwrap().on_deselect(); screens.pop_back();
						}
						screens.push_back(x);
					}
					ScPop=>{screens.pop_back(); },
					ScPush(x)=>{screens.push_back(x);},
					ScReplace(x)=>{screens.pop_back();screens.push_back(x);},
					ScCycleNext=>{let x=screens.pop_front().unwrap(); screens.push_back(x);}
					ScCyclePrev=>{let x=screens.pop_back().unwrap(); screens.push_front(x);}
					_=>{}
				}
				screens.back_mut().unwrap().on_select();
			}
		}
	}
}

pub static mut g_resources_init:bool=false;

pub struct AppScreens{
	screens:RingBuf<Box<Screen>>
}

#[no_mangle]
pub extern "C" fn app_display_create(s:&mut AppScreens) {
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
#[cfg(not(target_os = "android"))]
pub extern "C" fn app_create(argc:c_int, argv:*const *const c_char, w:c_int,h:c_int)->Box<AppScreens> {
	box AppScreens{
		screens:{
			let mut x=RingBuf::new();
			
			x.push_back(box flymode::FlyMode::new() as Box<Screen>);
			x.push_back(box shadertest::ShaderTest::new() as Box<Screen>); 
			x
		}
	}
}

#[no_mangle]
#[cfg(target_os = "android")]
pub extern "C" fn app_create(argc:c_int, argv:*const *const c_char, w:c_int,h:c_int)->Box<AppScreens> {
	box AppScreens{
		screens:{
			let mut x=RingBuf::new();
			x.push_back(box shadertest::ShaderTest::new() as Box<Screen>); 
			x
		}
	}
}










