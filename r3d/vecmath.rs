#![feature(globs)]
#![feature(tuple_indexing)]
#![allow(unused_attribute)]
#![feature(default_type_params)]
#![feature(macro_rules)]
#![allow(dead_code)]
#![allow(unused_variable)]
#![allow(unreachable_code)]

use super::to::*;
pub use super::common::*;

/// TODO: Split into XYZW interface+VecTypes,& Vecath dependant on XYZW interfaces
/// Generic maths classes
/// member functions prefixed with 'v' for easier life without code-completion, and to distinguish from operator overloads (official langauge level "add") etc
// Screen
#[deriving(Copy,Clone,Show)]
pub struct Vec2<T=f32>(pub T,pub T);

#[deriving(Copy,Clone,Show)]
pub struct Vec3<T=f32>(pub T,pub T,pub T);


#[deriving(Copy,Clone,Show)]
pub struct Vec4<T=f32>(pub T,pub T,pub T,pub T);

pub fn fmin<T:PartialOrd>(a:T,b:T)->T { if a<b{a}else{b} }
pub fn fmax<T:PartialOrd>(a:T,b:T)->T { if a>b{a}else{b} }

// 'Position' trait for  objects with a spatial centre.
// position should be an x,y,z or x,y,z,1
pub trait Pos<V=Vec3<f32>> {
	fn pos(&self)->V;
	fn set_pos(&mut self,v:&V) {fail!();}
}




//	type V2=Vec2<T>;
// TODO half-precision type for GL..
// TODO: Packed normal 10:10:10
// TODO: 565 colors

impl<T:Copy+Num> Vec2<T> {
	pub fn cross_to_scalar(&self,other:&Vec2<T>)->T {self.0*other.1-self.1*other.0}
	pub fn cross_z(&self,z:T)->Vec2<T> { Vec2(-self.1*z,self.0*z)}
	pub fn cross_one(&self)->Vec2<T> { Vec2(-self.1,self.0)}
	pub fn map<R>(&self, f:|&T|->R)->Vec2<R>{
		Vec2(f(&self.0),f(&self.1))
	}
	pub fn zip<X:Copy,R:Copy>(&self,&other:&Vec2<X>,f:|&T,&X|->R)->Vec2<R>{
		Vec2(f(&self.0,&other.0),f(&self.1,&other.1))
	}
	pub fn fold<R:Copy>(&self, src:R,f:|R,T|->R)->R{
		let Vec2(x,y)=*self;
		let f2=f(src,x);
		f(f2,y)
	}
}

impl<T:Copy+Zero+One> Vec3<T> {

	pub fn from_vec2(xy:&Vec2<T>,z:T)->Vec3<T> {Vec3::<T>(xy.0,xy.1,z)}

	pub fn map<R:Copy>(&self, f:|&T|->R)->Vec3<R>{
//		let Vec3(x,y,z)=*self;
		Vec3(f(&self.0),f(&self.1),f(&self.2))
	}
	pub fn zip<X:Copy,R:Copy>(&self,&Vec3(ref x1,ref y1,ref z1):&Vec3<X>,f:|&T,&X|->R)->Vec3<R>{
		Vec3(f(&self.0,x1),f(&self.1,y1),f(&self.2,z1))
	}
	pub fn fold<R:Copy>(&self, src:R,f:|R,T|->R)->R{
		let f2=f(src,self.0);
		let f3=f(f2,self.1);
		f(f3,self.2)
	}
}
impl<T:Copy+Zero+One> Vec4<T> {

	pub fn from_vec3(xyz:&Vec3<T>,w:T)->Vec4<T> {Vec4(xyz.0,xyz.1,xyz.2,w)}
	pub fn from_vec2(xy:&Vec2<T>,z:T,w:T)->Vec4<T> {Vec4(xy.0,xy.1,z,w)}
	pub fn from_vec2_vec2(xy:&Vec2<T>,zw:&Vec2<T>)->Vec4<T> {Vec4(xy.0,xy.1,zw.0,zw.1)}

	pub fn map<R:Copy>(&self, f:|&T|->R)->Vec4<R>{
		Vec4(f(&self.0),f(&self.1),f(&self.2),f(&self.3))
	}
	pub fn zip<X:Copy,R:Copy>(&self,other:&Vec4<X>,f:|T,X|->R)->Vec4<R>{
		Vec4(f(self.0,other.0),f(self.1,other.1),f(self.2,other.2), f(self.3,other.3))
	}
	pub fn fold<R>(&self, src:R,f:|R,T|->R)->R{
		let f2=f(src,self.0);
		let f3=f(f2,self.1);
		let f4=f(f3,self.2);
		f(f4,self.3)
	}
}
pub type V2<T> =Vec2<T>;
pub type V3<T> =Vec3<T>;
pub type V4<T> =Vec4<T>;
 
pub trait Vector<T:Zero+One+Copy=f32> :Copy{
//	type V2;
//	type V3;
//	type V4;
	fn x(&self)->T;
	fn y(&self)->T;
	fn z(&self)->T;
	fn w(&self)->T;

	fn from_xyzw(x:T,y:T,z:T,w:T)->Self;
	fn from_xyz(x:T,y:T,z:T)->Self { Vector::<T>::from_xyzw(x,y,z,zero::<T>()) }
	fn from_xyz1(x:T,y:T,z:T)->Self { Vector::<T>::from_xyzw(x,y,z,one::<T>()) }
	fn from_xy(x:T,y:T)->Self { Vector::<T>::from_xyzw(x,y,zero::<T>(),zero::<T>()) }
	fn xyzw(&self)->(T,T,T,T) {(self.x(),self.y(),self.z(),self.w())}

