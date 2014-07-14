#![feature(globs)]
#![allow(unused_attribute)]
#![feature(default_type_params)]
#![feature(macro_rules)]
#![allow(dead_code)]
#![allow(unused_variable)]
#![allow(unreachable_code)]

use std::cmp;
use super::to::To;
pub use std::num;
pub use std::f32::*;
pub use std::num::*;
pub use std::num::Float;
pub use std::num::Float;
use std::io;

/// TODO: Split into XYZW interface+VecTypes,& Vecath dependant on XYZW interfaces
/// Generic maths classes
/// member functions prefixed with 'v' for easier life without code-completion, and to distinguish from operator overloads (official langauge level "add") etc

#[deriving(Clone,Show)]
pub struct Vec2<T=f32>(pub T,pub T);

#[deriving(Clone,Show)]
pub struct Vec3<T=f32>(pub T,pub T,pub T);


#[deriving(Clone,Show)]
pub struct Vec4<T=f32>(pub T,pub T,pub T,pub T);

pub fn fmin<T:PartialOrd>(a:T,b:T)->T { if a<b{a}else{b} }
pub fn fmax<T:PartialOrd>(a:T,b:T)->T { if a>b{a}else{b} }


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
	pub fn splat(v:T)->Vec2<T> { Vec2(v.clone(),v.clone())}
}

impl<T:Clone+Num> Vec2<T> {
	pub fn cross_to_scalar(&self,other:&Vec2<T>)->T {self.x()*other.y()-self.y()*other.x()}
	pub fn cross_z(&self,z:T)->Vec2<T> { Vec2(-self.y()*z,self.x()*z)}
	pub fn cross_one(&self)->Vec2<T> { Vec2(-self.y().clone(),self.x().clone())}
	pub fn map<Y>(&self, f:|T|->Y)->Vec2<Y>{
		let Vec2(ref x,ref y)=*self;
		Vec2(f(x.clone()),f(y.clone()))
	}
}
impl<T:Clone+Zero+One> Vec3<T> {
	pub fn splat(v:T)->Vec3<T> { Vec3(v.clone(),v.clone(),v.clone())}

	pub fn from_vec2(xy:&Vec2<T>,z:T)->Vec3<T> {Vec3::<T>(xy.x().clone(),xy.y().clone(),z.clone())}
	pub fn map<Y>(&self, f:|T|->Y)->Vec3<Y>{
		let Vec3(ref x,ref y,ref z)=*self;
		Vec3(f(x.clone()),f(y.clone()),f(z.clone()))
	}
	pub fn foldr<Y>(&self, src:&Y,f:|&T,&Y|->Y)->Y{
		let Vec3(ref x,ref y,ref z)=*self;
		let f2=f(z,src);
		let f3=f(y,&f2);
		f(x,&f3)
	}
	pub fn foldl<Y>(&self, src:&Y,f:|&T,&Y|->Y)->Y{
		let Vec3(ref x,ref y,ref z)=*self;
		let f1=f(x,src);
		let f2=f(y,&f1);
		f(z,&f2)
	}

}
impl<T:Clone+Zero+One> Vec4<T> {
	pub fn splat(v:T)->Vec4<T> { Vec4(v.clone(),v.clone(),v.clone(),v.clone())}	// todo -move to elsewhere

	pub fn from_vec3(xyz:&Vec3<T>,w:T)->Vec4<T> {Vec4(xyz.x(),xyz.y(),xyz.z(),w.clone())}
	pub fn from_vec2(xy:&Vec2<T>,z:T,w:T)->Vec4<T> {Vec4(xy.x(),xy.y(),z.clone(),w.clone())}
	pub fn from_vec2_vec2(xy:&Vec2<T>,zw:&Vec2<T>)->Vec4<T> {Vec4(xy.x(),xy.y(),zw.x(),zw.y())}

	pub fn map<Y>(&self, f:|T|->Y)->Vec4<Y>{
		let Vec4(ref x,ref y,ref z, ref w)=*self;
		Vec4(f(x.clone()),f(y.clone()),f(z.clone()),f(w.clone()))
	}
	pub fn foldr<Y>(&self, src:&Y,f:|&T,&Y|->Y)->Y{
		let Vec4(ref x,ref y,ref z, ref w)=*self;
		let f1=f(w,src);
		let f2=f(z,&f1);
		let f3=f(y,&f2);
		f(x,&f3)
	}
	pub fn foldl<Y>(&self, src:&Y,f:|&T,&Y|->Y)->Y{
		let Vec4(ref x,ref y,ref z, ref w)=*self;
		let f1=f(x,src);
		let f2=f(y,&f1);
		let f3=f(z,&f2);
		f(w,&f3)
	}
}

pub trait XYZW<T> {
	fn x(&self)->T;
	fn y(&self)->T;
	fn z(&self)->T;
	fn w(&self)->T;
}
pub trait VecXY<T> : XYZW<T>{ fn xy(x:T,y:T)->Self; }
pub trait VecXYZ<T> : XYZW<T>{ fn xyz(x:T,y:T,z:T)->Self; }
pub trait VecXYZW<T> : XYZW<T>{ fn xyzw(x:T,y:T,z:T,w:T)->Self; }

