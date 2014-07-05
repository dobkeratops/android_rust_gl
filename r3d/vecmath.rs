#![feature(globs)]
#![allow(unused_attribute)]
#![feature(default_type_params)]
#![feature(macro_rules)]
#![allow(dead_code)]
#![allow(unused_variable)]
#![allow(unreachable_code)]

#[cfg(run)]
extern crate debug;

use std::cmp;


pub use std::num;
pub use std::f32::*;
pub use std::num::*;
pub use std::num::Float;
pub use std::num::Float;
use std::io;

//#[cfg(run)]
//mod macros;

#[cfg(run)]
fn log_print(i:int, s:&str){std::io::println(s);}

/// Generic maths classes
/// member functions prefixed with 'v' for easier life without code-completion, and to distinguish from operator overloads (official langauge level "add") etc

#[deriving(Clone,Show)]
pub struct Vec2<T=f32>(T,T);

#[deriving(Clone,Show)]
pub struct Vec3<T=f32>{pub x:T,pub y:T,pub z:T}

#[deriving(Clone,Show)]
pub struct Vec4<T=f32>{pub x:T,pub y:T,pub z:T,pub w:T}

fn min<T:PartialOrd>(a:T,b:T)->T { if a<b{a}else{b} }
fn max<T:PartialOrd>(a:T,b:T)->T { if a>b{a}else{b} }


pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;

pub type Vec2i = Vec2<i32>;
pub type Vec3i = Vec3<i32>;
pub type Vec4i = Vec4<i32>;

pub type Vec2d = Vec2<f64>;
pub type Vec3d = Vec3<f64>;
pub type Vec4d = Vec4<f64>;

pub type Vec2u = Vec2<u32>;
pub type Vec3u = Vec3<u32>;
pub type Vec4u = Vec4<u32>;

// TODO half-precision type for GL..
// TODO: Packed normal 10:10:10
// TODO: 565 colors

impl<T:Clone> Vec2<T> {
	pub fn new(x:T,y:T)->Vec2<T> { Vec2(x,y)}
	pub fn splat(v:T)->Vec2<T> { Vec2(v.clone(),v.clone())}
}

impl<T:Clone+Num> Vec2<T> {
	pub fn cross_to_scalar(&self,other:&Vec2<T>)->T {self.x()*other.y()-self.y()*other.x()}
	pub fn cross_z(&self,z:T)->Vec2<T> { Vec2(-self.y()*z,self.x()*z)}
	pub fn cross_one(&self)->Vec2<T> { Vec2(-self.y().clone(),self.x().clone())}
}
impl<T:Clone+Zero> Vec3<T> {
	pub fn new(vx:T,vy:T,vz:T)->Vec3<T>	{Vec3::<T>{x:vx.clone(),y:vy.clone(),z:vz.clone()}}
	pub fn splat(v:T)->Vec3<T> { Vec3{x:v.clone(),y:v.clone(),z:v.clone()}}

	pub fn from_vec2(xy:Vec2<T>,z:T)->Vec3<T> {Vec3::<T>{x:xy.x().clone(),y:xy.y().clone(),z:z.clone()}}
}
impl<T:Clone+Zero> Vec4<T> {
	pub fn new(x:T,y:T,z:T,w:T)->Vec4<T>	{Vec4{x:x.clone(),y:y.clone(),z:z.clone(),w:w.clone()}}
	pub fn splat(v:T)->Vec4<T> { Vec4{x:v.clone(),y:v.clone(),z:v.clone(),w:v.clone()}}	// todo -move to elsewhere

	pub fn from_vec3(xyz:Vec3<T>,w:T)->Vec4<T> {Vec4{x:xyz.x.clone(),y:xyz.y.clone(),z:xyz.z.clone(),w:w.clone()}}
	pub fn from_vec2(xy:Vec2<T>,z:T,w:T)->Vec4<T> {Vec4{x:xy.x().clone(),y:xy.y().clone(),z:z.clone(),w:w.clone()}}
	pub fn from_vec2_vec2(xy:Vec2<T>,zw:Vec2<T>)->Vec4<T> {Vec4{x:xy.x().clone(),y:xy.y().clone(),z:zw.x().clone(),w:zw.y().clone()}}
}

