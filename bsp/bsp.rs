#![feature(globs)]
#![feature(macro_rules)]
#![feature(default_type_params)]
#![allow(dead_code)]

#[cfg(testbed)]
extern crate collections;

#[cfg(testbed)]
extern crate libc;

#[cfg(testbed)]
extern crate debug;

//#[cfg(testbed)]

use r3d::*;

#[cfg(testbed)]
pub fn log_print(i:int, s:&str){std::io::println(s);}


// compile opt  --cfg testbed
#[cfg(testbed)]
#[path="../r3d/mod.rs"]
mod r3d;

#[cfg(testbed)]
pub fn main()
{
	unsafe {
		draw_init();
		let bsp=Blob::<BspHeader>::read(&Path::new("../data/e1m1.bsp"));
		let mut a=0.0f32;
		let mut tex_array=Vec::<GLuint>::new();
		// Load textures to GL
		bsp.visit_textures( &mut |i,_|{ // we miss do notation :(
				let (tx,img)=bsp.get_texture_image(i); 
				let txsize=(tx.width as u32,tx.height as u32);
				draw_image(txsize,&img, (((i&7)as f32)*(1.0/4.0)-1.0, (((i>>3)&7)as f32)*(1.0/4.0)-1.0) );
				let txi=create_texture((tx.width,tx.height), &img,8);
				tex_array.push(txi);
			}
		);
		// show the map, isometric.
		bsp.visit_triangles(
			&mut |_,(v0,v1,v2),(_,txinfo),(_,plane),(face_id,_)| {
				let txi=txinfo.miptex as uint;
				draw_set_texture(0,*tex_array.get(txi));
			
				fn applytx<'a>(tx:&'a TexInfo,v:&'a BspVec3)->(&'a BspVec3,(f32,f32)){
					(v, (v3dot(&tx.axis_s,v)+tx.ofs_s,v3dot(&tx.axis_t,v)+tx.ofs_t) )
				}
				draw_tri_iso_tex(applytx(txinfo,v0),applytx(txinfo,v1),applytx(txinfo,v2), 0xffffff, 1.0/2000.0)
			}
		);
		println!("foo");
		draw_show();
		draw_win_loop();
	}
}

pub struct Blob<HEADER> {
	data:Vec<u8>,
}

impl<T> Deref<T> for Blob<T> {
	fn deref<'s>(&'s self)->&'s T {
		unsafe {	&*(self.data.get(0)as*const _ as*const T)
		}
	}
}

impl<T> Blob<T> {
	pub fn num_bytes(&self) -> uint { self.data.len() }
	