	fn splat(f:T)->Self { Vector::<T>::from_xyzw(f,f,f,f)}
	fn splat_x(&self)->Self { Vector::<T>::splat(self.x())}
	fn splat_y(&self)->Self { Vector::<T>::splat(self.y())}
	fn splat_z(&self)->Self { Vector::<T>::splat(self.z())}
	fn splat_w(&self)->Self { Vector::<T>::splat(self.w())}

	fn set_x(&self,f:T)->Self {Vector::<T>::from_xyzw(f,self.y(),self.z(),self.w())}
	fn set_y(&self,f:T)->Self {Vector::<T>::from_xyzw(self.x(),f,self.z(),self.w())}
	fn set_z(&self,f:T)->Self {Vector::<T>::from_xyzw(self.x(),self.y(),f,self.w())}
	fn set_w(&self,f:T)->Self {Vector::<T>::from_xyzw(self.x(),self.y(),self.z(),f)}
	fn set_w1(&self,f:T)->Self {Vector::<T>::from_xyzw(self.x(),self.y(),self.z(),one())}
	fn set_w0(&self,f:T)->Self {Vector::<T>::from_xyzw(self.x(),self.y(),self.z(),zero())}
	fn to_point(&self)->Self {self.set_w(one())}	// synonymous with 'w=1'
	fn to_axis(&self)->Self {self.set_w(zero())}


	fn swap_yz(&self)->Self { Vector::<T>::from_xyzw(self.x(),self.z(),self.y(),self.w())}
	fn swap_xyz(&self)->Self { Vector::<T>::from_xyzw(self.z(),self.y(),self.x(),self.w())}
	fn swap_xyzw(&self)->Self { Vector::<T>::from_xyzw(self.w(),self.z(),self.y(),self.x())}

	fn xy(&self)->V2<T>	{ Vector::from_xy(self.x(),self.y())}
	fn yx(&self)->V2<T>	{ Vector::from_xy(self.y(),self.x())}
	fn xz(&self)->V2<T>	{ Vector::from_xy(self.x(),self.z())}
	fn yz(&self)->V2<T>	{ Vector::from_xy(self.y(),self.z())}

	fn xy01(&self)->V4<T>	{ Vector::from_xyzw(self.x(),self.y(),zero(),one())}

	fn xyz(&self)->V3<T>	 { Vector::from_xyz(self.x(),self.y(),self.z())}
	fn xyz1(&self)->V4<T>	{ Vector::from_xyzw(self.x(),self.y(),self.z(),one())}	// vec3 to homogeneous point
	fn xyz0(&self)->V4<T>	{ Vector::from_xyzw(self.x(),self.y(),self.z(),zero())}	// vec3 to homogeneous offset

	// permutes useful for swapping rgb/bgr
	fn zyx(&self)->V3<T>	{ Vector::from_xyz(self.z(),self.y(),self.x())}
	fn zyx0(&self)->V4<T>	{ Vector::from_xyzw(self.z(),self.y(),self.x(),zero())}
	fn zyx1(&self)->V4<T>	{ Vector::from_xyzw(self.z(),self.y(),self.x(),one())}
	fn zyxw(&self)->V4<T>	{ Vector::from_xyzw(self.z(),self.y(),self.x(),self.w())}

	// permutes for swapping Y or Z being up
	fn xzy(&self)->V3<T>	{ Vector::from_xyz(self.x(),self.z(),self.y())}
	fn xzy0(&self)->V4<T>	{ Vector::from_xyzw(self.x(),self.z(),self.y(),zero())}
	fn xzy1(&self)->V4<T>	{ Vector::from_xyzw(self.x(),self.z(),self.y(),one())}
	fn xzyw(&self)->V4<T>	{ Vector::from_xyzw(self.x(),self.z(),self.y(),self.w())}

	// swap all, swap pairs
	fn wzyx(&self)->V4<T>	{ Vector::from_xyzw(self.w(),self.z(),self.y(),self.x())}
	fn yxwz(&self)->V4<T>	{ Vector::from_xyzw(self.y(),self.x(),self.w(),self.z())}

	// permutes for cross-product
	// i  j   k
	// x0 y0 z0
	// x1 y1 z1

	// x'=y0*z1 - z0*y1 ,  y'=z0*x1-x0*z1,  z'=x0*y1-y0*x1
	fn yzx(&self)->V3<T> {Vector::from_xyz(self.y(),self.z(),self.x())}
	fn zxy(&self)->V3<T> {Vector::from_xyz(self.z(),self.x(),self.y())}
	fn yzxw(&self)->V4<T> {Vector::from_xyzw(self.y(),self.z(),self.x(),self.w())}
	fn zxyw(&self)->V4<T> {Vector::from_xyzw(self.z(),self.x(),self.y(),self.w())}
	fn yzx0(&self)->V4<T> {Vector::from_xyzw(self.y(),self.z(),self.x(),zero())}
	fn zxy0(&self)->V4<T> {Vector::from_xyzw(self.z(),self.x(),self.y(),zero())}

	// splats in permute syntax
	fn xxxx(&self)->V4<T>	{ Vector::from_xyzw(self.x(),self.x(),self.x(),self.x())}
	fn yyyy(&self)->V4<T>	{ Vector::from_xyzw(self.y(),self.y(),self.y(),self.y())}
	fn zzzz(&self)->V4<T>	{ Vector::from_xyzw(self.z(),self.z(),self.z(),self.z())}
	fn wwww(&self)->V4<T>	{ Vector::from_xyzw(self.w(),self.w(),self.w(),self.w())}

}




//pub trait VecNum<T:Num> {
//	fn from_xyz(x:T,y:T,z:T)->Self;
//}
pub trait VecCmp<T:PartialOrd> {
	fn min(&self,b:&Self)->Self;
	fn max(&self,b:&Self)->Self;
	fn max_elem_index(&self)->uint;
}