pub trait VecAccessors<T:Clone> {
	fn x(&self)->T;
	fn y(&self)->T;
	fn z(&self)->T;
	fn w(&self)->T;
}

pub trait VecConsts<T:Clone+One+Zero>
{
	fn origin()->Self;
	fn axis(i:int)->Self;
	fn one()->Self;
}

//	fn rsub(&self,&b:Self)->Self { b.sub(self)}
pub trait VecPermute<T:Clone+One+Zero> : VecAccessors<T> {
	fn xy(&self)->Vec2<T>	{ Vec2(self.x(),self.y())}
	fn yx(&self)->Vec2<T>	{ Vec2(self.y(),self.x())}
	fn xz(&self)->Vec2<T>	{ Vec2(self.x(),self.z())}
	fn yz(&self)->Vec2<T>	{ Vec2(self.y(),self.z())}

	fn xy01(&self)->Vec4<T>	{ Vec4::new(self.x(),self.y(),zero(),one())}

	fn xyz(&self)->Vec3<T>	{ Vec3::new(self.x(),self.y(),self.z())}
	fn xyz1(&self)->Vec4<T>	{ Vec4::new(self.x(),self.y(),self.z(),one())}	// vec3 to homogeneous point
	fn xyz0(&self)->Vec4<T>	{ Vec4::new(self.x(),self.y(),self.z(),zero())}	// vec3 to homogeneous offset
	fn xyzw(&self)->Vec4<T>	{ Vec4::new(self.x(),self.y(),self.z(),self.w())}

	// permutes useful for swapping rgb/bgr
	fn zyx(&self)->Vec3<T>	{ Vec3::new(self.z(),self.y(),self.x())}
	fn zyx0(&self)->Vec4<T>	{ Vec4::new(self.z(),self.y(),self.x(),zero())}
	fn zyx1(&self)->Vec4<T>	{ Vec4::new(self.z(),self.y(),self.x(),one())}

	// permutes for swapping Y or Z being up
	fn xzy(&self)->Vec3<T>	{ Vec3::new(self.x(),self.z(),self.y())}
	fn xzy0(&self)->Vec4<T>	{ Vec4::new(self.x(),self.z(),self.y(),zero())}
	fn xzy1(&self)->Vec4<T>	{ Vec4::new(self.x(),self.z(),self.y(),one())}
	// permutes for cross-product
	// i  j   k
	// x0 y0 z0
	// x1 y1 z1

	// x'=y0*z1 - z0*y1 ,  y'=z0*x1-x0*z1,  z'=x0*y1-y0*x1
	fn yzx(&self)->Vec3<T> {Vec3::new(self.y(),self.z(),self.x())}
	fn zxy(&self)->Vec3<T> {Vec3::new(self.z(),self.x(),self.y())}
	fn yzxw(&self)->Vec4<T> {Vec4::new(self.y(),self.z(),self.x(),self.w())}
	fn zxyw(&self)->Vec4<T> {Vec4::new(self.z(),self.x(),self.y(),self.w())}
	fn yzx0(&self)->Vec4<T> {Vec4::new(self.y(),self.z(),self.x(),zero())}
	fn zxy0(&self)->Vec4<T> {Vec4::new(self.z(),self.x(),self.y(),zero())}

	// splats in permute syntax
	fn xxxx(&self)->Vec4<T>	{ Vec4::new(self.x(),self.x(),self.x(),self.x())}
	fn yyyy(&self)->Vec4<T>	{ Vec4::new(self.y(),self.y(),self.y(),self.y())}
	fn zzzz(&self)->Vec4<T>	{ Vec4::new(self.z(),self.z(),self.z(),self.z())}
	fn wwww(&self)->Vec4<T>	{ Vec4::new(self.w(),self.w(),self.w(),self.w())}


	// Synonyms
	fn to_vec4_w1(&self,w:T)->Vec4<T> { self.xyz1()}
	fn to_vec4_w0(&self,w:T)->Vec4<T> { self.xyz0()}
	fn to_vec4(&self)->Vec4<T> { self.xyzw()}
	fn to_vec3(&self)->Vec3<T> { self.xyz()}
	fn to_vec2(&self)->Vec2<T> { self.xy()}
}

