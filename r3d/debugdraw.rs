use super::gl::*;
use super::vecmath::*;
use super::matrix::*;
use super::formats::*;

pub fn gl_matrix_projection(mat:&Matrix4) {
	unsafe{glMatrixMode(GL_PROJECTION);glLoadMatrixf(mat.ax().ref0());
	}
}
pub fn gl_matrix_modelview(mat:&Matrix4) {
	unsafe {glMatrixMode(GL_MODELVIEW);	glLoadMatrixf(mat.ax().ref0());
	}
}

pub fn dbg_cross(s:f32) {
	unsafe {
		glBegin(GL_LINES);

		glColor4f(1.0,0.0,0.0,1.0);
		glNormal3f(-1.0,-1.0,-1.0);
		glVertex3f(-s,-s,-s);
		glColor4f(0.0,0.0,0.0,1.0);
		glNormal3f(-1.0,-1.0,-1.0);
		glVertex3f(s,s,s);

		glColor4f(1.0,0.0,0.0,1.0);
		glNormal3f(-1.0,-1.0,-1.0);
		glVertex3f(s,-s,-s);
		glColor4f(1.0,0.0,0.0,1.0);
		glNormal3f(-1.0,-1.0,-1.0);
		glVertex3f(-s,s,s);

		glColor4f(0.0,0.0,1.0,1.0);
		glNormal3f(-1.0,-1.0,-1.0);
		glVertex3f(-s,s,-s);
		glColor4f(0.0,0.0,1.0,1.0);
		glNormal3f(-1.0,-1.0,-1.0);
		glVertex3f(s,-s,s);

		glColor4f(0.0,1.0,0.0,1.0);
		glNormal3f(-1.0,-1.0,-1.0);
		glVertex3f(s,s,-s);
		glColor4f(0.0,1.0,0.0,1.0);
		glNormal3f(-1.0,-1.0,-1.0);
		glVertex3f(-s,-s,s);
		glEnd();
	}
}
pub fn gl_vertex(v:&Vec3<f32>) {
	unsafe {glVertex3f(v.x(),v.y(),v.z())}
}
pub fn gl_vertex_vec4(v:&Vec4<f32>) {
	unsafe {glVertex3f(v.x(),v.y(),v.z())}
}
pub fn gl_color(v:&Vec4<f32>) {
	unsafe{glColor4f(v.x(),v.y(),v.z(),v.w())}
}
pub fn dbg_line(a:&Vec3<f32>,b:&Vec3<f32>, rgba:u32) {
	let color=rgba.unpack_8888();
	unsafe {
		glBegin(GL_LINES);
		gl_color(&color);
		glNormal3f(-1.0,-1.0,-1.0);
		gl_vertex(a);
		gl_color(&color);
		glNormal3f(-1.0,-1.0,-1.0);
		gl_vertex(b);
	}
}
pub fn dbg_line_vec4(a:&Vec4<f32>,b:&Vec4<f32>, rgba:u32) {
	let color=rgba.unpack_8888();
	unsafe {
		glBegin(GL_LINES);
		gl_color(&color);
		glNormal3f(-1.0,-1.0,-1.0);
		gl_vertex_vec4(a);
		gl_color(&color);
		glNormal3f(-1.0,-1.0,-1.0);
		gl_vertex_vec4(b);
	}
}