// componentwise multiplication operator for vectors
impl<F:Float> Mul<Vec2<F>,Vec2<F>> for Vec2<F> {
	fn mul(&self,b:&Vec2<F>)->Vec2<F> {
		Vec2(self.0*b.0,self.1*b.1)
	}
}
impl<F:Float> Div<Vec2<F>,Vec2<F>> for Vec2<F> {
	fn div(&self,b:&Vec2<F>)->Vec2<F> {
		Vec2(self.0/b.0,self.1/b.1)
	}
}
impl<F:Float> Div<Vec3<F>,Vec3<F>> for Vec3<F> {
	fn div(&self,b:&Vec3<F>)->Vec3<F> {
		Vec3(self.0/b.0,self.1/b.1,self.2/b.2)
	}
}
impl<F:Float> Div<Vec4<F>,Vec4<F>> for Vec4<F> {
	fn div(&self,b:&Vec4<F>)->Vec4<F> {
		Vec4(self.0/b.0,self.1/b.1,self.2/b.2,self.3/b.3)
	}
}
impl<T:Rem<T,T>> Rem<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn rem(&self,rhs:&Vec2<T>)->Vec2<T> {
		Vec2(self.0%rhs.0,self.1%rhs.1)
	}
}
impl<T:Rem<T,T>> Rem<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn rem(&self,rhs:&Vec3<T>)->Vec3<T> {
		Vec3(self.0%rhs.0,self.1%rhs.1,self.2%rhs.2)
	}
}
impl<T:Rem<T,T>> Rem<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn rem(&self,rhs:&Vec4<T>)->Vec4<T> {
		Vec4(self.0%rhs.0,self.1%rhs.1,self.2%rhs.2,self.3%rhs.3)
	}
}
impl<T:Neg<T>> Neg<Vec2<T>> for Vec2<T> {
	fn neg(&self)->Vec2<T> { Vec2(-self.0,-self.1) }
}
impl<T:Neg<T>> Neg<Vec3<T>> for Vec3<T> {
	fn neg(&self)->Vec3<T> { Vec3(-self.0,-self.1,-self.2) }
}
impl<T:Neg<T>> Neg<Vec4<T>> for Vec4<T> {
	fn neg(&self)->Vec4<T> { Vec4(-self.0,-self.1,-self.2,-self.3) }
}
pub trait Sum<T> {
	fn sum(&self)->T;
}


impl<T:Mul<T,T>> Mul<Vec3<T>,Vec3<T>> for Vec3<T>{
	fn mul(&self, rhs:&Vec3<T>)->Vec3<T> { Vec3(rhs.0*self.0,rhs.1*self.1,rhs.2*self.2) }
}
impl<T:Mul<T,T>> Mul<Vec4<T>,Vec4<T>> for Vec4<T>{
	fn mul(&self, rhs:&Vec4<T>)->Vec4<T> { Vec4(rhs.0*self.0,rhs.1*self.1,rhs.2*self.2,self.3*rhs.3) }
}
impl<T:Mul<T,T>> Mul<T,Vec4<T>> for Vec4<T>{
	fn mul(&self, rhs:&T)->Vec4<T> { Vec4(self.0**rhs,self.1**rhs, self.2**rhs, self.3**rhs) }
}
impl<T:Mul<T,T>> Mul<T,Vec3<T>> for Vec3<T>{
	fn mul(&self, rhs:&T)->Vec3<T> { Vec3(self.0**rhs,self.1**rhs, self.2**rhs) }
}


impl<T:PartialEq> PartialEq for Vec2<T> {
	fn eq(&self,rhs:&Vec2<T>)->bool { return self.0==rhs.0 && self.1==rhs.1 }
}
impl<T:PartialEq> PartialEq for Vec3<T> {
	fn eq(&self,rhs:&Vec3<T>)->bool { return self.0==rhs.0 && self.1==rhs.1 && self.2==rhs.2 }
}
impl<T:PartialEq> PartialEq for Vec4<T> {
	fn eq(&self,rhs:&Vec4<T>)->bool { return self.0==rhs.0 && self.1==rhs.1 && self.2==rhs.2 && self.3==rhs.3 }
}


// todo: satisfy if Num+Clone only
impl<T:Num+Copy+Float> Num for Vec2<T> {}
impl<T:Num+Copy+Float> Num for Vec3<T> {}
impl<T:Num+Copy+Float> Num for Vec4<T> {}

impl<T:Float+Copy> Vec3<T> {
	pub fn origin()->Vec3<T>	{ Vec3(zero(),zero(),zero()) }
	pub fn axis(i:int)->Vec3<T>{
		match i{ 0=>Vec3(one(),zero(),zero()),
                1=>Vec3(zero(),one(),zero()),
                2=>Vec3(zero(),zero(),one()),
                _=>Vec3(zero(),zero(),zero())}
	}
	
	pub fn xyz(&self)->Vec3<T> {Vec3(self.0,self.1,self.2)}
	pub fn xyz1(&self)->Vec4<T> {Vec4(self.0,self.1,self.2,one())}
	pub fn xyz0(&self)->Vec4<T> {Vec4(self.0,self.1,self.2,zero())}
	pub fn xyzw(&self)->Vec4<T> {Vec4(self.0, self.1, self.2, zero())}
	pub fn longest_axis(&self)->uint{ self.mul(self).max_elem_index()}
	// abstractions may help keeping in SIMD regs with direct impl, generic provided here
	// optimized versions might multiply by splatted x,y,z,w instead of scalar.
	pub fn mul_x(&self, b:&Vec3<T>)->Vec3<T> { self.scale(b.0) }
	pub fn mul_y(&self, b:&Vec3<T>)->Vec3<T> { self.scale(b.1) }
	pub fn mul_z(&self, b:&Vec3<T>)->Vec3<T> { self.scale(b.2) }
	fn mul_w(&self, b:&Vec3<T>)->Vec3<T> { Vec3(zero(),zero(),zero()) }
	fn add_mul_x(&self, b:&Vec3<T>,c:&Vec3<T>)->Vec3<T> { self.macc(b,c.0) }
	fn add_mul_y(&self, b:&Vec3<T>,c:&Vec3<T>)->Vec3<T> { self.macc(b,c.1) }
	fn add_mul_z(&self, b:&Vec3<T>,c:&Vec3<T>)->Vec3<T> { self.macc(b,c.2) }
	fn add_mul_w(&self, b:&Vec3<T>,c:&Vec3<T>)->Vec3<T> { *self }
	// transform self by axes
	pub fn mul_xyz_sum(&self, ax:&Vec3<T>,ay:&Vec3<T>,az:&Vec3<T>)->Vec3<T> {
		ax.mul_x(self).add_mul_y(ay,self).add_mul_z(az,self)
	}