pub trait VecNum<T:Num> {
	fn from_xyz(x:T,y:T,z:T)->Self;
}
pub trait VecCmp<T:PartialOrd> {
	fn min(&self,b:&Self)->Self;
	fn max(&self,b:&Self)->Self;
}
pub trait Scale<F> {
	fn scale(&self,f:F)->Self;
}
// indirection trait for premul by scalar.
trait PreMulFloat<T:Float>:Scale<T> {
	fn pre_mul_float(&self,f:T)->Self{self.scale(f)}
}
impl<T:Float> PreMulFloat<T> for Vec3<T> {}
impl<T:Float> PreMulFloat<T> for Vec4<T> {}
// todo- only possible if impl other trait/other type restriction is lifted.
//impl Mul<Vec3<f32>,Vec3<f32>> for f32 { fn mul(&self,v:&Vec3<f32>)->Vec3<f32> { v.pre_mul_float(self)}}

// componentwise multiplication operator for vectors
impl<F:Float> Mul<Vec2<F>,Vec2<F>> for Vec2<F> {
	fn mul(&self,b:&Vec2<F>)->Vec2<F> {
		Vec2::<F>(self.x()*b.x(),self.y()*b.y())
	}
}

impl<T:Float+Clone,OUT,RHS:PreMulVec3<T,OUT>> Mul<RHS,OUT> for Vec3<T> {
	fn mul(&self,b:&RHS)->OUT {
		b.pre_mul_vec3(self)
	}
}

impl<F:Float,OUT, RHS:PreMulVec4<F,OUT>> Mul<RHS,OUT> for Vec4<F> {
	fn mul(&self,b:&RHS)->OUT {
		b.pre_mul_vec4(self)
	}
}

pub trait Cross<T,V>{
	fn cross(&self,&V)->V;
	fn cross_to_vec3(&self,&V)->Vec3<T>;
}

pub trait SumElems<T> {
	fn sum_elems(&self)->T;
}