impl<T:Clone+Zero> VecXY<T> for Vec2<T> { fn xy(x:T,y:T)->Vec2<T>{Vec2::<T>(x,y)} }
impl<T:Clone+Zero> VecXYZ<T> for Vec3<T> { fn xyz(x:T,y:T,z:T)->Vec3<T>{Vec3::<T>(x,y,z)} }
impl<T:Clone+Zero> VecXYZW<T> for Vec4<T> { fn xyzw(x:T,y:T,z:T,w:T)->Vec4<T>{Vec4::<T>(x,y,z,w)} }

impl<T:Clone+Zero> VecXYZ<T> for Vec2<T> { fn xyz(x:T,y:T,z:T)->Vec2<T>{Vec2::<T>(x,y)} }
impl<T:Clone+Zero> VecXYZ<T> for Vec4<T> { fn xyz(x:T,y:T,z:T)->Vec4<T>{Vec4::<T>(x,y,z,zero())} }


impl<T:Clone+Zero> VecXY<T> for (T,T) { fn xy(x:T,y:T)->(T,T){(x,y)} }
impl<T:Clone+Zero> VecXYZ<T> for (T,T,T) { fn xyz(x:T,y:T,z:T)->(T,T,T){(x,y,z)} }
impl<T:Clone+Zero> VecXYZW<T> for (T,T,T,T) { fn xyzw(x:T,y:T,z:T,w:T)->(T,T,T,T){(x,y,z,w)} }

impl<T:Clone+Zero> VecXY<T> for [T,..2] { fn xy(x:T,y:T)->[T,..2]{[x,y]} }
impl<T:Clone+Zero> VecXYZ<T> for [T,..3] { fn xyz(x:T,y:T,z:T)->[T,..3]{[x,y,z]} }
impl<T:Clone+Zero> VecXYZW<T> for [T,..4] { fn xyzw(x:T,y:T,z:T,w:T)->[T,..4]{[x,y,z,w]} }

pub trait VecConsts<T:Clone+One+Zero>
{
	fn origin()->Self;
	fn axis(i:int)->Self;
	fn one()->Self;
}

/// 
/// VecPermute relying on type associations created by VXY VXYZ VXYZW
pub trait VecPermute<T:Clone+One+Zero,V2:VecXY<T>, V3:VecXYZ<T>,V4:VecXYZW<T>> : XYZW<T> {
	fn xy(&self)->Vec2<T>	{ Vec2(self.x(),self.y())}
	fn yx(&self)->Vec2<T>	{ Vec2(self.y(),self.x())}
	fn xz(&self)->Vec2<T>	{ Vec2(self.x(),self.z())}
	fn yz(&self)->Vec2<T>	{ Vec2(self.y(),self.z())}

	fn xy01(&self)->V4	{ VecXYZW::xyzw(self.x(),self.y(),zero(),one())}

	fn xyz(&self)->V3	{ VecXYZ::xyz(self.x(),self.y(),self.z())}
	fn xyz1(&self)->Vec4<T>	{ Vec4(self.x(),self.y(),self.z(),one())}	// vec3 to homogeneous point
	fn xyz0(&self)->Vec4<T>	{ Vec4(self.x(),self.y(),self.z(),zero())}	// vec3 to homogeneous offset
	fn xyzw(&self)->Vec4<T>	{ Vec4(self.x(),self.y(),self.z(),self.w())}

	// permutes useful for swapping rgb/bgr
	fn zyx(&self)->V3	{ VecXYZ::xyz(self.z(),self.y(),self.x())}
	fn zyx0(&self)->Vec4<T>	{ Vec4(self.z(),self.y(),self.x(),zero())}
	fn zyx1(&self)->Vec4<T>	{ Vec4(self.z(),self.y(),self.x(),one())}

	// permutes for swapping Y or Z being up
	fn xzy(&self)->V3	{ VecXYZ::xyz(self.x(),self.z(),self.y())}
	fn xzy0(&self)->Vec4<T>	{ Vec4(self.x(),self.z(),self.y(),zero())}
	fn xzy1(&self)->Vec4<T>	{ Vec4(self.x(),self.z(),self.y(),one())}
	// permutes for cross-product
	// i  j   k
	// x0 y0 z0
	// x1 y1 z1

	// x'=y0*z1 - z0*y1 ,  y'=z0*x1-x0*z1,  z'=x0*y1-y0*x1
	fn yzx(&self)->V3 {VecXYZ::xyz(self.y(),self.z(),self.x())}
	fn zxy(&self)->V3 {VecXYZ::xyz(self.z(),self.x(),self.y())}
	fn yzxw(&self)->Vec4<T> {Vec4(self.y(),self.z(),self.x(),self.w())}
	fn zxyw(&self)->Vec4<T> {Vec4(self.z(),self.x(),self.y(),self.w())}
	fn yzx0(&self)->Vec4<T> {Vec4(self.y(),self.z(),self.x(),zero())}
	fn zxy0(&self)->Vec4<T> {Vec4(self.z(),self.x(),self.y(),zero())}