	pub fn read(path:&Path)->Blob<T> {
		let data=
			match io::File::open(path).read_to_end() {
				Ok(data)=>{
					println!("read {} {} bytes", /*path_to_str*/path.as_str().unwrap_or(""), data.len());	
					data
				},
				Err(e)=>{
					println!("failed to read {}", path.as_str().unwrap_or("")); 
					//~[0,..intrinsics::size_of::<Header>()]		// still returns an empty object, hmmm.
					//vec::from_elem(0,intrinsics::size_of::<Header>())
					Vec::new()
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
//unsafe fn byte_ofs_ref<'a,X,Y=X,I:Int=int>(base:&'a X, ofs:I)->&'a Y {
//	&*( (base as *_ as *u8).offset( ofs.to_int().unwrap() ) as *Y)
//}


impl<Header,T> DEntry<Header,T> {
	fn len(&self)->uint { unsafe {self.size as uint /  size_of::<T>()} }
	fn get<'a>(&'a self, owner:&'a Header,i:uint) -> &'a T{
		// TODO: to REALLY be safe, the sub-elements need to check safety from the blob 'owner'
		// unfortunately 'bspheader' doesn't seem to have that, although the last elements' ofs & size could be used
		// for an assert?
		unsafe {
			&*(((owner as *const Header as *const u8).offset(self.offset as int) as *const T).offset(i as int))
//			&*(byte_ofs_ptr(owner, self.offset).offset(i as int))
		}
	}
}
pub type BspDEntry<T> =DEntry<BspHeader,T>;

#[repr(C)]
pub struct BspHeader {
	pub version:u32,
	pub entities:BspDEntry<Entity>,
	pub planes:BspDEntry<Plane>,

	pub miptex:BspDEntry<MipHeader>,
	pub vertices:BspDEntry<BspVec3>,

	pub visibility:BspDEntry<VisiList>,
	pub nodes:BspDEntry<BspNode>,

	pub texinfo:BspDEntry<TexInfo>,

	pub faces:BspDEntry<Face>,

	pub lightmaps:BspDEntry<LightMap>,
	pub clipnodes:BspDEntry<ClipNode>,

	pub leafs:BspDEntry<BspLeaf>,

	pub marksurfaces:BspDEntry<i16>, //? no
	pub edges:BspDEntry<Edge>,

	pub surfedges:BspDEntry<i32>, // ? no
	pub models:BspDEntry<Model>,
}
macro_rules! get {
	($obj:ident . $field:ident [ $id:expr ] )=>($obj . $field . get( $obj , $id as uint ))
}
impl BspHeader {
	pub fn dump_vertices(&self) {	
		println!("vertices:{}(",self.vertices.len());
		let mut i:uint=0;
		let vtlen=self.vertices.len();
		while i<vtlen { 
			let vtref= self.vertices.get(self,i);
			println!("vertex{}/{}:owner={:p} vertex= {:p} ,({},{},{})",
				i,vtlen, self, vtref,
				vtref.val0(),vtref.val1(),vtref.val2());
			i+=1;
			let v=*vtref;
		}
		println!("vertices:)");
	
	}
	pub fn dump(&self) {
		println!("ptrs: {:p}\t{:p}\t{:p}",self, &self.entities, &self.planes);
		println!("id: {}", self.version);
		println!("entities: {}{}", self.entities.offset, self.entities.size);
		println!("BSP info:-(");
		println!("entities: {:u}", self.entities.len());
		println!("planes: {:u}", self.planes.len());
		println!("miptex: {:u}", self.miptex.len());
		println!("vertices: {:u}", self.vertices.len())		println!("nodes: {:u}", self.nodes.len());
		println!("faces: {:u}", self.faces.len())
		println!("lightmaps: {:u}", self.lightmaps.len())
		println!("BSP info:-)");
		self.dump_vertices();
	}
	// some convinient accessors. - TODO autogenerate from a macro
	pub fn visit_triangles<'a,'b,R>(
			&'a self,
			fn_apply_to_tri:
				&mut |	tri_indices:(uint,uint,uint),
						tri_vertices:(&'a BspVec3,&'a BspVec3,&'a BspVec3),
						texinfo:(uint,&'a TexInfo),
						plane:(uint,&'a Plane),
						face_id:(uint,&'a Face)|:'b->R
			)->Vec<R>
	{
		let mut return_val:Vec<R> =Vec::new();	// todo: reserve
		for face_id in range(0,self.faces.len()) {
			let face=self.faces.get(self, face_id);
			let eii = face.firstedge;
			let first_ei= *get!{self.surfedges[eii]};
			let first_edge= get!{self.edges[if first_ei>=0{first_ei}else{-first_ei}]};
			let iv0=(if first_ei>=0 {first_edge.vertex0}else{first_edge.vertex1})  as uint;
			let v0 = self.vertices.get(self, iv0 as uint) ;
			
			// todo: iterate as strips, not fans.
			for esubi in range(0, face.num_edges) {
				let ei = *get!{self.surfedges[eii+esubi as i32]};
				let edge=get!{self.edges[ei]};
				let edge=get!{self.edges[if ei>0{ei}else{-ei}]};
				let (iv1,iv2)=if ei>=0{ 
					(edge.vertex0 as uint,edge.vertex1 as uint)
				} else {
					(edge.vertex1 as uint,edge.vertex0 as uint)
				};
				let mut v1=self.vertices.get(self, iv1 as uint);
				let mut v2=self.vertices.get(self, iv2 as uint);

				let tri_result=
				(*fn_apply_to_tri) (
					(iv0,iv1,iv2),
					(v0,v1,v2),	
					(face.texinfo as uint,	get!{self.texinfo[face.texinfo]} ),
					(face.plane as uint,	get!{self.planes[face.plane]} ),
					(face_id, face)
				);
				return_val.push(tri_result);
			}
		}
		return_val
	}
	pub fn visit_faces<'a>(&'a self, f:&mut |i:uint, f:&Face |:'a) {
		for i in range(0, self.faces.len()) {
			(*f)(i, get!{self.faces[i]} );
		}
	}

	pub fn get_used_textures(&self)->HashSet<uint> {
		let mut used_tx= HashSet::<uint>::new();
		self.visit_faces( &mut |i:uint,face:&Face|{used_tx.insert(face.texinfo as uint);});
		used_tx
	}

	pub fn get_texture<'a>(&'a self, i:uint)->&'a MipTex {
		let txh=self.miptex.get(self,0);
		let tx = unsafe {&*(
			(txh as *const _ as *const u8).offset(*txh.miptex_offset.unsafe_ref(i as uint) as int) as *const MipTex
		)};
		tx
	}

	pub fn visit_textures<'a>(&'a self, mut tex_fn:&mut|i:uint,tx:&MipTex|:'a) {
		println!("visit textures self={:p}",&self);
		let txh =self.miptex.get(self,0);
		println!("visit textures txh={:p}",txh);
		for i in range(0,txh.numtex) {
			println!("{}/{}",i,txh.numtex);
			let tx=self.get_texture(i as uint);
			unsafe {
				println!("tx: {} {} {} {}",
					i,
					CString::new(&tx.name[0],false).as_str().unwrap_or(""), 
					tx.width, tx.height);
			}
			(*tex_fn)( i as uint, tx );
		}
	}

}

impl BspHeader {
	fn draw_edges(&self) {
		let scale=1.0f32/3000.0f32;
		let mut i=0u;
		while i < self.edges.len() {
			let e= get!{self.edges[i]};
			let v0 = get!{self.vertices[e.vertex0]};
			let v1 = get!{self.vertices[e.vertex1]};
			draw_line_iso(v0,v1,0xffffff, scale);
			i+=1;
		}
	}
	fn draw_faces(&self) {
		let scale=1.0f32/3000.0f32;
		self.visit_triangles(
			&mut |(i0,i1,i2),(v0,v1,v2),(_,txinfo),_,(face_id,_)| draw_tri_iso(v0,v1,v2, random_color(face_id), scale)
		);
		
	}
	fn draw_all_surface_edges(&self)
	{
		for i in range(0, self.surfedges.len()) {
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


pub type Point3s=(i16,i16,i16);
pub type BBox=(Point3s,Point3s);
pub struct Entity(u8);
pub struct Plane {
	pub normal:BspVec3,
	pub dist:f32,
	pub plane_type:u32	// 0,1,2 = axial planes x,y,z; 3,4,5 = x,y,z predominant..
}
pub struct MipTex {
	pub name:[c_char,..16],
	pub width:u32, pub height:u32, pub offset1:u32, pub offset2:u32, pub offset4:u32, pub offset8:u32
}
pub struct MipHeader {
	pub numtex:u32, 
	pub miptex_offset:[u32,..0]	// actual size is..
}
impl MipHeader {
	pub unsafe fn tex_offsets(&self)->*const u32 {
		(self as *const _).offset(1) as *const u32
	}
	pub unsafe fn tex_offset(&self, i:int)->int {
		let ofs=self.tex_offsets();
		*ofs.offset(i) as int
	}
	pub fn get_texture(&self, i:int)->&MipTex {
		unsafe {
			&*((self as*const _ as*const u8).offset( self.tex_offset(i)) as*const MipTex)
		}
	}
}

pub type BspVec3=(f32,f32,f32);
pub type BspVec2=(f32,f32);
pub struct VisiList;
pub struct BspNode {
	plane_id:u32,
	children:[i16,..2],
	bbox:BBox,
	firstface:u16,
	numfaces:u16
}
pub enum BspNodeChild {
	ChildNode(i16),ChildLeaf(i16)
}
impl BspNode {
	pub fn child_node(&self, i:int)->BspNodeChild {
		match self.children[i as uint] {
			x if x>=0 => ChildNode(x),
			x  =>ChildLeaf(-(self.children[i as uint]+1))
		}
	}
}

pub struct TexInfo {
	pub axis_s:BspVec3, pub ofs_s:f32,
	pub axis_t:BspVec3, pub ofs_t:f32,
	pub miptex:i32,
	pub flags:i32
}
pub struct Faces(u8);
pub struct LightMap(u8); //{ 	texels:[u8]} ??
pub struct ClipNode {
	pub planenum:u32,
	pub front:u16, pub back:u16,
}
pub struct BspLeaf {
	pub contents:u32, 
	pub visofs:u32, 
	pub min:Point3s,
	pub max:Point3s,
	pub firstmarksurface:u16,
	pub nummarksurfaces:u16,
	pub ambient_level:[u8,..AmbientNum]
}

pub struct Edge {
	pub vertex0:u16,pub vertex1:u16
}

enum Max{
	MaxMapHulls=4
}
static AmbientNum:int = 4;
enum Ambient {
	AmbientWater=0,AmbientSky,AmbientSlime,AmbientLava
}

pub struct Model {
	pub bound:BBox,
	pub origin:BspVec3,
	pub headnode:[i32,..MaxMapHulls],
	pub visileafs:i32,
	pub firstface:i32,
	pub numfaces:i32
	
}
pub struct Face {
	pub plane:u16,
	pub side:u16,
	pub firstedge:i32,
	pub num_edges:u16,
	pub texinfo:u16,
//	typelight:u8,
//	baselight:u8,
	pub light:[u8,..2],
	pub lightmap_ofs:i32, // [styles*sursize] samples..
}

/// return a reference to a different type at a byte offset from the given base object reference
unsafe fn byte_ofs_ref<'a,X,Y=X,I:Int=int>(base:&'a X, ofs:I)->&'a Y {
	&*byte_ofs_ptr(base,ofs)
}
/// return a raw ptr to a different type at a byte offset from the given base object reference
unsafe fn byte_ofs_ptr<'a,FROM,TO=u8,I:Int=int>(base:&'a FROM, ofs:I)->*const TO {
	byte_ofs(base as *const _, ofs)
}
/// offsets a raw pointer by a byte amount, and changes type based on return value inference.
unsafe fn byte_ofs<'a,FROM,TO=u8,I:Int=int>(base:*const FROM, ofs:I)->*const TO {
	(base as *const u8).offset( ofs.to_int().unwrap() ) as *const TO
}

static g_palette:&'static [u8]=include_bin!("palette.lmp");

impl BspHeader {
	pub fn get_texture_image<'a>(&'a self, i:uint)->(&'a MipTex, Vec<u32>) {
		unsafe {
			let tx=self.get_texture(i);
			let mip0:*const u8=byte_ofs_ptr(tx, tx.offset1);


			let image = Vec::<u32>::from_fn(
				(tx.width*tx.height) as uint, 
				|i|{
					let color_index = *mip0.offset(i as int) as uint;
					let rgb_index=color_index*3;
					let r=g_palette[rgb_index+0] as u32;
					let g=g_palette[rgb_index+1] as u32;
					let b=g_palette[rgb_index+2] as u32;
					(r|(g<<8)|(b<<16)|(if color_index<255{0xff000000}else{0})) as u32
				}
			);
			(tx,image)
		}
	}

}