pub trait PreMulVec2<T,RESULT> {
	fn pre_mul_vec2(&self,&Vec2<T>)->RESULT;
}
impl<T:Float+Clone> PreMulVec2<T,Vec2<T>> for Vec2<T> {
	fn pre_mul_vec2(&self, lhs:&Vec2<T>)->Vec2<T> { Vec2(lhs.x()*self.x(),lhs.y()*self.y()) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec2<f32,Vec2<f32>> for f32 {
	fn pre_mul_vec2(&self, lhs:&Vec2<f32>)->Vec2<f32> { Vec2(lhs.x()**self,lhs.y()**self) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec2<f64,Vec2<f64>> for f64 {
	fn pre_mul_vec2(&self, lhs:&Vec2<f64>)->Vec2<f64> { Vec2(lhs.x()**self,lhs.y()**self) }
}


pub trait PreMulVec3<T,RESULT> {
	fn pre_mul_vec3(&self,&Vec3<T>)->RESULT;
}
impl<T:Float+Clone> PreMulVec3<T,Vec3<T>> for Vec3<T> {
	fn pre_mul_vec3(&self, lhs:&Vec3<T>)->Vec3<T> { Vec3::new(lhs.x*self.x,lhs.y*self.y,lhs.z*self.z) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec3<f32,Vec3<f32>> for f32 {
	fn pre_mul_vec3(&self, lhs:&Vec3<f32>)->Vec3<f32> { Vec3::new(lhs.x**self,lhs.y**self,lhs.z**self) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec3<f64,Vec3<f64>> for f64 {
	fn pre_mul_vec3(&self, lhs:&Vec3<f64>)->Vec3<f64> { Vec3::new(lhs.x**self,lhs.y**self,lhs.z**self) }
}


pub trait PreMulVec4<T,RESULT> {
	fn pre_mul_vec4(&self,lhs:&Vec4<T>)->RESULT;
}
impl<T:Float+Clone> PreMulVec4<T,Vec4<T>> for Vec4<T> {
	fn pre_mul_vec4(&self, lhs:&Vec4<T>)->Vec4<T> { Vec4::new(lhs.x*self.x,lhs.y*self.y,lhs.z*self.z,lhs.w*self.w) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec4<f32,Vec4<f32>> for f32 {
	fn pre_mul_vec4(&self, lhs:&Vec4<f32>)->Vec4<f32> { Vec4::new(lhs.x**self,lhs.y**self,lhs.z**self,lhs.w**self) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec4<f64,Vec4<f64>> for f64 {
	fn pre_mul_vec4(&self, lhs:&Vec4<f64>)->Vec4<f64> { Vec4::new(lhs.x**self,lhs.y**self,lhs.z**self,lhs.w**self) }
}


// vector maths gathers primitive operations and implements more in terms of them
pub trait VecMath<T:Float=f32>:Clone+VecAccessors<T>+VecPermute<T>+VecConsts<T>+Zero+VecNum<T>+VecCmp<T>+Add<Self,Self>+Sub<Self,Self>+Scale<T>+Mul<Self,Self>+Cross<T,Self>+SumElems<T>+PreMulFloat<T> {
	fn dot(&self,b:&Self)->T	{self.mul(b).sum_elems()}
	fn para(&self,vaxis:&Self)->Self {  	let dotp=self.dot(vaxis); vaxis.scale(dotp) }

	fn neg(&self)->Self {self.scale(-one::<T>())}
	fn avr(&self,b:&Self)->Self {self.add(b).scale(one::<T>()/(one::<T>()+one::<T>()))}
	fn mad(&self,b:&Self,f:T)->Self	{self.add(&b.scale(f))}
	fn lerp(&self,b:&Self,f:T)->Self	{self.mad(&b.sub(self),f)}
	fn sqr(&self)->T { self.dot(self)}
	fn length(&self)->T { self.sqr().sqrt()}
	fn inv_length(&self)->T { one::<T>()/self.sqr().sqrt()}
	fn scale_to_length(&self,length:T)->Self { self.scale(length/self.sqr().sqrt()) }
	fn normalize(&self)->Self { self.scale(one::<T>()/self.sqr().sqrt()) }
	fn perp(&self,axis:&Self)->Self { let vpara =self.para(axis); self.sub(&vpara)}
	fn cross_norm(&self, b:&Self)->Self { self.cross(b).normalize() }
	fn sub_norm(&self,b:&Self)->Self { self.sub(b).normalize() }
	//pub fn axisScale(i:int,f:VScalar)->Self;
	// { VecOps::axis(i).vscale(f)} how?
	fn reflect(&self,a:&Self)->Self { self.mad(a, self.dot(a)*(one::<T>()+one::<T>())) }

	fn para_perp(&self,vaxis:&Self)->(Self,Self) {
		let vpara=self.para(vaxis);
		(vpara.clone(),self.sub(&vpara))
	}
}

//todo: HALF
fn bilerp<F:Float,V:VecMath<F>>(((v00,v01),(v10,v11)):((V,V),(V,V)),(s,t):(F,F))->V{
	(v00.lerp(&v01,s)).lerp(&v10.lerp(&v10,s), t)
}

impl<T:Float,V:Clone+VecAccessors<T>+VecPermute<T>+VecConsts<T>+Zero+VecNum<T>+VecCmp<T>+Add<V,V>+Sub<V,V>+Scale<T>+Mul<V,V>+Cross<T,V>+SumElems<T>+PreMulFloat<T>

> VecMath<T> for V {} 

// free function interface to vec maths
pub fn vadd<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.add(b)}
pub fn vsub<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.sub(b)}
pub fn vmad<T:Float,V:VecMath<T>>(a:&V,b:&V,f:T)->V { a.add(&b.scale(f))}
pub fn vmul<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.mul(b)}
pub fn vsqr<T:Float,V:VecMath<T>>(a:&V)->T { vmul(a,a).sum_elems()}
pub fn vlerp<T:Float,V:VecMath<T>>( a:&V,b:&V,f:T)->V { vmad(a, &vsub(b,a), f) }
pub fn vdot<T:Float,V:VecMath<T>>( a:&V,b:&V)->T { a.mul(b).sum_elems()}
pub fn vlength<T:Float,V:VecMath<T>>( a:&V)->T { a.mul(a).sum_elems().sqrt()}
pub fn vnormalize<T:Float,V:VecMath<T>>( a:&V)->V { a.scale(a.mul(a).sum_elems().rsqrt()) }
pub fn vsub_norm<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.sub(b).normalize() }
pub fn vcross<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.cross(b)}
pub fn vcross_norm<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.cross(b).normalize()}

//  wtf this does,t work now
impl<T:Add<T,T>+Clone+Zero> Add<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn add(&self,rhs:&Vec2<T>)->Vec2<T> { 
		Vec2::<T>(self.x()+rhs.x(), self.y()+rhs.y())
	}
}
impl<T:Add<T,T>+Clone+Zero> Add<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn add(&self,rhs:&Vec3<T>)->Vec3<T> { 
		Vec3::<T>::new(self.x()+rhs.x()   , self.y()+rhs.y(), self.z()+rhs.z())
	}
}
impl<T:Add<T,T>+Clone+Zero> Add<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn add(&self,rhs:&Vec4<T>)->Vec4<T> { 
		Vec4::<T>{x:self.x()+rhs.x()   , y:self.y()+rhs.y(), z:self.z()+rhs.z(), w:self.w()+rhs.w()}
	}
}
//  wtf this does,t work now
impl<T:Sub<T,T>+Clone+Zero> Sub<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn sub(&self,rhs:&Vec2<T>)->Vec2<T> { 
		Vec2::<T>(self.x()-rhs.x(), self.y()-rhs.y())
	}
}
impl<T:Sub<T,T>+Clone+Zero> Sub<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn sub(&self,rhs:&Vec3<T>)->Vec3<T> { 
		Vec3::<T>::new(self.x()-rhs.x()   , self.y()-rhs.y(), self.z()-rhs.z())
	}
}
impl<T:Sub<T,T>+Clone+Zero> Sub<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn sub(&self,rhs:&Vec4<T>)->Vec4<T> { 
		Vec4::<T>{x:self.x()-rhs.x()   , y:self.y()-rhs.y(), z:self.z()-rhs.z(), w:self.w()-rhs.w()}
	}
}


