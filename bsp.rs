#[feature(globs)];
#[feature(macro_rules)];
#[feature(default_type_params)];
#[allow(dead_code)];

use r3d::gl::*;
use r3d::vecmath::*;
use std::vec_ng::Vec;
use std::io;
use std::intrinsics::{size_of,offset};
use std::libc::*;
use std::f32;
//use std::c_str::*;
mod r3d;
//mod macros;

// graphical test main, immiediate debug lines

fn draw_line(&(x0,y0,z0):&(f32,f32,f32),&(x1,y1,z1):&(f32,f32,f32), color:u32) {
	unsafe {
		glBegin(GL_LINES);
		gl_color(color);
		glVertex3f(x0,y0,z0); glVertex3f(x1,y1,z1);
		glEnd();
	}
}
fn v3isometric(&(x,y,z):&BspVec3)->BspVec3 {(x+y,z+(x-y)*0.5, z)}

fn draw_line_iso(v0:&BspVec3,v1:&BspVec3,color:u32, scale:f32) {
	draw_line(&v3isometric(&v3scale(v0,scale)),&v3isometric(&v3scale(v1,scale)), color)
}
unsafe fn gl_vertex_v3(&(x,y,z):&BspVec3) {
	glVertex3f(x,y,z);
}
unsafe fn gl_color(color:u32) {
	let r=(color)&255;
	let g=(color>>8)&255;
	let b=(color>>16)&255;
	glColor3f(r as f32 *(1.0/255.0),g as f32 *(1.0/255.0),b as f32 *(1.0/255.0));
}
fn draw_tri_iso(v0:&BspVec3,v1:&BspVec3,v2:&BspVec3,color:u32, scale:f32 ) {
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
		let bsp=Blob::<BspHeader>::read(&Path::new("data/e1m1.bsp"));
//		bsp.dump();
		let mut a=0.0;

		bsp.draw_faces();
		bsp.draw_edges();
//		bsp.draw_all_surface_edges();
		glFlush();

		while true {
			use std::f32;
			glutMainLoopEvent();
		}
	}
}


struct Blob<HEADER> {
	data:~[u8],
}

impl<T> std::ops::Deref<T> for Blob<T> {
	fn deref<'s>(&'s self)->&'s T {
		unsafe {	&*(&self.data[0] as *u8 as *T)
		}
	}
}

impl<T> Blob<T> {
	fn num_bytes(&self) -> uint { self.data.len() }
	
	fn read(path:&Path)->Blob<T> {
		let data=
			match io::File::open(path).read_to_end() {
				Ok(data)=>{
					println!("read {} {} bytes", /*path_to_str*/path.as_str().unwrap_or(""), data.len());	
					data
				},
				Err(E)=>{
					println!("failed to read {}", path.as_str().unwrap_or("")); 
					//~[0,..intrinsics::size_of::<Header>()]		// still returns an empty object, hmmm.
					//vec::from_elem(0,intrinsics::size_of::<Header>())
					~[]
				}
			};
		Blob::<T>  {data:data}
	}
}


pub struct DEntry<Header, T> { 
	// we got a nasty bug passing wrong base ptr without header typeinfo here
	// &self instead of self..
	offset:u32, 
	size:u32
}
impl<Header,T> DEntry<Header,T> {
	fn len(&self)->uint { unsafe {self.size as uint /  size_of::<T>()} }
	fn get<'a>(&'a self, owner:&'a Header,i:uint) -> &'a T{
		unsafe {
			&*(((owner as *Header as *u8).offset(self.offset as int) as *T).offset(i as int))
		}
	}
}
pub type BspDEntry<T> =DEntry<BspHeader,T>;
pub struct BspHeader {
	version:u32,
	entities:BspDEntry<Entity>,
	planes:BspDEntry<Plane>,

	miptex:BspDEntry<MipTex>,
	vertices:BspDEntry<BspVec3>,

	visibility:BspDEntry<VisiList>,
	nodes:BspDEntry<BspNode>,

	texinfo:BspDEntry<TextureInfo>,

