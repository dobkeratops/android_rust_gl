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

extern crate collections;
pub use std::str::raw::*;
pub use r3d::vecmath::*;
pub use std::vec;
pub use std::libc;
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

 
fn main() {
	shadertest::shadertest_main();
}

//////////////////////////////////////////////////
// android hooks

static mut g_rusty_value:int=100000;
#[fixed_stack_segment]

#[cfg(target_os = "android")]
#[no_mangle]
pub extern fn   rust_android_get_value()->libc::c_int {
	unsafe {g_rusty_value+=1; g_rusty_value as libc::c_int }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern fn   rust_android_init() {
	shadertest::lazy_create_resources();
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern fn   rust_android_render() {
	
	shadertest::render_no_swap();
}