impl<T:Zero+Clone> Zero for Vec2<T> {
	fn zero()->Vec2<T> {Vec2::<T>::new(zero::<T>(),zero::<T>())}
	fn is_zero(&self)->bool { fail!();false/*self.x==zero::<T>() && self.y==zero::<T>() */}

//	fn zero()->Vec2<T> {Vec2::<T> {x:zero::<T>(),y:zero::<T>()}}
//	fn is_zero(&self)->bool { self.x==zero::<T>() && self.y==zero::<T>() }
}

/*
impl<T:One+Zero+Clone+Float> VecConsts for Vec2<T>
{
	fn origin()->Vec2<T> { Vec2::new(zero(),zero())}
	fn axis(i:int)->Vec2<T> {
		match i{
            0=>Vec2::new::<T>(1.0,0.0),
            1=>Vec2::new::<T>(0.0,1.0),
            _=>Vec2::new::<T>(0.0,0.0)
        }
	}

	fn axis(i:int)->Vec2<T> {
		match i{
            0=>Vec2::new::<T>(one::<T>(),zero::<T>()),
            1=>Vec2::new::<T>(zero::<T>(),one::<T>()),
            _=>Vec2::new::<T>(zero::<T>(),zero::<T>())
        }
	}
}
*/
fn vec_axis_scale<T:Float,V:VecConsts<T>+VecMath<T>>(i:int,f:T)->V { let ret:V; ret=VecConsts::axis(i); ret.scale(f) }

impl<T:Clone+One+Zero> VecPermute<T> for Vec2<T> {
}
impl<T:Num+Clone> VecNum<T> for Vec2<T> {
//	fn vadd(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(zero(),zero())}
//	fn vsub(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(zero(),zero())}
//	fn min(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(zero(),zero())}
//	fn max(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(zero(),zero())}

	fn from_xyz(x:T,y:T,_:T)->Vec2<T>{Vec2(x.clone(),y.clone())}
}
impl<T:PartialOrd+Clone+Zero> VecCmp<T> for Vec2<T> {
	fn min(&self,b:&Vec2<T>)->Vec2<T>	{Vec2(min(self.x().clone(),b.x().clone()),min(self.y().clone(),b.y().clone()))}
	fn max(&self,b:&Vec2<T>)->Vec2<T>	{Vec2(max(self.x().clone(),b.x().clone()),max(self.y().clone(),b.y().clone()))}
	
}
//impl MyFloat for f32;
//impl MyFloat for f64;

