
use std::cmp;

pub use std::num;
pub use std::num::*;


/// Generic maths classes
/// member functions prefixed with 'v' for easier life without code-completion, and to distinguish from operator overloads (official langauge level "add") etc

#[deriving(Clone,Show)]
pub struct Vec2<T=f32> {x:T, y:T}

#[deriving(Clone,Show)]
pub struct Vec3<T=f32> {x:T, y:T, z:T}

#[deriving(Clone,Show)]
pub struct Vec4<T=f32> {x:T, y:T, z:T, w:T}

pub trait MyOrd : Num+Ord+Clone {	// relaxed handling of floats compared to s
	fn cmp(self, other:Self)->cmp::Ordering;
	fn min(a:Self,b:Self)->Self { match a.clone().cmp(b.clone()) { cmp::Less|cmp::Equal=>a,cmp::Greater=>b } }
	fn max(a:Self,b:Self)->Self { match a.clone().cmp(b.clone()) { cmp::Greater|cmp::Equal=>a,cmp::Less=>b } }
	fn clamp(a:Self, lo:Self,hi:Self)->Self { MyOrd::max(lo,MyOrd::min(a,hi)) }
	fn clamps(a:Self, hi:Self)->Self { clamp(a,hi.clone(),-hi) }
}
impl MyOrd for f32 {
	fn cmp(self, other:f32)->cmp::Ordering {
		if self<other {cmp::Less} else if self>other {cmp::Greater} else {cmp::Equal}
	}
}
impl MyOrd for i32 {
	fn cmp(self, other:i32)->cmp::Ordering {
		if self<other {cmp::Less} else if self>other {cmp::Greater} else {cmp::Equal}
	}
}
impl MyOrd for int {
	fn cmp(self, other:int)->cmp::Ordering {
		if self<other {cmp::Less} else if self>other {cmp::Greater} else {cmp::Equal}
	}
}

/*
if default type params are working..
#[deriving(Clone,Show)]
pub struct Vec2<X=f32,Y=X> {x:X, y:Y}

#[deriving(Clone,Show)]
pub struct Vec3<X=f32,Y=X,Z=Y> {x:X, y:Y, z:Z}

#[deriving(Clone,Show)]
pub struct Vec4<X=f32,Y=X,Z=Y,W=Z> {x:X, y:Y, z:Z, w:W}
*/

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

/*
pub struct Vec3f {x:float,y:float,z:float}
impl Vec3f {
	pub fn new2(x:float,y:float)->Vec3f	{ Vec3f{x:x,y:y,z:0.0} }
}
*/

impl<T:Clone> Vec2<T> {
	pub fn new(x:T,y:T)->Vec2<T>	{Vec2{x:x,y:y}}
	pub fn vsplat(v:T)->Vec2<T> { Vec2{x:v.clone(),y:v.clone()}}
}

impl<T:Clone+Num> Vec2<T> {
	pub fn vcrossToScalar(&self,other:&Vec2<T>)->T {self.x*other.y-self.y*other.x}
}
impl<T:Clone> Vec3<T> {
	pub fn new(vx:T,vy:T,vz:T)->Vec3<T>	{Vec3::<T>{x:vx.clone(),y:vy.clone(),z:vz.clone()}}
	pub fn vsplat(v:T)->Vec3<T> { Vec3{x:v.clone(),y:v.clone(),z:v.clone()}}

	pub fn fromVec2(xy:Vec2<T>,z:T)->Vec3<T> {Vec3::<T>{x:xy.x.clone(),y:xy.y.clone(),z:z.clone()}}
}
impl<T:Clone> Vec4<T> {
	pub fn new(x:T,y:T,z:T,w:T)->Vec4<T>	{Vec4{x:x.clone(),y:y.clone(),z:z.clone(),w:w.clone()}}
	pub fn vsplat(v:T)->Vec4<T> { Vec4{x:v.clone(),y:v.clone(),z:v.clone(),w:v.clone()}}	// todo -move to elsewhere

	pub fn vfromVec3(xyz:Vec3<T>,w:T)->Vec4<T> {Vec4{x:xyz.x.clone(),y:xyz.y.clone(),z:xyz.z.clone(),w:w.clone()}}
	pub fn vfromVec2(xy:Vec2<T>,z:T,w:T)->Vec4<T> {Vec4{x:xy.x.clone(),y:xy.y.clone(),z:z.clone(),w:w.clone()}}
	pub fn vfromVec2Vec2(xy:Vec2<T>,zw:Vec2<T>)->Vec4<T> {Vec4{x:xy.x.clone(),y:xy.y.clone(),z:zw.x.clone(),w:zw.y.clone()}}
}