	// todo: 'cross' could use permutes. However, we dont want this trait to depend on 'VecPermute'
	// because that needs the v2,v3,v4 versions
	// do we have a special set of permutes for exp

	pub fn cross(&self,b:&Vec3<T>)->Vec3<T>	{Vec3(self.1*b.2-self.2*b.1,self.2*b.0-self.0*b.2,self.0*b.1-self.1*b.0)}

	pub fn scale(&self,f:T)->Vec3<T>	{ Vec3(self.0*f,self.1*f,self.2*f) }
	pub fn dot(&self,b:&Vec3<T>)->T	{self.mul(b).sum()}
	pub fn para(&self,vaxis:&Vec3<T>)->Vec3<T> {  	let dotp=self.dot(vaxis); vaxis.scale(dotp) }

	pub fn neg(&self)->Vec3<T> {self.scale(-one::<T>())}
	pub fn avr(&self,b:&Vec3<T>)->Vec3<T> {self.add(b).scale(one::<T>()/(one::<T>()+one::<T>()))}
	pub fn macc(&self,b:&Vec3<T>,f:T)->Vec3<T>	{self.add(&b.scale(f))} //'Multiply-Accumulate' we prefer base+ofs*scale to a*b+c
	pub fn add_scale(&self,b:&Vec3<T>,f:T)->Vec3<T>{self.macc(b,f)} // synonymous. 
	pub fn add_mul(&self,b:&Vec3<T>,c:&Vec3<T>)->Vec3<T>{self.add(&b.mul(c))} // 
	pub fn lerp(&self,b:&Vec3<T>,f:T)->Vec3<T>	{self.macc(&b.sub(self),f)}
	pub fn sqr(&self)->T { self.dot(self)} //todo:ambiguous, maybe a*a which is componentwise.
	pub fn length(&self)->T { self.sqr().sqrt()}
	pub fn length_squared(&self)->T { self.dot(self)}
	pub fn inv_length(&self)->T { one::<T>()/self.sqr().sqrt()}
	pub fn scale_to_length(&self,length:T)->Vec3<T> { self.scale(length/self.sqr().sqrt()) }
	pub fn normalize(&self)->Vec3<T> { self.scale(one::<T>()/self.sqr().sqrt()) }
	pub fn perp(&self,axis:&Vec3<T>)->Vec3<T> { let vpara =self.para(axis); self.sub(&vpara)}
	pub fn cross_norm(&self, b:&Vec3<T>)->Vec3<T> { self.cross(b).normalize() }
	pub fn sub_norm(&self,b:&Vec3<T>)->Vec3<T> { self.sub(b).normalize() }
	//pub fn axisScale(i:int,f:VScalar)->Self;
	// { VecOps::axis(i).vscale(f)} how?
	pub fn reflect(&self,a:&Vec3<T>)->Vec3<T> { self.macc(a, self.dot(a)*(one::<T>()+one::<T>())) }

	pub fn para_perp(&self,vaxis:&Vec3<T>)->(Vec3<T>,Vec3<T>) {
		let vpara=self.para(vaxis);
		(vpara,self.sub(&vpara))
	}
	pub fn dist(&self,b:&Vec3<T>)->T {self.sub(b).length()}
	pub fn dist_squared(&self,b:&Vec3<T>)->T {self.sub(b).sqr()}
}


// vector maths gathers primitive operations and implements more in terms of them
// This is not 
impl<T:Float+Copy> Vec4<T> {
	pub fn origin()->Vec4<T>	{ Vec4(zero(),zero(),zero(),one()) }
	pub fn axis(i:int)->Vec4<T>{
		match i{ 0=>Vec4(one(),zero(),zero(),zero()),
                1=>Vec4(zero(),one(),zero(),zero()),
                2=>Vec4(zero(),zero(),one(),zero()),
                3=>Vec4(zero(),zero(),zero(),one()),
                _=>Vec4(zero(),zero(),zero(),zero())}
	}
	pub fn longest_axis(&self)->uint{ self.mul(self).max_elem_index()}

	// permute stuff
	pub fn xyz(&self)->Vec3<T> {Vec3(self.0,self.1,self.2)}

	// abstractions may help keeping in SIMD regs with direct impl, generic provided here
	// optimized versions might multiply by splatted x,y,z,w instead of scalar.
	pub fn mul_x(&self, b:&Vec4<T>)->Vec4<T> { self.scale(b.0) }
	pub fn mul_y(&self, b:&Vec4<T>)->Vec4<T> { self.scale(b.1) }
	pub fn mul_z(&self, b:&Vec4<T>)->Vec4<T> { self.scale(b.2) }
	pub fn mul_w(&self, b:&Vec4<T>)->Vec4<T> { self.scale(b.w()) }
	pub fn add_mul_x(&self, b:&Vec4<T>,c:&Vec4<T>)->Vec4<T> { self.macc(b,c.0) }
	pub fn add_mul_y(&self, b:&Vec4<T>,c:&Vec4<T>)->Vec4<T> { self.macc(b,c.1) }
	pub fn add_mul_z(&self, b:&Vec4<T>,c:&Vec4<T>)->Vec4<T> { self.macc(b,c.2) }
	pub fn add_mul_w(&self, b:&Vec4<T>,c:&Vec4<T>)->Vec4<T> { self.macc(b,c.3) }
	// transform self by axes
	pub fn mul_xyzw_sum(&self, ax:&Vec4<T>,ay:&Vec4<T>,az:&Vec4<T>,aw:&Vec4<T>)->Vec4<T> {
		ax.mul_x(self).add_mul_y(ay,self).add_mul_z(az,self).add_mul_w(aw,self)
	}