	// splats in permute syntax
	fn xxxx(&self)->Vec4<T>	{ Vec4(self.x(),self.x(),self.x(),self.x())}
	fn yyyy(&self)->Vec4<T>	{ Vec4(self.y(),self.y(),self.y(),self.y())}
	fn zzzz(&self)->Vec4<T>	{ Vec4(self.z(),self.z(),self.z(),self.z())}
	fn wwww(&self)->Vec4<T>	{ Vec4(self.w(),self.w(),self.w(),self.w())}

	// Synonyms
	fn to_vec4_w1(&self,w:T)->Vec4<T> { self.xyz1()}
	fn to_vec4_w0(&self,w:T)->Vec4<T> { self.xyz0()}
	fn to_vec4(&self)->Vec4<T> { self.xyzw()}
	fn to_vec3(&self)->V3 { self.xyz()}
	fn to_vec2(&self)->Vec2<T> { self.xy()}
}

//pub trait VecNum<T:Num> {
//	fn from_xyz(x:T,y:T,z:T)->Self;
//}
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
impl<T:Float> PreMulFloat<T> for Vec2<T> {}
impl<T:Float> PreMulFloat<T> for Vec3<T> {}
impl<T:Float> PreMulFloat<T> for Vec4<T> {}
// todo- only possible if impl other trait/other type restriction is lifted.
//impl Mul<Vec3<f32>,Vec3<f32>> for f32 { fn mul(&self,v:&Vec3<f32>)->Vec3<f32> { v.pre_mul_float(self)}}

// componentwise multiplication operator for vectors
impl<F:Mul<F,F>+Clone+Zero> Mul<Vec2<F>,Vec2<F>> for Vec2<F> {
	fn mul(&self,b:&Vec2<F>)->Vec2<F> {
		Vec2(self.x()*b.x(),self.y()*b.y())
	}
}
impl<F:Div<F,F>+Clone+Clone+Zero> Div<Vec2<F>,Vec2<F>> for Vec2<F> {
	fn div(&self,b:&Vec2<F>)->Vec2<F> {
		Vec2(self.x()/b.x(),self.y()/b.y())
	}
}
impl<F:Div<F,F>+Clone+Clone+Zero> Div<Vec3<F>,Vec3<F>> for Vec3<F> {
	fn div(&self,b:&Vec3<F>)->Vec3<F> {
		Vec3(self.x()/b.x(),self.y()/b.y(),self.z()/b.z())
	}
}
impl<F:Div<F,F>+Clone+Clone+Zero> Div<Vec4<F>,Vec4<F>> for Vec4<F> {
	fn div(&self,b:&Vec4<F>)->Vec4<F> {
		Vec4(self.x()/b.x(),self.y()/b.y(),self.z()/b.z(),self.w()/b.w())
	}
}


pub trait PreMulVec2<T,RESULT> {
	fn pre_mul_vec2(&self,&Vec2<T>)->RESULT;
}
impl<T:Mul<T,T>+Clone+Zero+One> PreMulVec2<T,Vec2<T>> for Vec2<T> {
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
pub trait PreDivVec2<T,RESULT> {
	fn pre_div_vec2(&self,&Vec2<T>)->RESULT;
}
impl<T:Div<T,T>+Clone+Zero+One> PreDivVec2<T,Vec2<T>> for Vec2<T> {
	fn pre_div_vec2(&self, lhs:&Vec2<T>)->Vec2<T> { Vec2(lhs.x()/self.x(),lhs.y()/self.y()) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreDivVec2<f32,Vec2<f32>> for f32 {
	fn pre_div_vec2(&self, lhs:&Vec2<f32>)->Vec2<f32> { Vec2(lhs.x()/ *self,lhs.y()/ *self) }
}

impl<T:Rem<T,T>+Clone+Zero> Rem<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn rem(&self,rhs:&Vec2<T>)->Vec2<T> {
		Vec2(self.x()%rhs.x(),self.y()%rhs.y())
	}
}
impl<T:Rem<T,T>+Clone+Zero> Rem<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn rem(&self,rhs:&Vec3<T>)->Vec3<T> {
		Vec3(self.x()%rhs.x(),self.y()%rhs.y(),self.z()%rhs.z())
	}
}
impl<T:Rem<T,T>+Clone+Zero> Rem<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn rem(&self,rhs:&Vec4<T>)->Vec4<T> {
		Vec4(self.x()%rhs.x(),self.y()%rhs.y(),self.z()%rhs.z(),self.w()%rhs.w())
	}
}

impl<T:Neg<T>+Clone+Zero> Neg<Vec2<T>> for Vec2<T> {
	fn neg(&self)->Vec2<T> { Vec2(-self.x(),-self.y()) }
}
impl<T:Neg<T>+Clone+Zero> Neg<Vec3<T>> for Vec3<T> {
	fn neg(&self)->Vec3<T> { Vec3(-self.x(),-self.y(),-self.z()) }
}
impl<T:Neg<T>+Clone+Zero> Neg<Vec4<T>> for Vec4<T> {
	fn neg(&self)->Vec4<T> { Vec4(-self.x(),-self.y(),-self.z(),-self.w()) }
}