pub trait VecAccessors<T:Clone> {
	fn vx(&self)->T;
	fn vy(&self)->T;
	fn vz(&self)->T;
	fn vw(&self)->T;
}

pub trait VecConsts<T:Clone+One+Zero>
{
	fn origin()->Self;
	fn vaxis(i:int)->Self;
	fn one()->Self;
}

//	fn rsub(&self,&b:Self)->Self { b.sub(self)}
pub trait VecPermute<T:Clone+One+Zero> : VecAccessors<T> {
	fn vxy(&self)->Vec2<T>	{ Vec2::new(self.vx(),self.vy())}
	fn vyx(&self)->Vec2<T>	{ Vec2::new(self.vy(),self.vx())}
	fn vxz(&self)->Vec2<T>	{ Vec2::new(self.vx(),self.vz())}
	fn vyz(&self)->Vec2<T>	{ Vec2::new(self.vy(),self.vz())}
	fn vxyz(&self)->Vec3<T>	{ Vec3::new(self.vx(),self.vy(),self.vz())}
	fn vxyz1(&self)->Vec4<T>	{ Vec4::new(self.vx(),self.vy(),self.vz(),one())}	// vec3 to homogeneous point
	fn vxyz0(&self)->Vec4<T>	{ Vec4::new(self.vx(),self.vy(),self.vz(),zero())}	// vec3 to homogeneous offset
	fn vzyx(&self)->Vec3<T>	{ Vec3::new(self.vz(),self.vy(),self.vx())}	// when using as color components
	fn vxzy(&self)->Vec3<T>	{ Vec3::new(self.vx(),self.vz(),self.vy())}	// changing which is up, y or z

}
// Where to put these..
/*
	fn ymx(&self)->Vec2<T>	{ Vec2::new(self.y(),-self.x())}
	fn myx(&self)->Vec2<T>	{ Vec2::new(-self.y(),self.x())}
	fn xmzy(&self)->Vec3<T>	{ Vec3::new(self.x(),-self.z(),self.y())}	// 
	fn xzmy(&self)->Vec3<T>	{ Vec3::new(self.x(),self.z(),-self.y())}	// 
*/
// Primitive operations
// TODO move to the appropriate rust stdlib traits where possible

pub trait VecNumOps<T:Num> {
/*:Clone+VecAccessors<T>+VecPermute<T>+VecConsts<T>+Zero*/ 
	fn vadd(&self,b:&Self)->Self;
	fn vsub(&self,b:&Self)->Self;
	fn vfromXYZ(x:T,y:T,z:T)->Self;
}
pub trait VecCmpOps<T:MyOrd> {
	fn vmin(&self,b:&Self)->Self;
	fn vmax(&self,b:&Self)->Self;
}
pub trait VecOps<T:Float>:Clone+VecAccessors<T>+VecPermute<T>+VecConsts<T>+Zero+VecNumOps<T>+VecCmpOps<T> {
// todo: actually, most of these are 'VecNumOps' ; its only things needing 'sqrt' that are 'VecFloatOps'
	fn vscale(&self,f:T)->Self;
	fn vmul(&self,b:&Self)->Self;
	fn vsum_elems(&self)->T;//	{self.x+self.y+self.z}
	fn vcross(&self,b:&Self)->Self;

//	pub fn axis(i:int)->Self;
//	pub fn origin()->Self;


	fn vdot(&self,b:&Self)->T	{self.vmul(b).vsum_elems()}
	fn vpara(&self,vaxis:&Self)->Self {  	let dotp=self.vdot(vaxis); vaxis.vscale(dotp) }

	fn vneg(&self)->Self {self.vscale(-one::<T>())}
	fn vavr(&self,b:&Self)->Self {self.vadd(b).vscale(one::<T>()/(one::<T>()+one::<T>()))}
	fn vmad(&self,b:&Self,f:T)->Self	{self.vadd(&b.vscale(f))}
	fn vlerp(&self,b:&Self,f:T)->Self	{self.vmad(&b.vsub(self),f)}
	fn vsqr(&self)->T { self.vdot(self)}
	fn vlength(&self)->T { self.vsqr().sqrt()}
	fn vinvLength(&self)->T { one::<T>()/self.vsqr().sqrt()}
	fn vtoLength(&self,length:T)->Self { self.vscale(length/self.vsqr().sqrt()) }
	fn vnormalize(&self)->Self { self.vscale(one::<T>()/sqrt(self.vsqr())) }
	fn vperp(&self,axis:&Self)->Self { let vpara =self.vpara(axis); self.vsub(&vpara)}
	fn vcross_norm(&self, b:&Self)->Self { self.vcross(b).vnormalize() }
	fn vsub_norm(&self,b:&Self)->Self { self.vsub(b).vnormalize() }
	//pub fn axisScale(i:int,f:VScalar)->Self;
	// { VecOps::axis(i).vscale(f)} how?