	// todo: 'cross' could use permutes. However, we dont want this trait to depend on 'VecPermute'
	// because that needs the v2,v3,v4 versions
	// do we have a special set of permutes for exp

	pub fn cross(&self,b:&Vec4<T>)->Vec4<T>	{Vec4(self.1*b.2-self.2*b.1,self.2*b.0-self.0*b.2,self.0*b.1-self.1*b.0, zero())}

	pub fn scale(&self,f:T)->Vec4<T>	{ Vec4(self.0*f,self.1*f,self.2*f,self.3*f) }
	pub fn dot(&self,b:&Vec4<T>)->T	{self.mul(b).sum()}
	pub fn para(&self,vaxis:&Vec4<T>)->Vec4<T> {  	let dotp=self.dot(vaxis); vaxis.scale(dotp) }

	pub fn neg(&self)->Vec4<T> {self.scale(-one::<T>())}
	pub fn avr(&self,b:&Vec4<T>)->Vec4<T> {self.add(b).scale(one::<T>()/(one::<T>()+one::<T>()))}
	pub fn macc(&self,b:&Vec4<T>,f:T)->Vec4<T>	{self.add(&b.scale(f))} //'Multiply-Accumulate' we prefer base+ofs*scale to a*b+c
	pub fn add_scale(&self,b:&Vec4<T>,f:T)->Vec4<T>{self.macc(b,f)} // synonymous. 
	pub fn add_mul(&self,b:&Vec4<T>,c:&Vec4<T>)->Vec4<T>{self.add(&b.mul(c))} // 
	pub fn lerp(&self,b:&Vec4<T>,f:T)->Vec4<T>	{self.macc(&b.sub(self),f)}
	pub fn sqr(&self)->T { self.dot(self)} //todo:ambiguous, maybe a*a which is componentwise.
	pub fn length(&self)->T { self.sqr().sqrt()}
	pub fn length_squared(&self)->T { self.dot(self)}
	pub fn inv_length(&self)->T { one::<T>()/self.sqr().sqrt()}
	pub fn scale_to_length(&self,length:T)->Vec4<T> { self.scale(length/self.sqr().sqrt()) }
	pub fn normalize(&self)->Vec4<T> { self.scale(one::<T>()/self.sqr().sqrt()) }
	pub fn perp(&self,axis:&Vec4<T>)->Vec4<T> { let vpara =self.para(axis); self.sub(&vpara)}
	pub fn cross_norm(&self, b:&Vec4<T>)->Vec4<T> { self.cross(b).normalize() }
	pub fn sub_norm(&self,b:&Vec4<T>)->Vec4<T> { self.sub(b).normalize() }
	//pub fn axisScale(i:int,f:VScalar)->Self;
	// { VecOps::axis(i).vscale(f)} how?
	pub fn reflect(&self,a:&Vec4<T>)->Vec4<T> { self.macc(a, self.dot(a)*(one::<T>()+one::<T>())) }

	pub fn para_perp(&self,vaxis:&Vec4<T>)->(Vec4<T>,Vec4<T>) {
		let vpara=self.para(vaxis);
		(vpara,self.sub(&vpara))
	}
	pub fn dist(&self,b:&Vec4<T>)->T {self.sub(b).length()}
	pub fn dist_squared(&self,b:&Vec4<T>)->T {self.sub(b).sqr()}
}


//todo: HALF

//  wtf this does,t work now
impl<T:Add<T,T>> Add<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn add(&self,rhs:&Vec2<T>)->Vec2<T> { 
		Vec2(self.0+rhs.0, self.1+rhs.1)
	}
}
impl<T:Add<T,T>> Add<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn add(&self,rhs:&Vec3<T>)->Vec3<T> { 
		Vec3(self.0+rhs.0   , self.1+rhs.1, self.2+rhs.2)
	}
}
impl<T:Add<T,T>> Add<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn add(&self,rhs:&Vec4<T>)->Vec4<T> { 
		Vec4(self.0+rhs.0   , self.1+rhs.1, self.2+rhs.2, self.3+rhs.3)
	}
}

//  wtf this does,t work now
impl<T:Sub<T,T>> Sub<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn sub(&self,rhs:&Vec2<T>)->Vec2<T> { 
		Vec2(self.0-rhs.0, self.1-rhs.1)
	}
}
impl<T:Sub<T,T>> Sub<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn sub(&self,rhs:&Vec3<T>)->Vec3<T> { 
		Vec3(self.0-rhs.0   , self.1-rhs.1, self.2-rhs.2)
	}
}
impl<T:Sub<T,T>> Sub<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn sub(&self,rhs:&Vec4<T>)->Vec4<T> { 
		Vec4(self.0-rhs.0   , self.1-rhs.1, self.2-rhs.2, self.3-rhs.3)
	}
}


impl<T:Zero+Copy+One> Zero for Vec2<T> {
	fn zero()->Vec2<T> {Vec2(zero::<T>(),zero::<T>())}
	fn is_zero(&self)->bool { let Vec2(ref x,ref y)=*self; x.is_zero() && y.is_zero()}
}
//fn vec_axis_scale<T:Float,V:VecMath<T>>(i:int,f:T)->V { let ret:V; ret=VecMath::axis(i); ret.scale(f) }