impl<T:Float> Scale<T> for Vec2<T> {
	fn scale(&self,f:T)->Vec2<T>		{Vec2(self.x()*f,self.y()*f)}
}

impl<T:Float+Zero+Clone> SumElems<T> for Vec2<T> {
	fn sum_elems(&self)->T	{self.x()+self.y()}
}

impl<T:Float> Cross<T,Vec2<T>> for Vec2<T> {
	fn cross(&self,_:&Vec2<T>)->Vec2<T>{Vec2(zero::<T>(),zero::<T>())}
	fn cross_to_vec3(&self,b:&Vec2<T>)->Vec3<T>	{Vec3::new(zero(),zero(),self.cross_to_scalar(b))}
//	pub fn axisScale(i:int,f:VScalar)->Vec2 { vecAxisScale(i,f) } 
}
impl<T:Clone+Zero> VecAccessors<T> for Vec2<T>
{
	fn x(&self)->T	{ self.x().clone()}
	fn y(&self)->T	{ self.y().clone()}
	fn z(&self)->T	{ zero::<T>()}
	fn w(&self)->T	{ zero::<T>()}
}

impl<T:Clone+Zero> Zero for Vec3<T> {
	fn zero()->Vec3<T>{Vec3::new(zero::<T>(),zero::<T>(),zero::<T>())}
	fn is_zero(&self)->bool  {fail!(); false/*self.x.is_zero() && self.y.is_zero() && self.z.is_zero()*/}
}

impl<T:Clone+Zero+One> VecConsts<T> for Vec3<T> {
	fn one()->Vec3<T>	{Vec3::new(one::<T>(),one::<T>(),one::<T>())}
	fn origin()->Vec3<T>	{Vec3::new(zero::<T>(),zero::<T>(),zero::<T>())}
	fn axis(i:int)->Vec3<T>{
		match i{ 0=>Vec3::new(one::<T>(),zero::<T>(),zero::<T>()),
                1=>Vec3::new(zero::<T>(),one::<T>(),zero::<T>()),
                2=>Vec3::new(zero::<T>(),zero::<T>(),one::<T>()),
                _=>Vec3::new(zero::<T>(),zero::<T>(),zero::<T>())}
	}
}
impl<T:Clone+Zero+One> VecConsts<T> for Vec2<T> {
	fn one()->Vec2<T>	{Vec2(one::<T>(),one::<T>())}
	fn origin()->Vec2<T>	{Vec2(zero::<T>(),zero::<T>())}
	fn axis(i:int)->Vec2<T>{
		match i{ 0=>Vec2(one::<T>(),zero::<T>()),1=>Vec2(zero::<T>(),one::<T>()),_=>Vec2(zero::<T>(),zero::<T>())}
	}
}



impl<T:Clone+One+Zero> VecPermute<T> for Vec3<T> {
}

impl<T:Clone+Num> VecNum<T> for Vec3<T> {
	fn from_xyz(x:T,y:T,z:T)->Vec3<T>{Vec3::new(x,y,z)}
}
impl<T:Clone+PartialOrd+Zero> VecCmp<T> for Vec3<T> {
	fn min(&self,b:&Vec3<T>)->Vec3<T>	{Vec3::new(
									min(self.x().clone(),b.x().clone()),
									min(self.y().clone(),b.y().clone()),
									min(self.z().clone(),b.z().clone()))}
	fn max(&self,b:&Vec3<T>)->Vec3<T>	{Vec3::new(
									max(self.x(),b.x()),
									max(self.y(),b.y()),
									max(self.z(),b.z()))}
}

impl<T:Float> Scale<T> for Vec3<T> { 
	fn scale(&self,f:T)->Vec3<T>		{Vec3::new(self.x*f,self.y*f,self.z*f)}
}

impl<T:Float> SumElems<T> for Vec3<T> {
	// todo-trait VecPrimOps
	fn sum_elems(&self)->T	{self.x+self.y+self.z}
}

