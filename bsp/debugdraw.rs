#[cfg(testbed)]
use r3d::gl::*;
use std::vec_ng::Vec;
use std::intrinsics::{size_of,offset};
use std::f32;

// thin wrapper around glut display functions,
// intended for simple debug graphics.



// minimal tuple vmath
type V2=(f32,f32);
type V3=(f32,f32,f32);
type V4=(f32,f32,f32,f32);
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
pub fn v3norm(v0:&V3)->V3{ v3scale(v0,1.0/f32::sqrt(v3dot(v0,v0))) }
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


#[cfg(testbed)]
pub fn	draw_image(size:(u32,u32),image:&Vec<u32>, pos:(f32,f32)) {
	unsafe {
//		let (tx,image)= self.get_texture_image(i);

		glRasterPos2f(pos.val0(),pos.val1());
		glDrawPixels(size.val0() as GLsizei,size.val1() as GLsizei, GL_RGBA, GL_UNSIGNED_BYTE, image.as_ptr() as *c_void);
		glFlush();
	}
}

#[cfg(testbed)]
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

#[cfg(testbed)]
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
		glTexImage2D(GL_TEXTURE_2D, 0, fmt as GLint, w as GLsizei,h as GLsizei, 0, fmt, fmt2, raw_pixels.as_ptr() as *c_void); 
		tx[0]
	}
}
#[cfg(testbed)]
pub fn draw_line(&(x0,y0,z0):&(f32,f32,f32),&(x1,y1,z1):&(f32,f32,f32), color:u32) {
	unsafe {
		glBegin(GL_LINES);
		gl_color(color);
		glVertex3f(x0,y0,z0); glVertex3f(x1,y1,z1);
		glEnd();
	}
}
#[cfg(testbed)]
pub fn v3isometric(&(x,y,z):&(f32,f32,f32))->(f32,f32,f32) {(x+y,z+(x-y)*0.5, z)}

#[cfg(testbed)]
pub fn draw_line_iso(v0:&V3,v1:&V3,color:u32, scale:f32) {
	draw_line(&v3isometric(&v3scale(v0,scale)),&v3isometric(&v3scale(v1,scale)), color)
}
#[cfg(testbed)]
pub unsafe fn gl_vertex_v3(&(x,y,z):&V3) {
	glVertex3f(x,y,z);
}
#[cfg(testbed)]
pub unsafe fn gl_tex0(&(u,v):&(f32,f32)) {
	glTexCoord2f(u,v);
}
#[cfg(testbed)]
pub unsafe fn gl_color(color:u32) {
	let r=(color)&255;
	let g=(color>>8)&255;
	let b=(color>>16)&255;
	glColor3f(r as f32 *(1.0/255.0),g as f32 *(1.0/255.0),b as f32 *(1.0/255.0));
}
#[cfg(testbed)]
pub fn draw_tri_iso(v0:&V3,v1:&V3,v2:&V3,color:u32, scale:f32 ) {
	let tv0=v3isometric(&v3scale(v0,scale));
	let tv1=v3isometric(&v3scale(v1,scale));
	let tv2=v3isometric(&v3scale(v2,scale));
	unsafe {
		glBegin(GL_TRIANGLES);
		gl_color(color);
		gl_vertex_v3(&tv0);	
		gl_vertex_v3(&tv1);	
		gl_vertex_v3(&tv2);	
		glEnd();
	}
}
#[cfg(testbed)]
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
		gl_color(color);
		gl_tex0(&uv0);
		gl_vertex_v3(&tv0);
		gl_tex0(&uv1);
		gl_vertex_v3(&tv1);	
		gl_tex0(&uv2);
		gl_vertex_v3(&tv2);	
		glEnd();
	}
}

pub unsafe fn draw_init() {
	let mut argc:c_int=0;
	let argv:Vec<*c_char> =Vec::new();
	glutInit((&mut argc) as *mut c_int,0 as **c_char );
		//::macros::test();

	glutInitDisplayMode(GLUT_RGBA);
	glutInitWindowSize(1024,1024);
	let win=glutCreateWindow("testbed".to_c_str().unwrap());

	glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);
}
pub unsafe fn draw_show() {
	glFlush();
}

// todo: some malarchy for quit key.
// you could even buffer everything up and allow
// teh user to zoom in and out on a 2d image.
pub fn draw_win_loop() {
	unsafe {
		while true {
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