impl<T:PartialOrd+Copy+Zero+One> VecCmp<T> for Vec2<T> {
	fn min(&self,b:&Vec2<T>)->Vec2<T>	{Vec2(fmin(self.0,b.0),fmin(self.1,b.1))}
	fn max(&self,b:&Vec2<T>)->Vec2<T>	{Vec2(fmax(self.0,b.0),fmax(self.1,b.1))}
	fn max_elem_index(&self)->uint { if self.x()>self.y() {0}else{1}}
	
}

impl<T:Add<T,T>> Sum<T> for Vec2<T> {
	fn sum(&self)->T	{self.0+self.1}
}


impl<T> Vec2<T> {
	pub fn ref0<'a>(&'a self)->&'a T { &self.0}
	pub fn ref1<'a>(&'a self)->&'a T { &self.1}
}

impl<T:Copy+Zero+One> Vector<T> for Vec2<T> {
//	type V2=Vec2<T>;
//	type V3=Vec3<T>;
//	type V4=Vec4<T>;

	fn x(&self)->T	{ self.0}
	fn y(&self)->T	{ self.1}
	fn z(&self)->T	{ zero::<T>()}
	fn w(&self)->T	{ zero::<T>()}
	fn from_xyzw(x:T,y:T,_:T,_:T)->Vec2<T> { Vec2(x,y) }
}

impl<T:Copy+Zero+One> Zero for Vec3<T> {
	fn zero()->Vec3<T>{Vec3(zero(),zero(),zero())}
	fn is_zero(&self)->bool  { self.0.is_zero() && self.1.is_zero() && self.2.is_zero()}
}

impl<T:Copy+One+Zero+Float> One for Vec2<T> {
	fn one()->Vec2<T>{Vec2(one(),one())}
}
impl<T:Copy+One+Zero+Float> One for Vec3<T> {
	fn one()->Vec3<T>{Vec3(one(),one(),one())}
}
impl<T:Copy+One+Zero+Float> One for Vec4<T> {
	fn one()->Vec4<T>{Vec4(one(),one(),one(),one())}
}


impl<T:Copy+PartialOrd+Zero+One> VecCmp<T> for Vec3<T> {
	fn min(&self,b:&Vec3<T>)->Vec3<T>	{
		let x=fmin(self.0, b.0);
		let y=fmin(self.1, b.1);
		let z=fmin(self.2, b.2);
		Vec3(x,y,z)}
	fn max(&self,b:&Vec3<T>)->Vec3<T>	{Vec3(
									fmax(self.0,b.0),
									fmax(self.1,b.1),
									fmax(self.2,b.2))}
	fn max_elem_index(&self)->uint { if self.0>self.1 {if self.0>self.2{0}else{2}}
									else{if self.1>self.2{1}else{2}}}
}

impl<T:Add<T,T>+Copy+Zero+One> Sum<T> for Vec3<T> {
	fn sum(&self)->T	{self.0+self.1+self.2}
}
impl<T:Add<T,T>> Sum<T> for [T,..2] {
	fn sum(&self)->T	{self[0]+self[1]}
}
impl<T:Add<T,T>> Sum<T> for [T,..3] {
	fn sum(&self)->T	{self[0]+self[1]+self[2]}
}
impl<T:Add<T,T>> Sum<T> for [T,..4] {
	fn sum(&self)->T	{self[0]+self[1]+self[2]+self[3]}
}

impl<T>  Vec3<T> {
	pub fn ref0<'a>(&'a self)->&'a T { &self.0}
	pub fn ref1<'a>(&'a self)->&'a T { &self.1}
	pub fn ref2<'a>(&'a self)->&'a T { &self.2}
}
impl<T:Copy+Zero+One> Vector<T> for Vec3<T> {
//	type V2 = Vec2<T>;
//	type V3 = Vec3<T>;
//	type V4 = Vec4<T>;
	fn x(&self)->T	{ self.0}
	fn y(&self)->T	{ self.1}
	fn z(&self)->T	{ self.2}
	fn w(&self)->T	{ zero()}
	fn from_xyzw(x:T,y:T,z:T,_:T)->Vec3<T> { Vec3(x,y,z) }
}

impl<T:Copy+Zero+One> Zero for Vec4<T> {
	fn zero()->Vec4<T>{ Vector::splat(zero::<T>())}
	fn is_zero(&self)->bool  {self.0.is_zero() && self.1.is_zero() && self.2.is_zero() && self.3.is_zero()}
}

// Converting Vec2,Vec3,Vec4 to/from tuples & arrays

impl<T:Copy+Zero+One> Vector<T> for [T,..2] {
//	type V2 = [T,..2];
//	type V3 = [T,..3];
//	type V4 = [T,..4];
	fn x(&self)->T { self[0] }
	fn y(&self)->T { self[1] }
	fn z(&self)->T { zero() }
	fn w(&self)->T { zero() }
	fn from_xyzw(x:T,y:T,z:T,w:T)->[T,..2] { [x,y] }
}
impl<T:Copy+Zero+One> Vector<T> for [T,..3] {
//	type V2 = [T,..2];
//	type V3 = [T,..3];
//	type V4 = [T,..4];
	fn x(&self)->T { self[0] }
	fn y(&self)->T { self[1] }
	fn z(&self)->T { self[2] }
	fn w(&self)->T { zero() }
	fn from_xyzw(x:T,y:T,z:T,w:T)->[T,..3]{ [x,y,z] }
}
impl<T:Copy+Zero+One> Vector<T> for [T,..4] {
//	type V2 = [T,..2];
//	type V3 = [T,..3];
//	type V4 = [T,..4];
	fn x(&self)->T { self[0] }
	fn y(&self)->T { self[1] }
	fn z(&self)->T { self[2] }
	fn w(&self)->T { self[3] }
	fn from_xyzw(x:T,y:T,z:T,w:T)->[T,..4]{ [x,y,z,w] }
}

