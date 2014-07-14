use super::macros::*;
use super::ut::*;
use super::vecmath::*;
use super::matrix::*;

struct Sphere<T=f32> {
	pos:Vec3<T>,
	radius:T
}

struct Ray<T=f32> {
	pos:Vec3<T>,
	dir:Vec3<T>
}

struct Plane<T=f32> {
	norm:Vec3<T>,
	dist:T
}

struct OOBB {
	mat:Matrix44f,
	ext:Extents<Vec3f>,
}

struct Triangle<V>{
	v0:V, v1:V, v2:V
}

impl<T:Float> Plane<T> {
	fn from_point(v0:&Vec3<T>, norm:&Vec3<T>)->Plane<T> {
		Plane{norm:norm.clone(),dist:v0.dot(norm)}
	}
	fn from_triangle(v0:&Vec3<T>,v1:&Vec3<T>,v2:&Vec3<T>)->Plane<T> {
		let norm=(v1.sub(v0)).cross(&(v2.sub(v0))).normalize();
		Plane{norm:norm,dist:norm.dot(v0)}
	}
}

struct Contact {
	pos:Vec3,
	norm:Vec3
}

#[deriving(Clone,Show)]
pub struct Extents<T=Vec3<f32>> {  
	min:T,max:T	
}

impl<T:Clone> Extents<T> {
	pub fn init(v:&T)->Extents<T>{ Extents::<T>{min:v.clone(),max:v.clone()}}
}
impl Extents<Vec3<f32>>{
	pub fn new()->Extents<Vec3<f32>> {
		let f=1000000.0f32;//todo: FLT_MAX
		Extents{min:Vec3(f,f,f),max:Vec3(-f,-f,-f)}
	}
}
pub trait Centre<V> {
	fn centre(&self)->V;
}
impl<V:Num> Centre<V> for Extents<V> {
	fn centre(&self)->V { (self.min+self.max)*(one::<V>()/(one::<V>()+one::<V>())) }
}
impl<T:Clone> Centre<Vec3<T>> for Sphere<T> {
	fn centre(&self)->Vec3<T> { self.pos.clone() }
}

impl<T:Num+PartialOrd,V:VecCmp<T>> Extents<V> { 
	pub fn include(&mut self, v:&V) {
		self.min=self.min.min(v);
		self.max=self.max.max(v);
	}
}

pub fn triangle_norm<T:Float,V:VecMath<T>>((v0,v1,v2):(&V,&V,&V))->V{
	let edge01=*v1-*v0;
	let edge12=*v2-*v1;
	return edge01.cross(&edge12);
}
pub fn triangle_extents<T:PartialOrd+Num+Clone,V:VecCmp<T>+Clone+Num>((v0,v1,v2):(&V,&V,&V))->Extents<V>{
	let mut ex=Extents::<V>::init(v0);
	ex.include(v1);
	ex.include(v2);
	ex
}