	fn vparaPerp(&self,vaxis:&Self)->(Self,Self) {
		let vpara=self.vpara(vaxis);
		(vpara.clone(),self.vsub(&vpara))
		// ^^^^ CLONE STILL NEEDED HERE ?
		// - possible bug because the default impl hasn't been told Self:Clone
		//   from the trait itself;
		// - VERIFIED that when this function is defined in the IMPL, 
		//		the .clone() is not needed.
	}

	fn vcross_to_vec3(&self,b:&Self)->Vec3<T>;
}

// free function interface to vec maths
pub fn vadd<T:Float,V:VecOps<T>>(a:&V,b:&V)->V { a.vadd(b)}
pub fn vsub<T:Float,V:VecOps<T>>(a:&V,b:&V)->V { a.vsub(b)}
pub fn vmad<T:Float,V:VecOps<T>>(a:&V,b:&V,f:T)->V { a.vadd(&b.vscale(f))}
pub fn vmul<T:Float,V:VecOps<T>>(a:&V,b:&V)->V { a.vmul(b)}
pub fn vsqr<T:Float,V:VecOps<T>>(a:&V)->T { vmul(a,a).vsum_elems()}
pub fn vlerp<T:Float,V:VecOps<T>>( a:&V,b:&V,f:T)->V { vmad(a, &vsub(b,a), f) }
pub fn vdot<T:Float,V:VecOps<T>>( a:&V,b:&V)->T { a.vmul(b).vsum_elems()}
pub fn vlength<T:Float,V:VecOps<T>>( a:&V)->T { a.vmul(a).vsum_elems().sqrt()}
pub fn vnormalize<T:Float,V:VecOps<T>>( a:&V)->V { a.vscale(a.vmul(a).vsum_elems().rsqrt()) }
pub fn vsub_norm<T:Float,V:VecOps<T>>(a:&V,b:&V)->V { a.vsub(b).vnormalize() }
pub fn vcross<T:Float,V:VecOps<T>>(a:&V,b:&V)->V { a.vcross(b)}
pub fn vcross_norm<T:Float,V:VecOps<T>>(a:&V,b:&V)->V { a.vcross(b).vnormalize()}

//  wtf this does,t work now
impl<T:Add<T,T>> Add<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn add(&self,rhs:&Vec2<T>)->Vec2<T> { 
		Vec2::<T>{x:self.x+rhs.x   , y:self.y+rhs.y}
	}
}
impl<T:Add<T,T>> Add<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn add(&self,rhs:&Vec3<T>)->Vec3<T> { 
		Vec3::<T>{x:self.x+rhs.x   , y:self.y+rhs.y, z:self.z+rhs.z}
	}
}
impl<T:Add<T,T>> Add<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn add(&self,rhs:&Vec4<T>)->Vec4<T> { 
		Vec4::<T>{x:self.x+rhs.x   , y:self.y+rhs.y, z:self.z+rhs.z, w:self.w+rhs.w}
	}
}


impl<T:Eq+Zero> Zero for Vec2<T> {
	fn zero()->Vec2<T> {Vec2::<T> {x:zero::<T>(),y:zero::<T>()}}
	fn is_zero(&self)->bool { true }

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
fn vecAxisScale<T:Float,V:VecConsts<T>+VecOps<T>>(i:int,f:T)->V { let ret:V; ret=VecConsts::vaxis(i); ret.vscale(f) }

impl<T:Clone+One+Zero> VecPermute<T> for Vec2<T> {
}

	// todo-trait VecPrimOps
impl<T:Clone+Num> VecNumOps<T> for Vec2<T> {
//	fn vadd(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(zero(),zero())}
//	fn vsub(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(zero(),zero())}
//	fn min(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(zero(),zero())}
//	fn max(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(zero(),zero())}