impl<T:Float> Cross<T,Vec3<T>> for Vec3<T> {
	fn cross(&self,b:&Vec3<T>)->Vec3<T>	{Vec3::new(self.y*b.z-self.z*b.y,self.z*b.x-self.x*b.z,self.x*b.y-self.y*b.x)}

	fn cross_to_vec3(&self,b:&Vec3<T>)->Vec3<T>	{self.cross(b)}
	//fpub fn axisScale(i:int,f:VScalar)->Vec3 { VecConsts::axis(i).scale(f)} 
}
impl<T:Clone+Zero> VecAccessors<T> for Vec3<T> {
	fn x(&self)->T	{ self.x.clone()}
	fn y(&self)->T	{ self.y.clone()}
	fn z(&self)->T	{ self.z.clone()}
	fn w(&self)->T	{ zero::<T>()}

}
/*
impl<T:Float> Vec3<T> {
	fn vadd(&self,b:Vec3<T>)->Vec3<T>{
		Vec3::new(self.x+b.x,self.y+b.y,self.z+b.z)
	}
	fn vsub(&self,b:Vec3<T>)->Vec3<T>{
		Vec3::new(self.x-b.x,self.y-b.y,self.z-b.z)
	}
	fn vscale(&self,f:T)->Vec3<T>{
		Vec3::new(self.x*f,self.y*f,self.z*f)
	}
	fn vlerp(&self,b:Vec3<T>,f:T)->Vec3<T>{
		self.vadd(self.vsub(b).vscale(f))
	}
}
*/
impl<T:Clone+Zero> Zero for Vec4<T> {
	fn zero()->Vec4<T>{Vec4::new(zero::<T>(),zero::<T>(),zero::<T>(),zero::<T>())}
	fn is_zero(&self)->bool  {fail!();false/*self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero()*/}
}
impl<T:Clone+Zero+One> VecConsts<T> for Vec4<T> {
	fn one()->Vec4<T>	{Vec4::new(one::<T>(),one::<T>(),one::<T>(),one::<T>())}
	fn origin()->Vec4<T>	{Vec4::new(zero::<T>(),zero::<T>(),zero::<T>(),one::<T>())}

	fn axis(i:int)->Vec4<T>{
		match i{
            0=>Vec4::new(one::<T>(),zero::<T>(),zero::<T>(),zero::<T>()),
            1=>Vec4::new(zero::<T>(),one::<T>(),zero::<T>(),zero::<T>()),
            2=>Vec4::new(zero::<T>(),zero::<T>(),one::<T>(),zero::<T>()),
            3=>Vec4::new(zero::<T>(),zero::<T>(),zero::<T>(),one::<T>()),
            _=>Vec4::new(zero::<T>(),zero::<T>(),zero::<T>(),zero::<T>())
        }
	}

}

// Converting Vec2,Vec3,Vec4 to/from tuples.

impl<T:Clone> Vec4<T> {
	pub fn to_tuple(&self)->(T,T,T,T) { (self.x.clone(),self.y.clone(),self.z.clone(),self.w.clone()) }
	pub fn from_tuple((x,y,z,w):(T,T,T,T))->Vec4<T> {
		Vec4{x:x.clone(),y:y.clone(),z:z.clone(),w:w.clone()}
	}
}
impl<T:Clone> Vec3<T> {
	fn to_tuple(&self)->(T,T,T) { (self.x.clone(),self.y.clone(),self.z.clone()) }
	fn from_tuple((x,y,z):(T,T,T))->Vec3<T> { Vec3{x:x.clone(),y:y.clone(),z:z.clone()} }
}
impl<T:Clone+Zero> Vec2<T> {
	fn to_tuple(&self)->(T,T) { (self.x(),self.y()) }
	fn from_tuple((x,y):(T,T))->Vec2<T> { Vec2::new(x,y) }
}

impl<T:Clone+One+Zero> VecPermute<T> for Vec4<T> {
}