impl<T:Copy+Zero+One> Vector<T> for (T,T) {
//	type V2 = (T,T);
//	type V3 = (T,T,T);
//	type V4 = (T,T,T,T);
	fn x(&self)->T { self.0 }
	fn y(&self)->T { self.1 }
	fn z(&self)->T { zero() }
	fn w(&self)->T { zero() }
	fn from_xyzw(x:T,y:T,z:T,w:T)->(T,T) { (x,y) }
}
impl<T:Copy+Zero+One> Vector<T> for (T,T,T) {
//	type V2 = (T,T);
//	type V3 = (T,T,T);
//	type V4 = (T,T,T,T);
	fn x(&self)->T { self.0 }
	fn y(&self)->T { self.1 }
	fn z(&self)->T { self.2 }
	fn w(&self)->T { zero() }
	fn from_xyzw(x:T,y:T,z:T,w:T)->(T,T,T) { (x,y,z) }

}
impl<T:Copy+Zero+One> Vector<T> for (T,T,T,T) {
//	type V2 = (T,T);
//	type V3 = (T,T,T);
//	type V4 = (T,T,T,T);
	fn x(&self)->T {self.0}
	fn y(&self)->T {self.1}
	fn z(&self)->T {self.2}
	fn w(&self)->T {self.3}
	fn from_xyzw(x:T,y:T,z:T,w:T)->(T,T,T,T) { (x,y,z,w) }
}

impl<T:Copy+Zero+One> Vec4<T> {
	pub fn to_array(&self)->[T,..4] { [self.0,self.1,self.2,self.3] }
	pub fn to_tuple(&self)->(T,T,T,T) { (self.0,self.1,self.2,self.3) }

	pub fn from_array([x,y,z,w]:[T,..4])->Vec4<T> {Vec4(x,y,z,w)}
	pub fn from_tuple((x,y,z,w):(T,T,T,T))->Vec4<T> {
		Vec4(x,y,z,w)
	}
}
impl<T:Copy+Zero+One> Vec3<T> {
	pub fn to_array(&self)->[T,..3] { [self.0,self.1,self.2] }
	fn to_tuple(&self)->(T,T,T) { (self.0,self.1,self.2) }

	pub fn from_array([x,y,z]:[T,..3])->Vec3<T> {Vec3(x,y,z)}
	fn from_tuple((x,y,z):(T,T,T))->Vec3<T> { Vec3(x,y,z) }
}
impl<T:Copy+Zero+One> Vec2<T> {
	pub fn to_array(&self)->[T,..2] { [self.0,self.1] }
	fn to_tuple(&self)->(T,T) { (self.0,self.1) }
	pub fn from_array([x,y]:[T,..2])->Vec2<T> {Vec2(x,y)}
	fn from_tuple((x,y):(T,T))->Vec2<T> { Vec2(x,y) }
}


impl<T:Copy+PartialOrd+Zero+One> VecCmp<T> for Vec4<T> {
	fn min(&self,b:&Vec4<T>)->Vec4<T>	{Vec4(
									fmin(self.0,b.0),
									fmin(self.1,b.1),
									fmin(self.2,b.2),
									fmin(self.3,b.3))}
	fn max(&self,b:&Vec4<T>)->Vec4<T>	{Vec4(
									fmax(self.0,b.0),
									fmax(self.1,b.1),
									fmax(self.2,b.2),
									fmax(self.3,b.3))}
	fn max_elem_index(&self)->uint { 
		let (f0,max_xy)=if self.0>self.1 {(self.0,0)}else{(self.1,1)};
		let (f1,max_zw)=if self.2>self.3 {(self.2,2)}else{(self.3,3)};
		if f0>f1 {max_xy} else{max_zw}
	}
}

//impl<T:Clone+Num> VecNum<T> for Vec4<T> {
//	fn from_xyz(x:T,y:T,z:T)->Vec4<T>{Vec4(x,y,z,zero::<T>())}
//}



impl<T:Num+Copy> Sum<T> for Vec4<T> {
	fn sum(&self)->T	{self.0+self.1+self.2+self.3}
}

impl<T> Vec4<T> {
	pub fn ref0<'a>(&'a self)->&'a T { &self.0}
	pub fn ref1<'a>(&'a self)->&'a T { &self.1}
	pub fn ref2<'a>(&'a self)->&'a T { &self.2}
	pub fn ref3<'a>(&'a self)->&'a T { &self.3}
}

impl<T:ToPrimitive+Copy+Zero+One> Vec4<T> {
	pub fn to_f32(&self)->Vec4<f32> { self.map(|&x|x.to_f32().unwrap()) }
	pub fn to_f64(&self)->Vec4<f64> { self.map(|&x|x.to_f64().unwrap()) }
	pub fn to_int(&self)->Vec4<int> { self.map(|&x|x.to_int().unwrap()) }
	pub fn to_i32(&self)->Vec4<i32> { self.map(|&x|x.to_i32().unwrap()) }
	pub fn to_u8(&self)->Vec4<u8> { self.map(|&x|x.to_u8().unwrap()) }
}

impl<T:Copy+Zero+One> Vector<T> for Vec4<T>
{
//	type V2=Vec2<T>;
//	type V3=Vec3<T>;
//	type V4=Vec4<T>;
	fn x(&self)->T {self.0}
	fn y(&self)->T {self.1}
	fn z(&self)->T {self.2}
	fn w(&self)->T {self.3}
	fn from_xyzw(x:T,y:T,z:T,w:T)->Vec4<T> { Vec4(x,y,z,w) }

}

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

pub trait ToVec2<T> {
	fn to_vec2(&self)->Vec2<T>;
}
pub trait ToVec3<T> {
	fn to_vec3(&self)->Vec3<T>;
}
pub trait ToVec4<T> {
	fn to_vec4(&self)->Vec4<T>;
	fn to_vec4_pos(&self)->Vec4<T>;
}