impl<T,OUT,RHS:PreMulVec3<T,OUT>> Mul<RHS,OUT> for Vec3<T> {
	fn mul(&self,b:&RHS)->OUT {
		b.pre_mul_vec3(self)
	}
}
impl<F:Mul<F,F>+Clone,OUT, RHS:PreMulVec4<F,OUT>> Mul<RHS,OUT> for Vec4<F> {
	fn mul(&self,b:&RHS)->OUT {
		b.pre_mul_vec4(self)
	}
}

pub trait Cross<T>{
	fn cross(&self,&Self)->Self;
}

pub trait SumElems<T> {
	fn sum_elems(&self)->T;
}



pub trait PreMulVec3<T,RESULT> {
	fn pre_mul_vec3(&self,&Vec3<T>)->RESULT;
}
impl<T:Float+Clone> PreMulVec3<T,Vec3<T>> for Vec3<T> {
	fn pre_mul_vec3(&self, lhs:&Vec3<T>)->Vec3<T> { Vec3(lhs.x()*self.x(),lhs.y()*self.y(),lhs.z()*self.z()) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec3<f64,Vec3<f64>> for f64 {
	fn pre_mul_vec3(&self/*=rhs*/, lhs:&Vec3<f64>)->Vec3<f64> { Vec3(lhs.x()**self,lhs.y()**self,lhs.z()**self) }
}
impl PreMulVec3<f32,Vec3<f32>> for f32 {
	fn pre_mul_vec3(&self, lhs:&Vec3<f32>)->Vec3<f32> { Vec3(lhs.x()**self,lhs.y()**self,lhs.z()**self) }
}

pub trait PreDivVec3<T,RESULT> {
	fn pre_div_vec3(&self,&Vec3<T>)->RESULT;
}
impl<T:Float+Clone> PreDivVec3<T,Vec3<T>> for Vec3<T> {
	fn pre_div_vec3(&self, lhs:&Vec3<T>)->Vec3<T> { Vec3(lhs.x()/self.x(),lhs.y()/self.y(),lhs.z()/self.z()) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreDivVec3<f32,Vec3<f32>> for f32 {
	fn pre_div_vec3(&self, lhs:&Vec3<f32>)->Vec3<f32> { Vec3(lhs.x()/ *self,lhs.y()/ *self,lhs.z()/ *self) }
}



pub trait PreMulVec4<T,RESULT> {
	fn pre_mul_vec4(&self,lhs:&Vec4<T>)->RESULT;
}
impl<T:Float+Clone> PreMulVec4<T,Vec4<T>> for Vec4<T> {
	fn pre_mul_vec4(&self, lhs:&Vec4<T>)->Vec4<T> { Vec4(lhs.x()*self.x(),lhs.y()*self.y(),lhs.z()*self.z(),lhs.w()*self.w()) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec4<f32,Vec4<f32>> for f32 {
	fn pre_mul_vec4(&self, lhs:&Vec4<f32>)->Vec4<f32> { Vec4(lhs.x()**self,lhs.y()**self,lhs.z()**self,lhs.w()**self) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec4<f64,Vec4<f64>> for f64 {
	fn pre_mul_vec4(&self, lhs:&Vec4<f64>)->Vec4<f64> { Vec4(lhs.x()**self,lhs.y()**self,lhs.z()**self,lhs.w()**self) }
}

pub trait PreDivVec4<T,RESULT> {
	fn pre_div_vec4(&self,lhs:&Vec4<T>)->RESULT;
}
impl<T:Div<T,T>+Clone> PreDivVec4<T,Vec4<T>> for Vec4<T> {
	fn pre_div_vec4(&self, lhs:&Vec4<T>)->Vec4<T> { Vec4(lhs.x()/self.x(),lhs.y()/self.y(),lhs.z()/self.z(),lhs.w()/self.w()) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreDivVec4<f32,Vec4<f32>> for f32 {
	fn pre_div_vec4(&self, lhs:&Vec4<f32>)->Vec4<f32> { Vec4(lhs.x()/ *self,lhs.y()/ *self,lhs.z()/ *self,lhs.w()/ *self) }
}

impl<T:PartialEq+Clone+Zero> PartialEq for Vec2<T> {
	fn eq(&self,rhs:&Vec2<T>)->bool { return self.x()==rhs.x() && self.y()==rhs.y() }
}
impl<T:PartialEq+Clone+Zero> PartialEq for Vec3<T> {
	fn eq(&self,rhs:&Vec3<T>)->bool { return self.x()==rhs.x() && self.y()==rhs.y() && self.z()==rhs.z() }
}
impl<T:PartialEq+Clone+Zero> PartialEq for Vec4<T> {
	fn eq(&self,rhs:&Vec4<T>)->bool { return self.x()==rhs.x() && self.y()==rhs.y() && self.z()==rhs.z() && self.w()==rhs.w() }
}

// todo: satisfy if Num+Clone only
impl<T:Num+Clone+One+Float> Num for Vec2<T> {}
impl<T:Num+Clone+One+Float> Num for Vec3<T> {}
impl<T:Num+Clone+One+Float> Num for Vec4<T> {}

// vector maths gathers primitive operations and implements more in terms of them
pub trait VecMath<T:Float=f32>:Clone+XYZW<T>+VecXYZ<T>+Num+VecConsts<T>+VecCmp<T>+Scale<T>+Cross<T>+SumElems<T> {
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
impl<T:Float,V2:VecXY<T>,V3:VecXYZ<T>,V4:VecXYZW<T>, V:Clone+XYZW<T>+VecCmp<T>+VecXYZ<T>+Num+VecPermute<T,V2,V3,V4>+VecConsts<T>+Scale<T>+Mul<V,V>+Cross<T>+SumElems<T>> VecMath<T> for V {} 

//todo: HALF
fn bilerp<F:Float,V:VecMath<F>>(((v00,v01),(v10,v11)):((V,V),(V,V)),(s,t):(F,F))->V{
	(v00.lerp(&v01,s)).lerp(&v10.lerp(&v10,s), t)
}


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
impl<T:Add<T,T>+Clone+Zero+One> Add<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn add(&self,rhs:&Vec2<T>)->Vec2<T> { 
		Vec2(self.x()+rhs.x(), self.y()+rhs.y())
	}
}

impl<T:Add<T,T>+Clone+Zero> Add<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn add(&self,rhs:&Vec3<T>)->Vec3<T> { 
		Vec3(self.x()+rhs.x()   , self.y()+rhs.y(), self.z()+rhs.z())
	}
}

impl<T:Add<T,T>+Clone+Zero> Add<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn add(&self,rhs:&Vec4<T>)->Vec4<T> { 
		Vec4(self.x()+rhs.x()   , self.y()+rhs.y(), self.z()+rhs.z(), self.w()+rhs.w())
	}
}
//  wtf this does,t work now
impl<T:Sub<T,T>+Clone+Zero+One> Sub<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn sub(&self,rhs:&Vec2<T>)->Vec2<T> { 
		Vec2(self.x()-rhs.x(), self.y()-rhs.y())
	}
}
impl<T:Sub<T,T>+Clone+Zero+One> Sub<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn sub(&self,rhs:&Vec3<T>)->Vec3<T> { 
		Vec3(self.x()-rhs.x()   , self.y()-rhs.y(), self.z()-rhs.z())
	}
}
impl<T:Sub<T,T>+Clone+Zero+One> Sub<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn sub(&self,rhs:&Vec4<T>)->Vec4<T> { 
		Vec4(self.x()-rhs.x()   , self.y()-rhs.y(), self.z()-rhs.z(), self.w()-rhs.w())
	}
}


impl<T:Zero+Clone+One> Zero for Vec2<T> {
	fn zero()->Vec2<T> {Vec2(zero::<T>(),zero::<T>())}
	fn is_zero(&self)->bool { fail!();false/*self.x==zero::<T>() && self.y==zero::<T>() */}
}
fn vec_axis_scale<T:Float,V:VecConsts<T>+VecMath<T>>(i:int,f:T)->V { let ret:V; ret=VecConsts::axis(i); ret.scale(f) }

impl<T:Clone+One+Zero> VecPermute<T,Vec2<T>,Vec3<T>,Vec4<T>> for Vec2<T> {
}
//impl<T:Num+Clone> VecNum<T> for Vec2<T> {
//	fn from_xyz(x:T,y:T,_:T)->Vec2<T>{Vec2(x.clone(),y.clone())}
//}
impl<T:PartialOrd+Clone+Zero+One> VecCmp<T> for Vec2<T> {
	fn min(&self,b:&Vec2<T>)->Vec2<T>	{Vec2(fmin(self.x(),b.x()),fmin(self.y(),b.y()))}
	fn max(&self,b:&Vec2<T>)->Vec2<T>	{Vec2(fmax(self.x(),b.x()),fmax(self.y(),b.y()))}
	
}
impl<T:Float> Scale<T> for Vec2<T> {
	fn scale(&self,f:T)->Vec2<T>		{Vec2(self.x()*f,self.y()*f)}
}

impl<T:Float+Zero+Clone> SumElems<T> for Vec2<T> {
	fn sum_elems(&self)->T	{self.x()+self.y()}
}

impl<T:Float> Cross<T> for Vec2<T> {
	fn cross(&self,_:&Vec2<T>)->Vec2<T>{Vec2(zero::<T>(),zero::<T>())}
}

impl<T> Vec2<T> {
	pub fn ref0<'a>(&'a self)->&'a T { let Vec2(ref x,ref y)=*self; x}
	pub fn ref1<'a>(&'a self)->&'a T { let Vec2(ref x,ref y)=*self; y}
}

impl<T:Clone+Zero> XYZW<T> for Vec2<T> {
	fn x(&self)->T	{ self.ref0().clone()}
	fn y(&self)->T	{ self.ref1().clone()}
	fn z(&self)->T	{ zero::<T>()}
	fn w(&self)->T	{ zero::<T>()}
}

impl<T:Clone+Zero> Zero for Vec3<T> {
	fn zero()->Vec3<T>{Vec3(zero(),zero(),zero())}
	fn is_zero(&self)->bool  {fail!(); false/*self.x.is_zero() && self.y.is_zero() && self.z.is_zero()*/}
}

impl<T:Clone+One+Zero+Float> One for Vec2<T> {
	fn one()->Vec2<T>{Vec2(one(),one())}
}
impl<T:Clone+One+Zero+Float> One for Vec3<T> {
	fn one()->Vec3<T>{Vec3(one(),one(),one())}
}
impl<T:Clone+One+Zero+Float> One for Vec4<T> {
	fn one()->Vec4<T>{Vec4(one(),one(),one(),one())}
}

impl<T:Clone+Zero+One> VecConsts<T> for Vec3<T> {
	fn one()->Vec3<T>	{Vec3(one(),one(),one())}
	fn origin()->Vec3<T>	{Vec3(zero(),zero(),zero())}
	fn axis(i:int)->Vec3<T>{
		match i{ 0=>Vec3(one(),zero(),zero()),
                1=>Vec3(zero(),one(),zero()),
                2=>Vec3(zero(),zero(),one()),
                _=>Vec3(zero(),zero(),zero())}
	}
}
impl<T:Clone+Zero+One> VecConsts<T> for Vec2<T> {
	fn one()->Vec2<T>	{Vec2(one(),one())}
	fn origin()->Vec2<T>	{Vec2(zero(),zero())}
	fn axis(i:int)->Vec2<T>{
		match i{ 0=>Vec2(one::<T>(),zero::<T>()),1=>Vec2(zero(),one()),_=>Vec2(zero(),zero())}
	}
}



impl<T:Clone+One+Zero> VecPermute<T,Vec2<T>,Vec3<T>,Vec4<T>> for Vec3<T> {}

// permutation of tuples.
impl<T:Clone+One+Zero> VecPermute<T,(T,T),(T,T,T),(T,T,T,T)> for (T,T) {}
impl<T:Clone+One+Zero> VecPermute<T,(T,T),(T,T,T),(T,T,T,T)> for (T,T,T) {}
impl<T:Clone+One+Zero> VecPermute<T,(T,T),(T,T,T),(T,T,T,T)> for (T,T,T,T) {}

//impl<T:Clone+Num> VecNum<T> for Vec3<T> {/
//	fn from_xyz(x:T,y:T,z:T)->Vec3<T>{Vec3(x,y,z)}
//}
impl<T:Clone+PartialOrd+Zero> VecCmp<T> for Vec3<T> {
	fn min(&self,b:&Vec3<T>)->Vec3<T>	{
		let x=fmin(self.x(), b.x());
		let y=fmin(self.y(), b.y());
		let z=fmin(self.z(), b.z());
		Vec3(x,y,z)}
	fn max(&self,b:&Vec3<T>)->Vec3<T>	{Vec3(
									fmax(self.x(),b.x()),
									fmax(self.y(),b.y()),
									fmax(self.z(),b.z()))}
}

impl<T:Float> Scale<T> for Vec3<T> { 
	fn scale(&self,f:T)->Vec3<T>		{Vec3(self.x()*f,self.y()*f,self.z()*f)}
}

impl<T:Float> SumElems<T> for Vec3<T> {
	fn sum_elems(&self)->T	{self.x()+self.y()+self.z()}
}

impl<T:Float> Cross<T> for Vec3<T> {
	fn cross(&self,b:&Vec3<T>)->Vec3<T>	{Vec3(self.y()*b.z()-self.z()*b.y(),self.z()*b.x()-self.x()*b.z(),self.x()*b.y()-self.y()*b.x())}

//	fn cross_to_vec3(&self,b:&Vec3<T>)->Vec3<T>	{self.cross(b)}
	//fpub fn axisScale(i:int,f:VScalar)->Vec3 { VecConsts::axis(i).scale(f)} 
}
impl<T>  Vec3<T> {
	pub fn ref0<'a>(&'a self)->&'a T { let Vec3(ref x,ref y,ref z)=*self; x}
	pub fn ref1<'a>(&'a self)->&'a T { let Vec3(ref x,ref y,ref z)=*self; y}
	pub fn ref2<'a>(&'a self)->&'a T { let Vec3(ref x,ref y,ref z)=*self; z}
}
impl<T:Clone+Zero> XYZW<T> for Vec3<T> {
	fn x(&self)->T	{ self.ref0().clone()}
	fn y(&self)->T	{ self.ref1().clone()}
	fn z(&self)->T	{ self.ref2().clone()}
	fn w(&self)->T	{ zero()}
}



