#[feature(globs)];
#[feature(macro_rules)];
#[feature(default_type_params)];
#[allow(dead_code)];

use r3d::gl::*;
use std::vec_ng::Vec;
//use std::c_str::*;
mod r3d;

// graphical test main, immiediate debug lines

fn drawline((x0,y0,z0):(f32,f32,f32),(x1,y1,z1):(f32,f32,f32)) {
	unsafe {
		glBegin(GL_LINES);
		glVertex3f(x0,y0,z0); glVertex3f(x1,y1,z1);
		glEnd();
	}
}

pub fn main()
{
	unsafe {
		let mut argc:c_int=0;
		let argv:Vec<*c_char> =Vec::new();
		glutInit((&mut argc) as *mut c_int,0 as **c_char );
		//::macros::test();

		glutInitDisplayMode(GLUT_RGBA);
		glutInitWindowSize(1024,1024);
		let win=glutCreateWindow("testbed".to_c_str().unwrap());

		glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);
		let mut a=0.0;
		while true {
			use std::f32;
			glutMainLoopEvent();
			a+=0.01;
			glColor3f(
				f32::sin(a*0.53)*0.5+0.0,
				f32::sin(a*0.62)*0.5+0.0,
				f32::sin(a*0.71)*0.5+0.0
				);
			drawline((f32::sin(a*0.92),f32::cos(a*1.02),0.5),(f32::cos(a*1.1),f32::sin(a*0.98),0.5));
			glFlush();
		}
	}
}