impl<T:Copy+Zero+One,V:Vec3To<T>> To<V> for Vec3<T>{
	fn to(&self)->V { Vec3To::vec3_to(self)}
}
impl<T:Copy+Zero+One,V:Vec4To<T>> To<V> for Vec4<T>{
	fn to(&self)->V { Vec4To::vec4_to(self)}
}
trait Vec3To<T> {
	fn vec3_to(s:&Vec3<T>)->Self;
}
trait Vec4To<T> {
	fn vec4_to(s:&Vec4<T>)->Self;
}
impl<T:Copy+Zero+One> Vec3To<T> for Vec3<T>{
	fn vec3_to(s:&Vec3<T>)->Vec3<T> { *s}
}
impl<T:Copy+Zero+One> Vec4To<T> for Vec4<T>{
	fn vec4_to(s:&Vec4<T>)->Vec4<T> { *s}
}
// Componentwise conversion for vector
/*
impl<A:To<B>+Clone+Zero+One, B:Clone+Zero+One> To<Vec3<B>> for Vec3<A> {
	fn to(&self)->Vec3<B> { Vec3( self.x().to(),	self.y().to(),  self.z().to() )}
}
impl<A:To<B>+Clone+Zero+One, B:Clone+Zero+One> To<Vec4<B>> for Vec4<A> {
	fn to(&self)->Vec4<B> { Vec4( self.x().to(),	self.y().to(),  self.z().to(), self.w().to() )}
}
*/


impl<T:Copy+Zero+One> Vec3To<T> for Vec4<T>{
	fn vec3_to(s:&Vec3<T>)->Vec4<T> { Vec4(s.0,s.1,s.2,zero())}
}
impl<T:Copy+Zero+One> Vec4To<T> for Vec3<T>{
	fn vec4_to(s:&Vec4<T>)->Vec3<T> { Vec3(s.0,s.1,s.2)}
}


impl<T:Copy+Zero> ToVec2<T> for (T,T){
	fn to_vec2(&self)->Vec2<T>{Vec2(self.0,self.1)}
}
impl<T:Copy+Zero> ToVec3<T> for (T,T,T){
	fn to_vec3(&self)->Vec3<T>{Vec3(self.0,self.1,self.2)}
}
impl<T:Copy+Zero+One> ToVec4<T> for (T,T,T,T){
	fn to_vec4(&self)->Vec4<T>{Vec4(self.0,self.1,self.2,self.3)}
	fn to_vec4_pos(&self)->Vec4<T>{Vec4(self.0,self.1,self.2,one())}
}

impl<T:Copy+Zero> ToVec2<T> for [T,..2]{
	fn to_vec2(&self)->Vec2<T>{Vec2(self[0],self[1])}
}
impl<T:Copy+Zero> ToVec3<T> for [T,..3]{
	fn to_vec3(&self)->Vec3<T>{Vec3(self[0],self[1],self[2])}
 }
impl<T:Copy+Zero+One> ToVec4<T> for [T,..4]{
	fn to_vec4(&self)->Vec4<T>{Vec4(self[0],self[1],self[2],self[3])}
	fn to_vec4_pos(&self)->Vec4<T>{Vec4(self[0],self[1],self[2],one())}
}
impl<T:Copy+Zero+One> ToVec3<T> for Vec4<T> {
	fn to_vec3(&self)->Vec3<T>{ Vec3(self.0,self.1,self.2) }
}
impl<T:Copy+Zero+One> ToVec3<T> for Vec3<T> {
	fn to_vec3(&self)->Vec3<T>{ Vec3(self.0,self.1,self.2) }
}
impl<T:Copy+Zero+One> ToVec3<T> for Vec2<T> {
	fn to_vec3(&self)->Vec3<T>{ Vec3(self.0,self.1,zero()) }
}
impl<T:Copy+Zero+One> ToVec4<T> for Vec4<T> {
	fn to_vec4(&self)->Vec4<T>{ Vec4(self.0,self.1,self.2,self.3) }
	fn to_vec4_pos(&self)->Vec4<T>{ Vec4(self.0,self.1,self.2,one()) }
}
impl<T:Copy+Zero+One> ToVec4<T> for Vec3<T> {
	fn to_vec4(&self)->Vec4<T>{ Vec4(self.0,self.1,self.2,zero()) }
	fn to_vec4_pos(&self)->Vec4<T>{ Vec4(self.0,self.1,self.2,one()) }
}


impl<T:Copy+Zero+One> ToVec4<T> for (Vec3<T>,T){
	fn to_vec4(&self)->Vec4<T>{let (Vec3(x,y,z), w)=*self;Vec4(x,y,z,w)}
	fn to_vec4_pos(&self)->Vec4<T>{let (Vec3(x,y,z), w)=*self;Vec4(x,y,z,one())}
}
impl<T:Copy+Zero+One> ToVec4<T> for ((T,T,T),T){
	fn to_vec4(&self)->Vec4<T>{let ((x,y,z),w)=*self;Vec4(x,y,z,w)}
	fn to_vec4_pos(&self)->Vec4<T>{let ((x,y,z),w)=*self;Vec4(x,y,z,one()) }
}
impl<T:Copy+Zero+One> ToVec4<T> for ([T,..3],T){
	fn to_vec4(&self)->Vec4<T>{let ([x,y,z],w)=*self;Vec4(x,y,z,w)}
	fn to_vec4_pos(&self)->Vec4<T>{let ([x,y,z],w)=*self;Vec4(x,y,z,one())}
}



// app_render
#[cfg(run)]
fn main() {
	io::println("Vec Math Test");
	dump!(Vec3(1.0f32,2.0f32,3.0f32)*2.0f32);
	dump!(Vec3(1.0f32,2.0f32,3.0f32)*Vec3(3.0f32,2.0f32,1.0f32));
	dump!(1i,2i,3i,4i);
	let x:Vec4<i32>= Vec4(0u32,1u32,2u32,3u32).to(); 
	dump!(x);
}

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