impl<T:Clone+Zero> Zero for Vec4<T> {
	fn zero()->Vec4<T>{Vec4(zero(),zero(),zero(),zero())}
	fn is_zero(&self)->bool  {fail!();false/*self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero()*/}
}
impl<T:Clone+Zero+One> VecConsts<T> for Vec4<T> {
	fn one()->Vec4<T>	{Vec4(one(),one(),one(),one())}
	fn origin()->Vec4<T>	{Vec4(zero(),zero(),zero(),one())}

	fn axis(i:int)->Vec4<T>{
		match i{
            0=>Vec4(one(),zero(),zero(),zero()),
            1=>Vec4(zero(),one(),zero(),zero()),
            2=>Vec4(zero(),zero(),one(),zero()),
            3=>Vec4(zero(),zero(),zero(),one()),
            _=>Vec4(zero(),zero(),zero(),zero())
        }
	}

}

// Converting Vec2,Vec3,Vec4 to/from tuples & arrays

impl<T:Clone+Zero> XYZW<T> for [T,..2] {
	fn x(&self)->T { self[0].clone() }
	fn y(&self)->T { self[1].clone() }
	fn z(&self)->T { zero() }
	fn w(&self)->T { zero() }
}
impl<T:Clone+Zero> XYZW<T> for [T,..3] {
	fn x(&self)->T { self[0].clone() }
	fn y(&self)->T { self[1].clone() }
	fn z(&self)->T { self[2].clone() }
	fn w(&self)->T { zero() }
}
impl<T:Clone+Zero> XYZW<T> for [T,..4] {
	fn x(&self)->T { self[0].clone() }
	fn y(&self)->T { self[1].clone() }
	fn z(&self)->T { self[2].clone() }
	fn w(&self)->T { self[3].clone() }
}