	fn vadd(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(self.x+b.x,self.y+b.y)}
	fn vsub(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(self.x-b.x,self.y-b.y)}
	fn vfromXYZ(x:T,y:T,_:T)->Vec2<T>{Vec2::new(x.clone(),y.clone())}
}
impl<T:Clone+MyOrd> VecCmpOps<T> for Vec2<T> {
	fn vmin(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(MyOrd::min(self.x.clone(),b.x.clone()),MyOrd::min(self.y.clone(),b.y.clone()))}
	fn vmax(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(MyOrd::max(self.x.clone(),b.x.clone()),MyOrd::max(self.y.clone(),b.y.clone()))}
	
}
impl<T:Clone+Float+MyOrd> VecOps<T> for Vec2<T> {
	fn vscale(&self,f:T)->Vec2<T>		{Vec2::new(self.x*f,self.y*f)}
	fn vmul(&self,b:&Vec2<T>)->Vec2<T>	{Vec2::new(self.x*b.x,self.y*b.y)}
	fn vsum_elems(&self)->T	{self.x+self.y}
	// todo .. not entirely happy with this interface.
	// cross product for a vector type returning its own type seems awkward
	// perhaps 'crossToSelf and crossToVec3' .. and 'cross' for vec3 only?
	fn vcross(&self,_:&Vec2<T>)->Vec2<T>{Vec2::new(zero::<T>(),zero::<T>())}
	fn vcross_to_vec3(&self,b:&Vec2<T>)->Vec3<T>	{Vec3::new(zero(),zero(),self.vcrossToScalar(b))}
//	pub fn axisScale(i:int,f:VScalar)->Vec2 { vecAxisScale(i,f) } 
}
impl<T:Clone+Zero> VecAccessors<T> for Vec2<T>
{
	fn vx(&self)->T	{ self.x.clone()}
	fn vy(&self)->T	{ self.y.clone()}
	fn vz(&self)->T	{ zero::<T>()}
	fn vw(&self)->T	{ zero::<T>()}
}

impl<T:Clone+Zero+Eq> Zero for Vec3<T> {
	fn zero()->Vec3<T>{Vec3::new(zero::<T>(),zero::<T>(),zero::<T>())}
	fn is_zero(&self)->bool  {self.x.is_zero() && self.y.is_zero() && self.z.is_zero()}
}

impl<T:Clone+Zero+One> VecConsts<T> for Vec3<T> {
	fn one()->Vec3<T>	{Vec3::new(one::<T>(),one::<T>(),one::<T>())}
	fn origin()->Vec3<T>	{Vec3::new(zero::<T>(),zero::<T>(),zero::<T>())}
	fn vaxis(i:int)->Vec3<T>{
		match i{ 0=>Vec3::new(one::<T>(),zero::<T>(),zero::<T>()),
                1=>Vec3::new(zero::<T>(),one::<T>(),zero::<T>()),
                2=>Vec3::new(zero::<T>(),zero::<T>(),one::<T>()),
                _=>Vec3::new(zero::<T>(),zero::<T>(),zero::<T>())}
	}
}
impl<T:Clone+Zero+One> VecConsts<T> for Vec2<T> {
	fn one()->Vec2<T>	{Vec2::new(one::<T>(),one::<T>())}
	fn origin()->Vec2<T>	{Vec2::new(zero::<T>(),zero::<T>())}
	fn vaxis(i:int)->Vec2<T>{
		match i{ 0=>Vec2::new(one::<T>(),zero::<T>()),1=>Vec2::new(zero::<T>(),one::<T>()),_=>Vec2::new(zero::<T>(),zero::<T>())}
	}
}



impl<T:Clone+One+Zero> VecPermute<T> for Vec3<T> {
}

impl<T:Clone+Num> VecNumOps<T> for Vec3<T> {
	fn vadd(&self,b:&Vec3<T>)->Vec3<T>	{Vec3::new(self.x+b.x,self.y+b.y,self.z+b.z)}
	fn vsub(&self,b:&Vec3<T>)->Vec3<T>	{Vec3::new(self.x-b.x,self.y-b.y,self.z-b.z)}
	fn vfromXYZ(x:T,y:T,z:T)->Vec3<T>{Vec3::new(x,y,z)}
}
impl<T:Clone+Ord+MyOrd> VecCmpOps<T> for Vec3<T> {
	fn vmin(&self,b:&Vec3<T>)->Vec3<T>	{Vec3::new(
									MyOrd::min(self.x.clone(),b.x.clone()),
									MyOrd::min(self.y.clone(),b.y.clone()),
									MyOrd::min(self.z.clone(),b.z.clone()))}
	fn vmax(&self,b:&Vec3<T>)->Vec3<T>	{Vec3::new(
									MyOrd::max(self.x.clone(),b.x.clone()),
									MyOrd::max(self.y.clone(),b.y.clone()),
									MyOrd::max(self.z.clone(),b.z.clone()))}
}

impl<T:Float+MyOrd> VecOps<T> for Vec3<T> {
	// todo-trait VecPrimOps
	
