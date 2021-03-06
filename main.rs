//pub use r3d::landscape::*;
//pub use r3d::mesh::*;

#![feature(globs)]
#![feature(import_shadowing)]
#![feature(macro_rules)]
#![feature(tuple_indexing)]
#![feature(associated_types)]
#![allow(unused_attribute)]
#![feature(default_type_params)]
#![allow(unused_imports)]
#![allow(unused_variable)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_unsafe)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![macro_escape]
#![allow(improper_ctypes)]
#![allow(non_upper_case_globals)]

//extern crate debug;
extern crate libc;
extern crate collections;

use shadertest::ShaderTest;
use r3d::*;
use rustwin::*;
use common::*;

pub mod r3d;
pub mod shadertest;
pub mod bsp;
pub mod bsprender;
pub mod rustwin;
pub mod flymode;
pub mod voxel;

pub mod common {
	pub use r3d::*;
	pub use std::vec;
	pub use libc::{c_int,c_char};
	pub use r3d::vecmath::*;
	pub use r3d::{glut,gl};
	pub use r3d::gl::*;
	pub use r3d::glut::*;
}

// framework can be: Android, Glut, (iOS,.o.)
// 'app_*' are hooks called from platform specific stub
// these entrypoints implement screenflow system

#[cfg(target_os = "android")]
extern { fn android_log_print(lvl:c_int,  s:*const c_char);}
/*
mod common {

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

	let v = Vec3(1.0f32,2.0f32,3.0f32);
	dump!(v.0, v.1, v.2);
	dump!(v);

	let mut hm = HashMap::new();
	hm.insert("apple",2i);
	dump!(hm["apple"],hm["banana"]);

	unsafe {
		let mut argc:c_int=0;
		render_glut_init(argc,0 as *const *const c_char);


		let mut app = app_create(0,0 as *const *const c_char,1280,800);
		app_display_create(&mut *app);

// osx impl requires some callback, even though we render manually here. for clean1er window handling, we should ensure this shows the backbuffer without redraw?
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
	if get_key_state('z')!=0 {
		dump!(get_key_state('z'));
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
fn change_screen<S:Deque<Box<Screen+'static>>>(screens:&mut S,ns:ScreenChange) {
	unsafe {
		match ns {
			ScContinue=>{},
			_=>{
				screens.back_mut().unwrap().on_deselect();
				match ns {
					ScRoot(x)=>{ while screens.len()>0 {
							screens.back_mut().unwrap().on_deselect(); screens.pop();
						}
						screens.push(x);
					}
					ScPop=>{screens.pop(); },
					ScPush(x)=>{screens.push(x);},
					ScReplace(x)=>{screens.pop();screens.push(x);},
					ScCycleNext=>{let x=screens.pop_front().unwrap(); screens.push(x);}
					ScCyclePrev=>{let x=screens.pop().unwrap(); screens.push_front(x);}
					_=>{}
				}
				screens.back_mut().unwrap().on_select();
			}
		}
	}
}

trait Collide2<T,Y> {
	fn collide(&self,x:&T,y:&Y);
}
struct A; struct B;
impl Collide2<A,A> for A { fn collide(&self,x:&A,y:&A){println!("A-A-A")}}
impl Collide2<B,A> for B  { fn collide(&self,x:&B,y:&A){println!("B-B-A")}}
impl Collide2<A,B> for A  { fn collide(&self,x:&A,y:&B){println!("A-A-B")}}
impl Collide2<B,B> for B  { fn collide(&self,x:&B,y:&B){println!("B-B-B")}}
fn test() {
	let a=A;
	let b=B;
	a.collide(&a,&a);
	b.collide(&b,&a);
	a.collide(&a,&b);
	b.collide(&b,&b);
	a*b;
	b*a;
	//b.collide(&b);
}
impl Mul<B,()> for A{
	fn mul(&self,rhs:&B)->(){println!("A * B")}
}
impl Mul<A,()> for B{
	fn mul(&self,rhs:&A)->(){println!("B * A")}
}



pub static mut g_resources_init:bool=false;

pub struct AppScreens{
	screens:RingBuf<Box<Screen+'static>>
}

#[no_mangle]
pub extern "C" fn app_display_create(s:&mut AppScreens) {
test();
	for  screen in s.screens.iter_mut(){
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
			let mut x= RingBuf::new();
			
			x.push(box flymode::FlyMode::new() as Box<Screen>);
			x.push(box shadertest::ShaderTest::new() as Box<Screen>); 
			x
		}
	}
}

#[no_mangle]
#[cfg(target_os = "android")]
pub extern "C" fn app_create(argc:c_int, argv:*const *const c_char, w:c_int,h:c_int)->Box<AppScreens> {
	test();
	box AppScreens{
		screens:{
			let mut x=RingBuf::new();
			x.push(box shadertest::ShaderTest::new() as Box<Screen>); 
			x
		}
	}
}