impl<T:Clone+Zero> XYZW<T> for (T,T) {
	fn x(&self)->T { let(ref v,_,)=*self; v.clone() }
	fn y(&self)->T { let(_,ref v,)=*self;v.clone() }
	fn z(&self)->T { zero() }
	fn w(&self)->T { zero() }
}
impl<T:Clone+Zero> XYZW<T> for (T,T,T) {
	fn x(&self)->T { let(ref v,_,_)=*self; v.clone() }
	fn y(&self)->T { let(_,ref v,_)=*self;v.clone() }
	fn z(&self)->T { let(_,_,ref v)=*self;v.clone() }
	fn w(&self)->T { zero() }
}
impl<T:Clone> XYZW<T> for (T,T,T,T) {
	fn x(&self)->T { let(ref v,_,_,_)=*self; v.clone() }
	fn y(&self)->T { let(_,ref v,_,_)=*self;v.clone() }
	fn z(&self)->T { let(_,_,ref v,_)=*self;v.clone() }
	fn w(&self)->T { let(_,_,_,ref v)=*self;v.clone() }
}

impl<T:Clone> Vec4<T> {
	pub fn to_array(&self)->[T,..4] { [self.x(),self.y(),self.z(),self.w()] }
	pub fn from_array([x,y,z,w]:[T,..4])->Vec4<T> {Vec4(x.clone(),y.clone(),z.clone(),w.clone())}
	pub fn to_tuple(&self)->(T,T,T,T) { (self.x(),self.y(),self.z(),self.w()) }
	pub fn from_tuple((x,y,z,w):(T,T,T,T))->Vec4<T> {
		Vec4(x.clone(),y.clone(),z.clone(),w.clone())
	}
}
impl<T:Clone+Zero> Vec3<T> {
	pub fn to_array(&self)->[T,..3] { [self.x(),self.y(),self.z()] }
	pub fn from_array([x,y,z]:[T,..3])->Vec3<T> {Vec3(x.clone(),y.clone(),z.clone())}
	fn to_tuple(&self)->(T,T,T) { (self.x(),self.y(),self.z()) }
	fn from_tuple((x,y,z):(T,T,T))->Vec3<T> { Vec3(x.clone(),y.clone(),z.clone()) }
}
impl<T:Clone+Zero> Vec2<T> {
	pub fn to_array(&self)->[T,..2] { [self.x(),self.y()] }
	pub fn from_array([x,y]:[T,..2])->Vec2<T> {Vec2(x.clone(),y.clone())}
	fn to_tuple(&self)->(T,T) { (self.x(),self.y()) }
	fn from_tuple((x,y):(T,T))->Vec2<T> { Vec2(x,y) }
}