	fn vscale(&self,f:T)->Vec3<T>		{Vec3::new(self.x*f,self.y*f,self.z*f)}
	fn vmul(&self,b:&Vec3<T>)->Vec3<T>	{Vec3::new(self.x*b.x,self.y*b.y,self.z*b.z)}
	fn vsum_elems(&self)->T	{self.x+self.y+self.z}
	fn vcross(&self,b:&Vec3<T>)->Vec3<T>	{Vec3::new(self.y*b.z-self.z*b.y,self.z*b.x-self.x*b.z,self.x*b.y-self.y*b.x)}

	fn vcross_to_vec3(&self,b:&Vec3<T>)->Vec3<T>	{self.vcross(b)}
	//fpub fn axisScale(i:int,f:VScalar)->Vec3 { VecConsts::axis(i).scale(f)} 
}
impl<T:Clone+Zero+One> VecAccessors<T> for Vec3<T> {
	fn vx(&self)->T	{ self.x.clone()}
	fn vy(&self)->T	{ self.y.clone()}
	fn vz(&self)->T	{ self.z.clone()}
	fn vw(&self)->T	{ zero::<T>()}

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
	fn is_zero(&self)->bool  {self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero()}
}
impl<T:Clone+Zero+One> VecConsts<T> for Vec4<T> {
	fn one()->Vec4<T>	{Vec4::new(one::<T>(),one::<T>(),one::<T>(),one::<T>())}
	fn origin()->Vec4<T>	{Vec4::new(zero::<T>(),zero::<T>(),zero::<T>(),one::<T>())}

	fn vaxis(i:int)->Vec4<T>{
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
	fn vto_tuple(&self)->(T,T,T,T) { (self.x.clone(),self.y.clone(),self.z.clone(),self.w.clone()) }
	fn vfrom_tuple((x,y,z,w):(T,T,T,T))->Vec4<T> {
		Vec4{x:x.clone(),y:y.clone(),z:z.clone(),w:w.clone()}
	}
}
impl<T:Clone> Vec3<T> {
	fn vto_tuple(&self)->(T,T,T) { (self.x.clone(),self.y.clone(),self.z.clone()) }
	fn vfrom_tuple((x,y,z):(T,T,T))->Vec3<T> { Vec3{x:x.clone(),y:y.clone(),z:z.clone()} }
}
impl<T:Clone> Vec2<T> {
	fn vto_tuple(&self)->(T,T) { (self.x.clone(),self.y.clone()) }
	fn vfrom_tuple((x,y):(T,T))->Vec2<T> { Vec2{x:x.clone(),y:y.clone()} }
}

impl<T:Clone+One+Zero> VecPermute<T> for Vec4<T> {
}


impl<T:Clone+Num> VecNumOps<T> for Vec4<T> {
	fn vadd(&self,b:&Vec4<T>)->Vec4<T>	{Vec4::new(self.x+b.x,self.y+b.y,self.z+b.z,self.w+b.w)}
	fn vsub(&self,b:&Vec4<T>)->Vec4<T>	{Vec4::new(self.x-b.x,self.y-b.y,self.z-b.z,self.w-b.w)}
	fn vfromXYZ(x:T,y:T,z:T)->Vec4<T>{Vec4::new(x,y,z,zero::<T>())}
}
impl<T:Clone+MyOrd> VecCmpOps<T> for Vec4<T> {
	fn vmin(&self,b:&Vec4<T>)->Vec4<T>	{Vec4::new(
									MyOrd::min(self.x.clone(),b.x.clone()),
									MyOrd::min(self.y.clone(),b.y.clone()),
									MyOrd::min(self.z.clone(),b.z.clone()),
									MyOrd::min(self.w.clone(),b.w.clone()))}
	fn vmax(&self,b:&Vec4<T>)->Vec4<T>	{Vec4::new(
									MyOrd::max(self.x.clone(),b.x.clone()),
									MyOrd::max(self.y.clone(),b.y.clone()),
									MyOrd::max(self.z.clone(),b.z.clone()),
									MyOrd::max(self.w.clone(),b.w.clone()))}
}
impl<T:Clone+Float+MyOrd> VecOps<T> for Vec4<T> {
	// todo-trait VecPrimOps