impl<T:Clone+Num> VecNum<T> for Vec4<T> {
	fn from_xyz(x:T,y:T,z:T)->Vec4<T>{Vec4::new(x,y,z,zero::<T>())}
}
impl<T:Clone+PartialOrd+Zero> VecCmp<T> for Vec4<T> {
	fn min(&self,b:&Vec4<T>)->Vec4<T>	{Vec4::new(
									min(self.x.clone(),b.x.clone()),
									min(self.y.clone(),b.y.clone()),
									min(self.z.clone(),b.z.clone()),
									min(self.w.clone(),b.w.clone()))}
	fn max(&self,b:&Vec4<T>)->Vec4<T>	{Vec4::new(
									max(self.x.clone(),b.x.clone()),
									max(self.y.clone(),b.y.clone()),
									max(self.z.clone(),b.z.clone()),
									max(self.w.clone(),b.w.clone()))}
}

impl<T:Float> Scale<T> for Vec4<T> {
	fn scale(&self,f:T)->Vec4<T>		{Vec4::new(self.x*f,self.y*f,self.z*f,self.w*f)}
}

impl<T:Num> SumElems<T> for Vec4<T> {
	fn sum_elems(&self)->T	{self.x+self.y+self.z+self.w}
}

impl<T:Num+Clone> Cross<T,Vec4<T>> for Vec4<T> {
	fn cross(&self,b:&Vec4<T>)->Vec4<T>	{Vec4::new(self.y*b.z-self.z*b.y,self.z*b.x-self.x*b.z,self.x*b.y-self.y*b.x,zero::<T>())}

	fn cross_to_vec3(&self,b:&Vec4<T>)->Vec3<T>	{self.cross(b).xyz()}
}

impl<T:Clone> VecAccessors<T> for Vec4<T>
{
	fn x(&self)->T	{ self.x.clone()}
	fn y(&self)->T	{ self.y.clone()}
	fn z(&self)->T	{ self.z.clone()}
	fn w(&self)->T	{ self.w.clone()}
}

pub fn vec_normalize<T:Float,V:VecMath<T>>(v:&V)->V { v.scale(v.sqr().rsqrt()) }
// TOOD - Zero for Vec3<T> , Vec4<T>


#[deriving(Clone,Show)]
pub struct Extents<T> {  
	min:T,max:T	
}

impl<T:Clone> Extents<T> {
	fn init(v:&T)->Extents<T>{ Extents::<T>{min:v.clone(),max:v.clone()}}
}

impl<T:Num+PartialOrd,V:VecMath<T>> Extents<V> { 
	fn size(&self)->V { self.max.sub(&self.min) }
	fn centre(&self)->V { self.min.add(&self.max).scale(one::<T>()/(one::<T>()+one::<T>())) }
	fn include(&mut self, v:&V) { self.min=self.min.min(v); self.max=self.max.max(v);}
}

pub fn triangle_norm<T:Float,V:VecMath<T>>((v0,v1,v2):(&V,&V,&V))->V{
	let edge01=*v1-*v0;
	let edge12=*v2-*v1;
	return edge01.cross(&edge12);
}
pub fn triangle_extents<T:PartialOrd+Num+Clone,V:VecMath<T>>((v0,v1,v2):(&V,&V,&V))->Extents<V>{
	let mut ex=Extents::<V>::init(v0);
	ex.include(v1);
	ex.include(v2);
	ex
}

// todo - math UT, ask if they can go in the stdlib.

fn clamp<T:PartialOrd>(x:T, lo:T, hi:T)->T {
	max(min(x,hi),lo)
}
fn clamp_s<T:PartialOrd+Num>(value:T, limit:T)->T {
	clamp(value,-limit,limit)
}
fn deadzone<T:PartialOrd+Zero>(value:T, deadzone:T)->T {
	if value<deadzone || value>deadzone { value }
	else {zero()}
}

pub fn vec2<T:Clone+Zero>(x:T,y:T)->Vec2<T>{ Vec2(x,y) }
pub fn vec3<T:Clone+Zero>(x:T,y:T,z:T)->Vec3<T>{ Vec3::new(x,y,z) }
pub fn vec4<T:Clone+Zero>(x:T,y:T,z:T,w:T)->Vec4<T>{ Vec4::new(x,y,z,w) }


#[cfg(run)]
fn main() {
	io::println("Vec Math Test");
	dump!(Vec3::new(1.0f32,2.0f32,3.0f32)*2.0f32);
	dump!(Vec3::new(1.0f32,2.0f32,3.0f32)*Vec3::new(3.0f32,2.0f32,1.0f32));
	dump!(1,2,3,4);
}