impl<T:Clone+One+Zero> VecPermute<T,Vec2<T>,Vec3<T>,Vec4<T>> for Vec4<T> {}
impl<T:Clone+One+Zero> VecPermute<T,[T,..2],[T,..3],[T,..4]> for [T,..4] {}

impl<T:Clone+PartialOrd+Zero> VecCmp<T> for Vec4<T> {
	fn min(&self,b:&Vec4<T>)->Vec4<T>	{Vec4(
									fmin(self.x(),b.x()),
									fmin(self.y(),b.y()),
									fmin(self.z(),b.z()),
									fmin(self.w(),b.w()))}
	fn max(&self,b:&Vec4<T>)->Vec4<T>	{Vec4(
									fmax(self.x(),b.x()),
									fmax(self.y(),b.y()),
									fmax(self.z(),b.z()),
									fmax(self.w(),b.w()))}
}

//impl<T:Clone+Num> VecNum<T> for Vec4<T> {
//	fn from_xyz(x:T,y:T,z:T)->Vec4<T>{Vec4(x,y,z,zero::<T>())}
//}


impl<T:Num+Clone> Scale<T> for Vec4<T> {
	fn scale(&self,f:T)->Vec4<T>		{Vec4(self.x()*f,self.y()*f,self.z()*f,self.w()*f)}
}