	fn vscale(&self,f:T)->Vec4<T>		{Vec4::new(self.x*f,self.y*f,self.z*f,self.w*f)}
	fn vmul(&self,b:&Vec4<T>)->Vec4<T>	{Vec4::new(self.x*b.x,self.y*b.y,self.z*b.z,self.w*b.w)}
	fn vsum_elems(&self)->T	{self.x+self.y+self.z+self.w}
//	pub fn zero()->Vec4{Vec4{x:0.0,y:0.0,z:0.0,w:0.0}}

	fn vcross(&self,b:&Vec4<T>)->Vec4<T>	{Vec4::new(self.y*b.z-self.z*b.y,self.z*b.x-self.x*b.z,self.x*b.y-self.y*b.x,zero::<T>())}

	fn vcross_to_vec3(&self,b:&Vec4<T>)->Vec3<T>	{self.vcross(b).vxyz()}


	//pub fn axisScale(i:int,f:VScalar)->Vec4 { VecConsts::axis(i).scale(f)}
}

impl<T:Clone> VecAccessors<T> for Vec4<T>
{
	fn vx(&self)->T	{ self.x.clone()}
	fn vy(&self)->T	{ self.y.clone()}
	fn vz(&self)->T	{ self.z.clone()}
	fn vw(&self)->T	{ self.w.clone()}
}

pub fn vecNormalize<T:Float,V:VecOps<T>>(v:&V)->V { v.vscale(v.vsqr().rsqrt()) }
// TOOD - Zero for Vec3<T> , Vec4<T>


/*
fn repeated<T:Num+Ord>(count:T, myfunc:&fn(x:T)->T){
	let mut i:T=count;
	while i>core::Zero::zero() { myfunc(i); i-=(1 as T)}
}
*/
/*
	use core::io;
	let a:~[i32]=~[1,2,3,4,5,6];
	let x=a.len();
	let r=do a.map |x|{*x +1};
	io::println(r[0].to_str());
	io::println(lerp!(1.0f,2.0f by: 0.4f between: 0.3f,0.6f).to_str());
	
	io::println(fooToStr(foo2{x:10.0,y:20.0}));
*/


macro_rules! lerp{
	($y0:expr:$y1:expr by $t:expr)=>
	(
		$y0+($y1-$y0)*$t
	);
	($y0:expr,$y1:expr by: $x:expr between: $x0:expr,$x1:expr)=>
	(
		$y0+($x*($y1-$y0)/($x1-$x0))
	)
}



// experiment ... vector maths on tuples.
// not having to name a custom type, less namespace clutter.



// operations unique to Vec3



/*
pub fn v3add<T:Float>(a:&Vec3<T>, b:&Vec3<T>)->Vec3<T>{
	Vec3{x:a.x+b.x,y:a.y+b.y,z:a.z+b.z}
}
pub fn v3sub<T:Float>(a:&Vec3<T>, b:&Vec3<T>)->Vec3<T>{
	Vec3{x:a.x-b.x,y:a.y-b.y,z:a.z-b.z}
}
pub fn v3scale<T:Float>(a:&Vec3<T>, f:T)->Vec3<T>{
	Vec3{x:a.x*f,y:a.y*f,z:a.z*f}
}
pub fn v3lerp<T:Float>(a:&Vec3<T>,b:&Vec3<T>,f:T)->Vec3<T> {v3add(a,&v3scale(&v3sub(b,a),f))}
*/

#[deriving(Clone,Show)]
pub struct MinMax<T> {  
	min:T,max:T	
}
impl<T:MyOrd+Num,V:VecOps<T>> MinMax<V> { 
	fn size(&self)->V { self.max.vsub(&self.min) }
}
impl<T:MyOrd+Num+Float,V:VecOps<T>> MinMax<V> { 
	fn centre(&self)->V { self.min.vadd(&self.max).vscale(one::<T>()/(one::<T>()+one::<T>())) }
}

// todo - math UT, ask if they can go in the stdlib.

fn clamp<T:Num+MyOrd>(x:T, lo:T, hi:T)->T {
	MyOrd::max(MyOrd::min(x,hi),lo)
}
fn clamp_s<T:Num+MyOrd>(value:T, limit:T)->T {
	clamp(value,-limit,limit)
}
fn deadzone<T:Num+MyOrd+Zero>(value:T, deadzone:T)->T {
	if value<deadzone || value>deadzone { value }
	else {zero()}
}

