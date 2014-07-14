use super::gl::*;
use super::vecmath::*;
use super::matrix::*;
use super::formats::*;
use super::glut::*;
use std::mem::size_of;
use libc::c_char;

pub fn gl_matrix_projection(mat:&Matrix4) {
	unsafe{glMatrixMode(GL_PROJECTION);glLoadMatrixf(mat.ax().ref0());
	}
}
pub fn gl_matrix_modelview(mat:&Matrix4) {
	unsafe {glMatrixMode(GL_MODELVIEW);	glLoadMatrixf(mat.ax().ref0());
	}
}

pub fn draw_cross(s:f32) {
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

trait GlColor {fn gl_color(&self);}
impl GlColor for u32  {
	fn gl_color(&self) {
		let r=(*self)&255;
		let g=(*self>>8)&255;
		let b=(*self>>16)&255;
		unsafe {
			glColor3f(r as f32 *(1.0/255.0),g as f32 *(1.0/255.0),b as f32 *(1.0/255.0));
		}
	}
}
impl GlColor for Vec4  {
	fn gl_color(&self) {
		unsafe{
			glColor4f(self.x(),self.y(),self.z(),self.w())
		}
	}
}
impl GlColor for (f32,f32,f32,f32)  {
	fn gl_color(&self) {
		let (r,g,b,a)=*self;
		unsafe{
			glColor4f(r,g,b,a)
		}
	}
}
trait GlVertex{ fn gl_vertex(&self);}
impl<V:XYZ<f32>> GlVertex for V { fn gl_vertex(&self){ unsafe{glVertex3f(self.x(),self.y(),self.z());}}}
//impl GlVertex for Vec4<f32>{ fn gl_vertex(&self){ unsafe{glVertex3f(self.x(),self.y(),self.z());}}}
//impl GlVertex for (f32,f32,f32){ fn gl_vertex(&self){ let(x,y,z)=*self;unsafe{glVertex3f(x,y,z);}}}

fn gl_color<T:GlColor>(v:&T) {v.gl_color(); }
fn gl_vertex<T:GlVertex>(v:&T) {v.gl_vertex(); }

pub fn draw_line<V:GlVertex,C:GlColor>(a:&V,b:&V, color:&C) {
	unsafe {
		glBegin(GL_LINES);
		gl_color(color);
		glNormal3f(-1.0,-1.0,-1.0);
		gl_vertex(a);
		gl_color(color);
		glNormal3f(-1.0,-1.0,-1.0);
		gl_vertex(b);
	}
}
pub fn draw_line_vec4(a:&Vec4<f32>,b:&Vec4<f32>, rgba:u32) {
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


// minimal tuple vmath
pub type V2=(f32,f32);
pub type V3=(f32,f32,f32);
pub type V4=(f32,f32,f32,f32);
pub fn v3scale(&(x,y,z):&V3,s:f32)->V3 {	(x*s,y*s,z*s)}
pub fn v3add(&(x0,y0,z0):&V3,&(x1,y1,z1):&V3)->V3 {	(x0+x1,y0+y1,z0+z1)}
pub fn v3add4(a:&V3,b:&V3,c:&V3,d:&V3)->V3 {v3add(&v3add(a,b),&v3add(c,d))}
pub fn v3add3(a:&V3,b:&V3,c:&V3)->V3 {v3add(&v3add(a,b),c)}
pub fn v3sub(&(x0,y0,z0):&V3,&(x1,y1,z1):&V3)->V3 {	(x0-x1,y0-y1,z0-z1)}
pub fn v3neg(&(x,y,z):&V3)->V3 { (-x,-y,-z)}
pub fn v3mad(v0:&V3,v1:&V3,f:f32)->V3 { v3add(v0,&v3scale(v1,f))}
pub fn v3lerp(v0:&V3,v1:&V3,f:f32)->V3 { v3add(v0,&v3scale(&v3sub(v1,v0),f))}
pub fn v3dot(&(x0,y0,z0):&V3,&(x1,y1,z1):&V3)->f32 {	x0*x1+y0*y1+z0*z1 }
pub fn v3cross(&(x0,y0,z0):&V3,&(x1,y1,z1):&V3)->V3 { ((y0*z1-z0*y1),(z0*x1-z1*x0),(x0*y1-x1*y0)) }
pub fn v3norm(v0:&V3)->V3{ v3scale(v0,1.0/(v3dot(v0,v0).sqrt())) }
pub fn v3sub_norm(v0:&V3,v1:&V3)->V3{ v3norm(&v3sub(v0,v1))}
pub fn v3perp(v0:&V3,axis:&V3)->V3{ v3mad(v0, axis, -v3dot(v0,axis))}
pub fn v3para_perp(v0:&V3,axis:&V3)->(V3,V3){ let para=v3scale(axis, v3dot(v0,axis)); (para, v3sub(v0,&para)) }
pub fn v3mat_mul(&(ref ax,ref ay,ref az,ref pos):&(V3,V3,V3,V3), &(x,y,z):&V3)->V3 { v3add4(&v3scale(ax,x), &v3scale(ay,y), &v3scale(az,z),pos ) }
// inv only if orthonormal
pub fn v3mat_invmul(&(ref ax,ref ay,ref az,ref pos):&(V3,V3,V3,V3),src:&V3)->V3 { let ofs=v3sub(src,pos); (v3dot(src,ax),v3dot(src,ay),v3dot(src,az)) }
pub fn v3mat_lookat(pos:&V3, at:&V3,up:&V3)->(V3,V3,V3,V3) { let az=v3sub_norm(at,pos); let ax=v3norm(&v3cross(&az,up)); let ay=v3cross(&ax,&az); (ax,ay,az,pos.clone()) }
pub fn v3mat_identity()->(V3,V3,V3,V3) {((1.0,0.0,0.0),(0.0,1.0,0.0),(0.0,0.0,1.0),(0.0,0.0,0.0))}
pub fn v3triangle_norm(v0:&V3,v1:&V3,v2:&V3)->V3 { let v01=v3sub(v1,v0); let v02=v3sub(v2,v0); v3norm(&v3cross(&v02,&v01))}
// inv only if orthonormal
pub fn v3mat_inv(&((xx,xy,xz),(yx,yy,yz),(zx,zy,zz),ref pos):&(V3,V3,V3,V3) )->(V3,V3,V3,V3){ let ax=(xx,yx,zx);let ay=(xy,yy,zy);let az=(xz,yz,zz); let px= -v3dot(&ax,pos); let py=-v3dot(&ay,pos); let pz=-v3dot(&az,pos); let invpos=(px,py,pz); (ax,ay,az, invpos) }


pub fn	draw_image(size:(u32,u32),image:&Vec<u32>, pos:(f32,f32)) {
	unsafe {
//		let (tx,image)= self.get_texture_image(i);

		glRasterPos2f(pos.val0(),pos.val1());
		glDrawPixels(size.val0() as GLsizei,size.val1() as GLsizei, GL_RGBA, GL_UNSIGNED_BYTE, image.as_ptr() as *const  c_void);
		glFlush();
	}
}

pub fn get_format(bytes_per_pixel:u32, alpha_bits:u32)->(GLenum,GLenum) {
	match (bytes_per_pixel,alpha_bits) {
		(4,_) => (GL_RGBA,GL_UNSIGNED_BYTE),
		(3,0) => (GL_RGB,GL_UNSIGNED_BYTE),
		(2,4) => (GL_RGBA, GL_UNSIGNED_SHORT_4_4_4_4),
		(2,1) => (GL_RGBA, GL_UNSIGNED_SHORT_5_5_5_1),
		(2,0) => (GL_RGB, GL_UNSIGNED_SHORT_5_6_5),
		(1,8) => (GL_RGB, GL_UNSIGNED_BYTE_3_3_2),	// todo:should mean compressed.
		(1,_) => (GL_RGB, GL_UNSIGNED_BYTE_3_3_2),	// todo:should mean compressed.
		_ => (GL_RGBA, GL_UNSIGNED_BYTE)
	}
}

pub fn create_texture<Texel>((w,h):(u32,u32), raw_pixels:&Vec<Texel>, alpha_bits:u32)->GLuint {
	// todo: generic over format, u16->1555, u32->8888 u8->dxt5 and so on
	unsafe {
		let (fmt,fmt2)=get_format(size_of::<Texel>() as u32, alpha_bits);
		assert!(w*h==raw_pixels.len() as u32)
		let mut tx:[GLuint,..1]=[0,..1];
		glGenTextures(1,tx.as_mut_ptr());
		glBindTexture(GL_TEXTURE_2D,tx[0]);
		glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,GL_LINEAR as GLint);
		glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,GL_LINEAR as GLint);
		glTexImage2D(GL_TEXTURE_2D, 0, fmt as GLint, w as GLsizei,h as GLsizei, 0, fmt, fmt2, raw_pixels.as_ptr() as *const c_void); 
		tx[0]
	}
}
pub fn v3isometric(&(x,y,z):&(f32,f32,f32))->(f32,f32,f32) {(x+y,z+(x-y)*0.5, z)}