impl<T:Num+Clone> SumElems<T> for Vec4<T> {
	fn sum_elems(&self)->T	{self.x()+self.y()+self.z()+self.w()}
}

impl<T:Num+Clone> Cross<T> for Vec4<T> {
	fn cross(&self,b:&Vec4<T>)->Vec4<T>	{Vec4(self.y()*b.z()-self.z()*b.y(),self.z()*b.x()-self.x()*b.z(),self.x()*b.y()-self.y()*b.x(),zero::<T>())}
}
impl<T> Vec4<T> {
	pub fn ref0<'a>(&'a self)->&'a T { let Vec4(ref x,ref y,ref z,ref w)=*self; x}
	pub fn ref1<'a>(&'a self)->&'a T { let Vec4(ref x,ref y,ref z,ref w)=*self; y}
	pub fn ref2<'a>(&'a self)->&'a T { let Vec4(ref x,ref y,ref z,ref w)=*self; z}
	pub fn ref3<'a>(&'a self)->&'a T { let Vec4(ref x,ref y,ref z,ref w)=*self; w}
}
impl<T:Clone> XYZW<T> for Vec4<T>
{
	fn x(&self)->T {self.ref0().clone()}
	fn y(&self)->T {self.ref1().clone()}
	fn z(&self)->T {self.ref2().clone()}
	fn w(&self)->T {self.ref3().clone()}
}
pub fn vec_normalize<T:Float,V:VecMath<T>>(v:&V)->V { v.scale(v.sqr().rsqrt()) }
// TOOD - Zero for Vec3<T> , Vec4<T>



// todo - math UT, ask if they can go in the stdlib.

fn clamp<T:PartialOrd>(x:T, lo:T, hi:T)->T {
	fmax(fmin(x,hi),lo)
}
fn clamp_s<T:PartialOrd+Num>(value:T, limit:T)->T {
	clamp(value,-limit,limit)
}
fn deadzone<T:PartialOrd+Zero>(value:T, deadzone:T)->T {
	if value<deadzone || value>deadzone { value }
	else {zero()}
}

trait ToVec2<T> {
	fn to_vec2(&self)->Vec2<T>;
}
trait ToVec3<T> {
	fn to_vec3(&self)->Vec3<T>;
}
trait ToVec4<T> {
	fn to_vec4(&self)->Vec4<T>;
}

impl<T:Clone+Zero> ToVec2<T> for (T,T){
	fn to_vec2(&self)->Vec2<T>{Vec2(self.ref0().clone(),self.ref1().clone())}
}
impl<T:Clone+Zero> ToVec3<T> for (T,T,T){
	fn to_vec3(&self)->Vec3<T>{Vec3(self.ref0().clone(),self.ref1().clone(),self.ref2().clone())}
}
impl<T:Clone+Zero> ToVec4<T> for (T,T,T,T){
	fn to_vec4(&self)->Vec4<T>{Vec4(self.ref0().clone(),self.ref1().clone(),self.ref2().clone(),self.ref3().clone())}
}

impl<T:Clone+Zero> ToVec2<T> for [T,..2]{
	fn to_vec2(&self)->Vec2<T>{Vec2(self[0].clone(),self[1].clone())}
}
impl<T:Clone+Zero> ToVec3<T> for [T,..3]{
	fn to_vec3(&self)->Vec3<T>{Vec3(self[0].clone(),self[1].clone(),self[2].clone())}
}
impl<T:Clone+Zero> ToVec4<T> for [T,..4]{
	fn to_vec4(&self)->Vec4<T>{Vec4(self[0].clone(),self[1].clone(),self[2].clone(),self[3].clone())}
}

// Componentwise conversion for vector
impl<A:To<B>+Clone+Zero, B:Clone+Zero> To<Vec3<B>> for Vec3<A> {
	fn to(&self)->Vec3<B> { Vec3( self.x().to(),	self.y().to(),  self.z().to() )}
}
impl<A:To<B>+Clone+Zero, B:Clone+Zero> To<Vec4<B>> for Vec4<A> {
	fn to(&self)->Vec4<B> { Vec4( self.x().to(),	self.y().to(),  self.z().to(), self.w().to() )}
}

#[cfg(run)]
fn main() {
	io::println("Vec Math Test");
	dump!(Vec3(1.0f32,2.0f32,3.0f32)*2.0f32);
	dump!(Vec3(1.0f32,2.0f32,3.0f32)*Vec3(3.0f32,2.0f32,1.0f32));
	dump!(1i,2i,3i,4i);
	let x:Vec4<i32>= Vec4(0u32,1u32,2u32,3u32).to(); 
	dump!(x);
}