	faces:BspDEntry<Face>,

	lightmaps:BspDEntry<LightMap>,
	clipnodes:BspDEntry<ClipNode>,

	leafs:BspDEntry<BspLeaf>,

	marksurfaces:BspDEntry<i16>, //? no
	edges:BspDEntry<Edge>,

	surfedges:BspDEntry<i32>, // ? no
	models:BspDEntry<Model>
}

impl BspHeader {
	fn dump_vertices(&self) {	
		println!("{}",self.vertices.len());
		let mut i:uint=0;
		let vtlen=self.vertices.len();
		while i<vtlen { 
			let vtref= self.vertices.get(self,i as uint);
			println!("vertex{}/{}:owner={:p} vertex= {:p} ,({},{},{})",
				i,vtlen, self, vtref,
				vtref.val0(),vtref.val1(),vtref.val2());
			i+=1;
			let v=*vtref;
		}
		println!("{:p}, {:p}, {:p}", 
			self.vertices.get(self,0),
			self.vertices.get(self,1),
			self.vertices.get(self,2));
	
	}
	fn dump(&self) {
		println!("vertices: {:u}", self.vertices.len());
		self.dump_vertices();
	}
	fn draw_edges(&self) {
		let scale=1.0f32/3000.0f32;
		let mut i=0u;
		while i < self.edges.len() {
			let e= self.edges.get(self,i);
			let v0 = self.vertices.get(self, e.vertex0 as uint);
			let v1 = self.vertices.get(self, e.vertex1 as uint);
			draw_line_iso(v0,v1,0xffffff, scale);
			i+=1;
		}
	}
	fn draw_faces(&self) {
		let scale=1.0f32/3000.0f32;

		for i in range(0,self.faces.len()) {
			let face=self.faces.get(self, i);
			let eii = face.firstedge;
			let first_ei= *self.surfedges.get(self, eii as uint);
			let first_edge=if first_ei>=0 {self.edges.get(self,first_ei as uint)} else {self.edges.get(self,-first_ei as uint)};
			let first_vertex=if first_ei>=0 {first_edge.vertex0}else{first_edge.vertex1};
			let vfirst = self.vertices.get(self, first_vertex as uint);
			
			for esubi in range(0, face.num_edges) {
				let ei = *self.surfedges.get(self, (eii+esubi as i32) as uint);
				let edge=self.edges.get(self, ei as uint);
				let (v0,v1)=if ei>0 {
					let edge=self.edges.get(self, ei as uint);
					(	self.vertices.get(self, edge.vertex0 as uint),
						self.vertices.get(self, edge.vertex1 as uint)
					)
				} else {
					let edge=self.edges.get(self, -ei as uint);
					(	self.vertices.get(self, edge.vertex1 as uint),
						self.vertices.get(self, edge.vertex0 as uint)
					)
				};
				draw_tri_iso(vfirst, v0,v1, (((i^ i <<13)*i)^(i<<13)) as u32, scale);
//				draw_line_iso(v0,v1,scale);
			}
		}
	}
	fn draw_all_surface_edges(&self)
	{
		for i in range(0, self.surfedges.len()) {
			//let mut ei=self.edgelist.get(self, i);
			
//			let edge = self.edges.get(self, ei);
			self.draw_edge(*(self.surfedges.get(self, i))  as int);
		}
	}
	fn draw_edge(&self, mut ei:int) {
		let scale=1.0f32/3000.0f32;
		if ei<0 {ei=-ei}
		let edge=self.edges.get(self, ei as uint);
		let v0 = self.vertices.get(self, edge.vertex0 as uint);
		let v1 = self.vertices.get(self, edge.vertex1 as uint);
		draw_line_iso(v0,v1,0xffffff, scale);
	}
}


