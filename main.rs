//pub use r3d::landscape::*;
//pub use r3d::mesh::*;

#[feature(globs)];
#[feature(macro_rules)];
#[feature(default_type_params)];
#[allow(unused_imports)];
#[allow(unused_variable)];
#[allow(dead_code)];
#[allow(unreachable_code)];
#[allow(unused_unsafe)];
#[allow(unused_mut)];
#[allow(non_camel_case_types)];
#[macro_escape];


extern crate collections;
pub use std::str::raw::*;
pub use r3d::vecmath::*;
pub use std::vec;
pub use std::libc;
pub use std::libc::{c_int,c_char};
use shadertest::c_str;
use std::io;

mod macros;	// must preceed others for macro visibility.
mod r3d;
mod shadertest;



#[cfg(target_os = "android")]
extern { fn android_log_print(lvl:c_int,  s:*c_char);}

#[cfg(target_os = "android")]
fn log_print(level:int, s:&str) {
	unsafe {
		android_log_print(level as c_int, c_str(s));
	}
}

#[cfg(not(target_os = "android"))]
fn log_print(level:int, s:&str) {
	io::println(s);
}

 
#[cfg(not(target_os = "android"))]
fn main() {
	shadertest::shadertest_main();
}

//////////////////////////////////////////////////
// android hooks
// These functions are statically linked by the
// modified sample code main loop giving various entry points
// for rust
// no rendering is done there, just surface creation and swap.


#[no_mangle]
#[cfg(target_os = "android")]
pub extern fn   rust_android_init_app() {
	logi!("init app");
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern fn   rust_android_init_display() {
	logi!("init display");
	shadertest::create_resources();
}
#[no_mangle]
pub extern fn   rust_android_term_display() {
	logi!("terminate display");
}

// todo - call to recreate resources. lazy init isn't the right way! it must always init when called.

static MAX_TOUCH_POINTERS:u32=12;

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


#[cfg(target_os = "android")]
#[no_mangle]
pub extern fn   rust_android_render() {

	// Struct holding accumulated input state
	let inp=unsafe {android_get_inputs()};

//	logi!("input={:?}",inp);

	shadertest::render_no_swap();
}

#[no_mangle]
pub extern fn   rust_android_term_app() {
	logi!("terminate app");
}