pub fn draw_line_iso(v0:&V3,v1:&V3,color:u32, scale:f32) {
	draw_line(&v3isometric(&v3scale(v0,scale)),&v3isometric(&v3scale(v1,scale)), &color)
}
pub unsafe fn gl_tex0(&(u,v):&(f32,f32)) {
	glTexCoord2f(u,v);
}

pub fn draw_tri_iso(v0:&V3,v1:&V3,v2:&V3,color:u32, scale:f32 ) {
	let tv0=v3isometric(&v3scale(v0,scale));
	let tv1=v3isometric(&v3scale(v1,scale));
	let tv2=v3isometric(&v3scale(v2,scale));
	unsafe {
		glBegin(GL_TRIANGLES);
		gl_color(&color);
		gl_vertex(&tv0);	
		gl_vertex(&tv1);	
		gl_vertex(&tv2);	
		glEnd();
	}
}
pub fn draw_tri_iso_tex(
		(v0,uv0):(&V3,V2), 
		(v1,uv1):(&V3,V2),
		(v2,uv2):(&V3,V2),
		color:u32, scale:f32 ) {
	let tv0=v3isometric(&v3scale(v0,scale));
	let tv1=v3isometric(&v3scale(v1,scale));
	let tv2=v3isometric(&v3scale(v2,scale));
	unsafe {
		glBegin(GL_TRIANGLES);
		gl_color(&color);
		gl_tex0(&uv0);
		gl_vertex(&tv0);
		gl_tex0(&uv1);
		gl_vertex(&tv1);	
		gl_tex0(&uv2);
		gl_vertex(&tv2);	
		glEnd();
	}
}
pub fn draw_tri_tex(
		(v0,uv0):(&V3,V2), 
		(v1,uv1):(&V3,V2),
		(v2,uv2):(&V3,V2),
		color:u32,scale:f32) {
	unsafe {
		glBegin(GL_TRIANGLES);
		gl_color(&color);
		gl_tex0(&uv0);
		gl_vertex(&v3scale(v0,scale));
		gl_tex0(&uv1);
		gl_vertex(&v3scale(v1,scale));
		gl_tex0(&uv2);
		gl_vertex(&v3scale(v2,scale));
		glEnd();
	}
}

