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
#[macro_escape]

extern crate collections;
pub use std::str::raw::*;
pub use r3d::vecmath::*;
pub use std::vec;
pub use std::libc;
pub use std::libc::{c_int,c_char};
use shadertest::c_str;
//use camera;

//mod everywhere;
mod r3d;
mod shadertest;
//mod testcommon;


#[link(name = "GLU")]
#[link(name = "Xext")]
#[link(name = "glut")]
#[link(name = "GL")]
#[link(name = "stdc++")]

#[cfg(target_os = "android")]
extern { fn android_log_print(lvl:c_int,  s:*c_char);}

#[cfg(not(target_os = "android"))]
#[no_mangle]
extern fn android_log_print(lvl:c_int,  s:*c_char) {
	::std::io::println(s.to_str());
}
fn log_print(level:int, s:&str) {
	unsafe {
		android_log_print(level as c_int, c_str(s));
	}
}
macro_rules! logi{
	($($arg:tt)*)=>( ::log_print(5, format!("{:s}:{:u}: ",file!(),line!())+format!($($arg)*)))
}
 
#[cfg(not(target_os = "android"))]
fn main() {
	shadertest::shadertest_main();
}

//////////////////////////////////////////////////
// android hooks


#[fixed_stack_segment]


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

#[cfg(target_os = "android")]
#[no_mangle]
pub extern fn   rust_android_render() {
	
	shadertest::render_no_swap();
}

#[no_mangle]
pub extern fn   rust_android_term_app() {
	logi!("terminate app");
}