// minimal tuple vmath
pub fn v3scale(&(x,y,z):&BspVec3,s:f32)->BspVec3 {	(x*s,y*s,z*s)}
pub fn v3add(&(x0,y0,z0):&BspVec3,&(x1,y1,z1):&BspVec3)->BspVec3 {	(x0+x1,y0+y1,z0+z1)}
pub fn v3sub(&(x0,y0,z0):&BspVec3,&(x1,y1,z1):&BspVec3)->BspVec3 {	(x0-x1,y0-y1,z0-z1)}
pub fn v3mad(v0:&BspVec3,v1:&BspVec3,f:f32)->BspVec3 { v3add(v0,&v3scale(v1,f))}
pub fn v3lerp(v0:&BspVec3,v1:&BspVec3,f:f32)->BspVec3 { v3add(v0,&v3scale(&v3sub(v1,v0),f))}
pub fn v3dot(&(x0,y0,z0):&BspVec3,&(x1,y1,z1):&BspVec3)->f32 {	x0*x1+y0*y1+z0*z1}
pub fn v3cross(&(x0,y0,z0):&BspVec3,&(x1,y1,z1):&BspVec3)->BspVec3 { ((y0*z1-z0*y1),(z0*x1-z1*x0),(x0*y1-x1*y0)) }
pub fn v3norm(v0:&BspVec3)->BspVec3{ v3scale(v0,1.0/f32::sqrt(v3dot(v0,v0))) }

pub type Point3s=(i16,i16,i16);
pub type BBox=(Point3s,Point3s);
pub struct Entity;
pub struct Plane {
	normal:BspVec3,
	dist:f32,
	plane_type:u32	// 0,1,2 = axial planes x,y,z; 3,4,5 = x,y,z predominant..
}
pub struct MipTex {
	name:[c_char,..16],
	width:u32, height:u32, offset1:u32, offset2:u32, offset4:u32, offset8:u32
}
pub struct MipHeader {
	numtex:u32, 
}
impl MipHeader {
	pub unsafe fn tex_offsets(&self)->*u32 {
		(self as *_).offset(1) as *u32
	}
	pub unsafe fn tex_offset(&self, i:int)->u32 {
		let ofs=self.tex_offsets();
		*ofs.offset(i)
	}
	pub fn get_texture(&self, i:int)->&MipTex {
		unsafe {
			&*((self as *_ as *u8).offset( self.tex_offset(i) as int ) as *MipTex)
		}
	}
}

pub type BspVec3=(f32,f32,f32);
pub struct VisiList;
pub struct BspNode {
	plane_id:u32,
	priv children:[i16,..2],
	bbox:BBox,
	firstface:u16,
	numfaces:u16
}
enum BspNodeChild {
	ChildNode(i16),ChildLeaf(i16)
}
impl BspNode {
	pub fn child_node(&self, i:int)->BspNodeChild {
		match self.children[i] {
			x if x>=0 => ChildNode(x),
			x  =>ChildLeaf(-(self.children[i]+1))
		}
	}
}

pub struct TextureInfo {
	axis_s:BspVec3, ofs_s:f32,
	axis_t:BspVec3, ofs_t:f32,
	miptex:int,
	flags:int
}
pub struct Faces;
pub struct LightMap; //{ 	texels:[u8]} ??
pub struct ClipNode {
	planenum:u32,
	front:u16, back:u16,
}
pub struct BspLeaf {
	contents:u32, 
	visofs:u32, 
	min:Point3s,
	max:Point3s,
	firstmarksurface:u16,
	nummarksurfaces:u16,
	ambient_level:[u8,..AmbientNum]
}

pub struct Edge {
	vertex0:u16,vertex1:u16
}

enum Max{
	MaxMapHulls=4
}
static AmbientNum:int = 4;
enum Ambient {
	AmbientWater=0,AmbientSky,AmbientSlime,AmbientLava
}

pub struct Model {
	bound:BBox,
	origin:BspVec3,
	headnode:[i32,..MaxMapHulls],
	visileafs:i32,
	firstface:i32,
	numfaces:i32
	
}
pub struct Face {
	plane:u16,
	side:u16,
	firstedge:i32,
	num_edges:u16,
	texinfo:u16,
//	typelight:u8,
//	baselight:u8,
	light:[u8,..2],
	lightmap_ofs:i32, // [styles*sursize] samples..
}



 