pub unsafe fn draw_init() {	
	dump!();
	let mut argc:c_int=0;
	let argv:Vec<*const c_char> =Vec::new();
	glutInit((&mut argc) as *mut c_int,0 as *const *const c_char );

	glutInitDisplayMode(GLUT_RGBA|GLUT_SINGLE);
	glutInitWindowSize(1024,1024);
	let win=glutCreateWindow("testbed".to_c_str().unwrap());
	glutDisplayFunc(draw_null as *const u8);
	
	glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);
	dump!();
}
pub unsafe fn draw_null(){
	glFlush();
}
pub unsafe fn draw_show() {
	glFlush();
}

// todo: some malarchy for quit key.
// you could even buffer everything up and allow
// teh user to zoom in and out on a 2d image.
pub fn draw_win_loop() {
	unsafe {
		loop {
			use std::f32;
			glutMainLoopEvent();
		}
	}
}

pub unsafe fn draw_set_texture(tex_unit:i32, tex_id:GLuint) {
	assert!(tex_unit>=0 && tex_unit<16);
	glActiveTexture((GL_TEXTURE0 as int +tex_unit as int) as GLenum);
	if tex_id>0 {
		glEnable(GL_TEXTURE_2D);
		glBindTexture(GL_TEXTURE_2D, tex_id);
	} else {
		glDisable(GL_TEXTURE_2D);
	}
}


pub fn random_color3(a:uint,b:uint,c:uint)->u32 {
	(a*b*c ^(a<<3)^(b<<8)*(c<<2)^(a<<19)^(b<<22)*(c<<13) )as u32
}
pub fn random_color(a:uint)->u32 {
	(a^(a<<3)^(a<<8)*(a<<2)^(a<<19)^(a<<22)*(a<<13) )as u32
}









