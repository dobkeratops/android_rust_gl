use super::vectypes::*;
use super::vecmath::*;
pub use std::cmp::*;

// fuck this shit.
#[deriving(Copy,Clone,Show)]
pub struct Matrix2<AXIS=Vec2<f32>> (pub AXIS,pub AXIS);

#[deriving(Copy,Clone,Show)]
pub struct Matrix33<T=f32> (pub Vec3<T>,pub Vec3<T>,pub Vec3<T>);

#[deriving(Copy,Clone,Show)]
pub struct Matrix34<T=f32> (pub Vec4<T>,pub Vec4<T>,pub Vec4<T>);

#[deriving(Copy,Clone,Show)]
pub struct Matrix44<T=f32> (pub Vec4<T>,pub Vec4<T>,pub Vec4<T>, pub Vec4<T>);

#[deriving(Copy,Clone,Show)]
pub struct Matrix43<T=f32> (pub Vec3<T>,pub Vec3<T>,pub Vec3<T>, pub Vec3<T>);

// todo: Write all matrix maths in terms of this interface
// various concrete datastructures could expose a matrix

pub struct Scaling<T>{
	pub sx:T,
	pub sy:T,
	pub sz:T
}

impl<T:Float> Scaling<T>
{
	pub fn identity()->Scaling<T> { Scaling{sx:one(),sy:one(),sz:one()}}
	pub fn new(x:T,y:T,z:T)->Scaling<T> {Scaling{sx:x,sy:y,sz:z} }
	pub fn from_vec3(v:&Vec3<T>)->Scaling<T> {Scaling{sx:v.x(),sy:v.y(),sz:v.z()}}
	pub fn from_xyz<V:Vector<T>>(v:&V)->Scaling<T> {Scaling{sx:v.x(),sy:v.y(),sz:v.z()}}
	pub fn to_vec3(&self, v:&Vec3<T>)->Vec3<T> { Vec3(self.sx,self.sy,self.sz)}
	pub fn to_vec4(&self, v:&Vec3<T>)->Vec3<T> { Vec3(self.sx,self.sy,self.sz)}
}

struct RotateX<T>(T);
struct RotateY<T>(T);
struct RotateZ<T>(T);

pub trait GetAxes<V=Vec3> {
	fn ax<'a>(&'a self)->&'a V;
	fn ay<'a>(&'a self)->&'a V;
	fn az<'a>(&'a self)->&'a V;
}

pub trait Transpose<OUT> {
	fn transpose(&self)->OUT;
}
impl<V> Matrix2<V> {
	pub fn ax<'a>(&'a self)->&'a V { &self.0}
	pub fn ay<'a>(&'a self)->&'a V { &self.1}
}

impl<'a, T> GetAxes<Vec3<T>> for  Matrix33<T> {
	fn ax<'a>(&'a self)->&'a Vec3<T> { &self.0}
	fn ay<'a>(&'a self)->&'a Vec3<T> { &self.1}
	fn az<'a>(&'a self)->&'a Vec3<T> { &self.2}
}

// Accessor for axes, by value: may be efficeint to construct axes simultaneously.
// individual acessors may be overriden if its' convenient
pub trait ToAxes<V> {
	fn axis_x(&self)->V {self.axes().0}
	fn axis_y(&self)->V {self.axes().1}
	fn axis_z(&self)->V {self.axes().2}
	fn axes(&self)->(V,V,V);
}
pub trait SetAxes<V> {
	fn set_ax(&mut self,a:V);
	fn set_ay(&mut self,a:V);
	fn set_az(&mut self,a:V);
	fn set_axes(&mut self,ax:V,ay:V,az:V){ self.set_ax(ax);self.set_ay(ay); self.set_az(az);}
}
trait Axes<V>:GetAxes<V>+SetAxes<V>{}
impl<V:Copy,M:GetAxes<V>+SetAxes<V>> Axes<V> for M{}

//impl<V:Copy> ToAxes<V> for Matrix4<V> {
//	fn axes(&self)->(V,V,V) {
//		(self.0,self.1,self.2)
//	}
//}
impl<T:Float+Copy,V:ToVec3<T>> SetAxes<V> for Matrix43<T> {
	fn set_ax(&mut self, a:V){
		self.0=a.to_vec3();
	}
	fn set_ay(&mut self, a:V){
		self.1=a.to_vec3();
	}
	fn set_az(&mut self, a:V){
		self.2=a.to_vec3();
	}
}
impl<T:Float+Copy,V:ToVec4<T>> SetAxes<V> for Matrix44<T> {
	fn set_ax(&mut self, a:V){
		self.0=a.to_vec4();
	}
	fn set_ay(&mut self, a:V){
		self.1=a.to_vec4();
	}
	fn set_az(&mut self, a:V){
		self.2=a.to_vec4();
	}
}
impl<T:Float+Copy,V:ToVec3<T>> SetAxes<V> for Matrix33<T> {
	fn set_ax(&mut self, a:V){
		self.0=a.to_vec3();
	}
	fn set_ay(&mut self, a:V){
		self.1=a.to_vec3();
	}
	fn set_az(&mut self, a:V){
		self.2=a.to_vec3();
	}
}
impl<T:Float+Copy,V:ToVec4<T>> SetAxes<V> for Matrix34<T> {
	fn set_ax(&mut self, a:V){
		self.0=a.to_vec4();
	}
	fn set_ay(&mut self, a:V){
		self.1=a.to_vec4();
	}
	fn set_az(&mut self, a:V){
		self.2=a.to_vec4();
	}
}

impl<T> GetAxes<Vec4<T>> for Matrix44<T> {
	fn ax<'a>(&'a self)->&'a Vec4<T> { &self.0}
	fn ay<'a>(&'a self)->&'a Vec4<T> { &self.1}
	fn az<'a>(&'a self)->&'a Vec4<T> { &self.2}
}

//impl<V> Matrix4<V> {
//	pub fn aw<'a>(&'a self)->&'a V { &self.3}
//}
impl<T:Float+Copy> Pos<Vec4<T>> for Matrix44<T> {
	fn pos(&self)->Vec4<T> { self.3}
	fn set_pos(&mut self ,v:&Vec4<T>) { self.3=v.to_point();}
}



pub trait Matrix<T:Float> {
	fn transpose(&self)->Self;
	fn to_mat33(&self)->Matrix33<T>;
	fn to_mat43(&self)->Matrix43<T>;
	fn to_mat34(&self)->Matrix34<T>;
	fn inv_orthonormal(&self)->Self;

	fn orthonormalize_zyx(&self)->Self;
	fn orthonormalize_yzx(&self)->Self;

	fn mul_vec4(&self,pt:&Vec4<T>)->Vec4<T>;
	fn mul_point(&self,pt:&Vec3<T>)->Vec3<T>;
	fn mul_axis(&self,pt:&Vec3<T>)->Vec3<T>;
	fn inv_mul_point(&self,pt:&Vec3<T>)->Vec3<T>;
	fn inv_mul_axis(&self,axis:&Vec3<T>)->Vec3<T>;
	fn inv_mul_vec4(&self,pt:&Vec4<T>)->Vec4<T>;
	fn mul_vec3_w0(&self,pt:&Vec3<T>)->Vec3<T>{ self.mul_axis(pt)}
	fn mul_vec3_w1(&self,pt:&Vec3<T>)->Vec3<T>{ self.mul_point(pt)}

	fn mul_matrix(&self,other:&Self)->Self;
	fn inv_mul_matrix(&self,other:&Self)->Self;
	fn lerp(&self, other:&Self, f:T)->Self;
}

impl<T:Copy+Float> Matrix44<T> {
	pub fn identity()->Matrix44<T>{
		Matrix44::<T>(
			Vec4::<T>::axis(0),
			Vec4::<T>::axis(1),
			Vec4::<T>::axis(2),
			Vec4::<T>::axis(3))
	}
	pub fn translate(trans:&Vec4<T>)->Matrix44<T>{
		Matrix44::<T>(
			Vec4::<T>::axis(0),
			Vec4::<T>::axis(1),
			Vec4::<T>::axis(2),
//			Vec4::<T>::axis(3))
			*trans)
	}

	pub fn look_along(pos:&Vec4<T>,fwd:&Vec4<T>,up:&Vec4<T>)->Matrix44<T>{
		let az=fwd.normalize();
		let ax=up.cross(&az).normalize();
		let ay=az.cross(&ax);
		Matrix44(ax,ay,az,pos.to_point())
	}
	// Variation of 'look-at' calculation that prioritizes upvector
	pub fn look_up_along(pos:&Vec4<T>,up:&Vec4<T>,fwd:&Vec4<T>)->Matrix44<T>{
		let ay=up.normalize();
		let ax=fwd.cross(up).normalize();
		let az=ax.cross(&ay);
		Matrix44(ax,ay,az,pos.to_point())
	}

	pub fn look_at(pos:&Vec4<T>,target:&Vec4<T>,up:&Vec4<T>)->Matrix44<T> { Matrix44::look_along(pos,&target.sub(pos),up) }


	pub fn from_vec4(ax:Vec4<T>,ay:Vec4<T>,az:Vec4<T>,aw:Vec4<T>)->Matrix44<T> {
		Matrix44(ax,ay,az,aw)
	}
	pub fn from_vec3(ax:Vec3<T>,ay:Vec3<T>,az:Vec3<T>,aw:Vec3<T>)->Matrix44<T> {
		Matrix44(ax.to_vec4(),ay.to_vec4(),az.to_vec4(),(aw,one()).to_vec4())
	}

	pub fn from_mat33(mat33:&Matrix33<T>)->Matrix44<T> {
		Matrix44::from_mat33_pos(mat33,&Vec4(zero(),zero(),zero(),one()))
	}
	pub fn from_mat33_pos(mat33:&Matrix33<T>,pos:&Vec4<T>)->Matrix44<T> {
		Matrix44(
			Vec4::from_vec3(&mat33.0,zero()),
			Vec4::from_vec3(&mat33.1,zero()),
			Vec4::from_vec3(&mat33.2,zero()),
			*pos)
	}
	pub fn from_mat43(mat43:&Matrix43<T>)->Matrix44<T> {
		Matrix44(
			Vec4::from_vec3(&mat43.0,zero()),
			Vec4::from_vec3(&mat43.1,zero()),
			Vec4::from_vec3(&mat43.2,zero()),
			Vec4::from_vec3(&mat43.3,one()))
	}
	pub fn from_mat34_pos(mat:&Matrix34<T>,pos:&Vec4<T>)->Matrix44<T> {
		Matrix44(mat.0,mat.1,mat.2,*pos)
	}
}
 
impl<T:Copy+Float> Matrix34<T> {
	pub fn mul_vec3_w0(&self,p:&Vec3<T>) -> Vec4<T> {
		self.0.scale(p.0) + self.1.scale(p.1) + self.2.scale(p.2)
	}
}

// Indirection Traits via PreMulMatXX to get Matrix*Matrix and Matrix*Vector
//impl<T:Float, OUT, RHS:PreMulMat44<T,OUT> > Mul<RHS,OUT> for Matrix44<T> {
//	fn mul(&self, rhs:&RHS)->OUT { rhs.pre_mul_mat44(self) } 
//}
//impl<T:Float, OUT, RHS:PreMulMat33<T,OUT> > Mul<RHS,OUT> for Matrix33<T> {
//	fn mul(&self, rhs:&RHS)->OUT { rhs.pre_mul_mat33(self) } 
//}
//impl<T:Float, V:VecMath<T>,OUT, RHS:PreMulMat4<V,OUT> > Mul<RHS,OUT> for Matrix4<V> {
//	fn mul(&self, rhs:&RHS)->OUT { rhs.pre_mul_mat4(self) } 
//}
/*
impl<T:Copy+Float> Transpose<Matrix3<Vec4<T>>> for Matrix4<Vec3<T>> {
	fn transpose(&self)->Matrix3<Vec4<T>> {
		// todo-SIMD..
		Matrix3(
			Vec4(self.0 .0, self.1 .0, self.2 .0, self.3 .0),
			Vec4(self.0 .1, self.1 .1, self.2 .1, self.3 .1),
			Vec4(self.0 .2, self.1 .2, self.2 .2, self.3 .2),
		)
	}
}
impl<T:Float> Transpose<Matrix4<Vec3<T>,Vec3<T>>> for Matrix3<Vec4<T>> {
	fn transpose(&self)->Matrix4<Vec3<T>> {
		// todo-SIMD..
		Matrix4(
			Vec3(self.0 .0, self.1 .0, self.2 .0),
			Vec3(self.0 .1, self.1 .1, self.2 .1),
			Vec3(self.0 .2, self.1 .2, self.2 .2),
			Vec3(self.0 .3, self.1 .3, self.2 .3)
		)
	}
}

impl<T:Float> Transpose<Matrix33<T>> for Matrix3<Vec3<T>> {
	fn transpose(&self)->Matrix3<Vec3<T>> {
		// todo-SIMD..
		Matrix3(
			Vec3(self.0 .0, self.1 .0, self.2 .0),
			Vec3(self.0 .1, self.1 .1, self.2 .1),
			Vec3(self.0 .2, self.1 .2, self.2 .2),
		)
	}
}
*/
impl<T:Float=f32> Matrix<T> for Matrix44<T> {

	fn transpose(&self)->Matrix44<T> {
		Matrix44(
			Vec4(self.0 .0, self.1 .0, self.2 .0, self.3 .0),
			Vec4(self.0 .1, self.1 .1, self.2 .1, self.3 .1),
			Vec4(self.0 .2, self.1 .2, self.2 .2, self.3 .1),
			Vec4(self.0 .3, self.1 .3, self.2 .3, self.3 .2)
		)
	}
	fn to_mat33(&self)->Matrix33<T> {
		Matrix33(self.0.xyz(),self.1.xyz(),self.2.xyz())
	}
	fn to_mat43(&self)->Matrix43<T> {
		Matrix43(self.0.xyz(),self.1.xyz(),self.2.xyz(),self.3.xyz())
	}
	fn to_mat34(&self)->Matrix34<T> {
		Matrix34(self.0,self.1,self.2)
	}

	fn orthonormalize_zyx(&self)->Matrix44<T> {
		Matrix44::look_along(&self.3,&self.2,&self.1)
	}
	fn orthonormalize_yzx(&self)->Matrix44<T> {
		Matrix44::look_up_along(&self.3,&self.1,&self.2)
	}
//	pub fn mul_vec(&self,pt:&Vec4<T>)->Vec4<T>{	// 'vec'=x,y,z,w
//		self.0.scale(pt.0).macc(&self.1,pt.1).macc(&self.2,pt.2).macc(&self.3,pt.3)
//	}
	fn mul_point(&self,pt:&Vec3<T>)->Vec3<T>{	// 'point'=x,y,z,1
		self.3.macc(&self.0,pt.x()).macc(&self.1,pt.y()).macc(&self.2,pt.z()).to_vec3()
	}
	fn mul_axis(&self,pt:&Vec3<T>)->Vec3<T>{	// 'axis'=x,y,z,0
		self.0.scale(pt.x()).macc(&self.1,pt.y()).macc(&self.2,pt.z()).to_vec3()
	}
	fn inv_mul_point(&self,pt:&Vec3<T>)->Vec3<T>{
		let ofs=pt.sub(&self.3.to_vec3());
		Vec3(ofs.dot(&self.0.to_vec3()),ofs.dot(&self.1.to_vec3()),ofs.dot(&self.2.to_vec3()))
	}
	fn inv_mul_axis(&self,axis:&Vec3<T>)->Vec3<T>{
		Vec3(axis.dot(&self.0.to_vec3()),axis.dot(&self.1.to_vec3()),axis.dot(&self.2.to_vec3()))
	}
	fn inv_mul_vec4(&self,pt:&Vec4<T>)->Vec4<T>{
		let mut ofs=pt.sub(&self.3); ofs.3=zero();
		Vec4(ofs.dot(&self.0),ofs.dot(&self.1),ofs.dot(&self.2),pt.3)
	}
	fn mul_vec3_w0(&self,pt:&Vec3<T>)->Vec3<T>{
		self.0 .scale(pt.0).macc(&self.1,pt.1).macc(&self.2,pt.2).to_vec3()
	}
	fn mul_vec3_w1(&self,pt:&Vec3<T>)->Vec3<T>{
		self.3 .macc(&self.0,pt.0).macc(&self.1,pt.1).macc(&self.2,pt.2).to_vec3()
	}
	fn mul_vec4(&self,pt:&Vec4<T>)->Vec4<T>{
		self.0 .scale(pt.0).macc(&self.1,pt.1).macc(&self.2,pt.2).macc(&self.3,pt.3)
	}
	fn mul_matrix(&self,other:&Matrix44<T>)->Matrix44<T> {
		Matrix44(
			self.mul_vec4(&other.0),
			self.mul_vec4(&other.1),
			self.mul_vec4(&other.2),
			self.mul_vec4(&other.3))
	}
	fn inv_mul_matrix(&self,other:&Matrix44<T>)->Matrix44<T> {
		// TODO assert affine assumption
		Matrix44(
			self.inv_mul_axis(&other.0.to_vec3()).to_vec4(),
			self.inv_mul_axis(&other.1.to_vec3()).to_vec4(),
			self.inv_mul_axis(&other.2.to_vec3()).to_vec4(),
			self.inv_mul_point(&other.3.to_vec3()).to_vec4_pos())
	}
	// matrix inverse with assumption of being orthonormalized.
	fn inv_orthonormal(&self)->Matrix44<T> {
		let ax=Vec4(self.0 .0,self.1 .0,self.2 .0, zero());
		let ay=Vec4(self.0 .1,self.1 .1,self.2 .1, zero());
		let az=Vec4(self.0 .2,self.1 .2,self.2 .2, zero());
		let aw=self.inv_mul_axis(&-self.3.to_vec3());
		Matrix44(ax,ay,az,aw.to_vec4_pos())
	}

	fn lerp(&self,other:&Matrix44<T>,t:T)->Matrix44<T> {
		Matrix44(
			self.0.lerp(&other.0,t),
			self.1.lerp(&other.1,t),
			self.2.lerp(&other.2,t),
			self.3.lerp(&other.3,t))
	}

}


impl<T:Float=f32> Matrix33<T> {
	pub fn identity()->Matrix33<T>{
		Matrix33::<T>(
			Vec3::<T>::axis(0),
			Vec3::<T>::axis(1),
			Vec3::<T>::axis(2))
	}
	fn transpose(&self)->Matrix44<T> {
		Matrix44(
			Vec4(self.0 .0, self.1 .0, self.2 .0, zero()),
			Vec4(self.0 .1, self.1 .1, self.2 .1, zero()),
			Vec4(self.0 .2, self.1 .2, self.2 .2, zero()),
			Vec4(zero(), zero(), zero(), one())
		)
	}
	fn transpose33(&self)->Matrix33<T> {
		Matrix33(
			Vec3(self.0 .0, self.1 .0, self.2 .0),
			Vec3(self.0 .1, self.1 .1, self.2 .1),
			Vec3(self.0 .2, self.1 .2, self.2 .2),
		)
	}

	pub fn look_fwd_up(fwd:&Vec3<T>,up:&Vec3<T>)->Matrix33<T>{
		let az=fwd.normalize();
		let ax=up.cross(&az).normalize();
		let ay=az.cross(&ax);
		Matrix33(ax,ay,az)
	}
	// Variation of 'look-at' calculation that prioritizes upvector
	pub fn look_up_fwd(up:&Vec3<T>,fwd:&Vec3<T>)->Matrix33<T>{
		let ay=up.normalize();
		let ax=fwd.cross(up).normalize();
		let az=ax.cross(&ay);
		Matrix33(ax,ay,az)
	}

	pub fn look_at(pos:&Vec3<T>,target:&Vec3<T>,up:&Vec3<T>)->Matrix33<T> { Matrix33::look_fwd_up(&target.sub(pos),up) }

	pub fn orthonormalize_zyx(&self)->Matrix33<T> {
		Matrix33::look_fwd_up(&self.2,&self.1)
	}
	pub fn orthonormalize_yzx(&self)->Matrix33<T> {
		Matrix33::look_up_fwd(&self.1,&self.2)
	}
	pub fn mul_vec4(&self,pt:&Vec4<T>)->Vec4<T>{
		self.0.scale(pt.0).macc(&self.1,pt.1).macc(&self.2,pt.2).to_vec4()
	}
	pub fn mul_point(&self,pt:&Vec3<T>)->Vec3<T>{	// 'point'=x,y,z,1
		self.0.scale(pt.x()).macc(&self.1,pt.y()).macc(&self.2,pt.z())
	}
	pub fn mul_axis(&self,pt:&Vec3<T>)->Vec3<T>{	// 'axis'=x,y,z,0
		self.0.scale(pt.x()).macc(&self.1,pt.y()).macc(&self.2,pt.z())
	}
	pub fn inv_mul_point(&self,pt:&Vec3<T>)->Vec3<T>{
		let ofs=pt;
		Vec3(ofs.dot(&self.0),ofs.dot(&self.1),ofs.dot(&self.2))
	}
	pub fn inv_mul_axis(&self,axis:&Vec3<T>)->Vec3<T>{
		Vec3(axis.dot(&self.0),axis.dot(&self.1),axis.dot(&self.2))
	}
	pub fn mul_matrix(&self,other:&Matrix33<T>)->Matrix33<T> {
		Matrix33(
			self.mul_axis(&other.0),
			self.mul_axis(&other.1),
			self.mul_axis(&other.2))
	}
	pub fn inv_mul_matrix(&self,other:&Matrix33<T>)->Matrix33<T> {
		Matrix33(
			self.inv_mul_axis(&other.0),
			self.inv_mul_axis(&other.1),
			self.inv_mul_axis(&other.2))
	}
}


// Todo: trait Matrix.
impl<T:Float=f32> Matrix43<T> {
	pub fn identity()->Matrix43<T>{
		Matrix43(
			Vec3::<T>::axis(0),
			Vec3::<T>::axis(1),
			Vec3::<T>::axis(2),
			Vec3::<T>::axis(3))
	}
	pub fn translate(trans:&Vec3<T>)->Matrix43<T>{
		Matrix43::<T>(
			Vec3::<T>::axis(0),
			Vec3::<T>::axis(1),
			Vec3::<T>::axis(2),
//			Vec4::<T>::axis(3))
			*trans)
	}	

	fn transpose(&self)->Matrix44<T> {
		Matrix44(
			Vec4(self.0 .0, self.1 .0, self.2 .0, self.3 .0),
			Vec4(self.0 .1, self.1 .1, self.2 .1, self.3 .1),
			Vec4(self.0 .2, self.1 .2, self.2 .2, self.3 .1),
			Vec4(zero(), zero(), zero(), one())
		)
	}
	fn transpose33(&self)->Matrix33<T> {
		Matrix33(
			Vec3(self.0 .0, self.1 .0, self.2 .0),
			Vec3(self.0 .1, self.1 .1, self.2 .1),
			Vec3(self.0 .2, self.1 .2, self.2 .2),
		)
	}

	pub fn look_along(pos:&Vec3<T>,fwd:&Vec3<T>,up:&Vec3<T>)->Matrix43<T>{
		let az=fwd.normalize();
		let ax=up.cross(&az).normalize();
		let ay=az.cross(&ax);
		Matrix43(ax,ay,az,pos.to_point())
	}
	// Variation of 'look-at' calculation that prioritizes upvector
	pub fn look_up_along(pos:&Vec3<T>,up:&Vec3<T>,fwd:&Vec3<T>)->Matrix43<T>{
		let ay=up.normalize();
		let ax=fwd.cross(up).normalize();
		let az=ax.cross(&ay);
		Matrix43(ax,ay,az,pos.to_point())
	}

	pub fn look_at(pos:&Vec3<T>,target:&Vec3<T>,up:&Vec3<T>)->Matrix43<T> { Matrix43::look_along(pos,&target.sub(pos),up) }
	pub fn orthonormalize_zyx(&self)->Matrix43<T> {
		Matrix43::look_along(&self.3,&self.2,&self.1)
	}
	pub fn orthonormalize_yzx(&self)->Matrix43<T> {
		Matrix43::look_up_along(&self.3,&self.1,&self.2)
	}
	pub fn mul_vec(&self,pt:&Vec3<T>)->Vec3<T>{	// 'vec'=x,y,z,w
		self.0.scale(pt.x()).macc(&self.1,pt.y()).macc(&self.2,pt.z()).macc(&self.3,pt.w())
	}
	pub fn mul_point(&self,pt:&Vec3<T>)->Vec3<T>{	// 'point'=x,y,z,1
		self.3.macc(&self.0,pt.x()).macc(&self.1,pt.y()).macc(&self.2,pt.z())
	}
	pub fn mul_axis(&self,pt:&Vec3<T>)->Vec3<T>{	// 'axis'=x,y,z,0
		self.0.scale(pt.x()).macc(&self.1,pt.y()).macc(&self.2,pt.z())
	}
	pub fn inv_mul_point(&self,pt:&Vec3<T>)->Vec3<T>{
		let ofs=pt.sub(&self.3);
		Vec3(ofs.dot(&self.0),ofs.dot(&self.1),ofs.dot(&self.2))
	}
	pub fn inv_mul_axis(&self,axis:&Vec3<T>)->Vec3<T>{
		Vec3(axis.dot(&self.0),axis.dot(&self.1),axis.dot(&self.2))
	}
	pub fn mul_vec3_w0(&self,pt:&Vec3<T>)->Vec3<T>{
		let Vec3(x,y,z)=*pt;
		self.0 .scale(x).macc(&self.1,y).macc(&self.2,z).to_vec3()
	}
	pub fn mul_vec3_w1(&self,pt:&Vec3<T>)->Vec3<T>{
		let Vec3(x,y,z)=*pt;
		self.3 .macc(&self.0,x).macc(&self.1,y).macc(&self.2,z).to_vec3()
	}
	pub fn mul_vec4(&self,pt:&Vec4<T>)->Vec4<T>{
		self.0 .scale(pt.0).macc(&self.1,pt.1).macc(&self.2,pt.2).macc(&self.3,pt.3).to_vec4()
	}
	pub fn mul_matrix(&self,other:&Matrix43<T>)->Matrix43<T> {
		Matrix43(
			self.mul_vec(&other.0),
			self.mul_vec(&other.1),
			self.mul_vec(&other.2),
			self.mul_vec(&other.3))
	}
	// matrix inverse with assumption of being orthonormalized.
	pub fn inv_orthonormal(&self)->Matrix43<T> {
		let ax=Vec3(self.0 .0,self.1 .0,self.2 .0);
		let ay=Vec3(self.0 .1,self.1 .1,self.2 .1);
		let az=Vec3(self.0 .2,self.1 .2,self.2 .2);
		let aw=self.inv_mul_axis(&-self.3);
		Matrix43(ax,ay,az,aw.to_point())
	}
	// todo: full 
}


/*
impl<T:Float> Matrix3<Vec3<T>> {
	pub fn identity()->Matrix3<Vec3<T>>{
		Matrix3(
			Vec3::<T>::axis(0),
			Vec3::<T>::axis(1),
			Vec3::<T>::axis(2))
	}
	pub fn mul_vec3(&self,pt:&Vec3<T>)->Vec3<T>{
		self.0.mul_x(pt).add_mul_y(&self.1,pt).add_mul_z(&self.2,pt)
	}
	pub fn mul_matrix(&self,other:&Matrix3<Vec3<T>>)->Matrix3<Vec3<T>> {
		Matrix3(
			self.mul_vec3(&other.0),
			self.mul_vec3(&other.1),
			self.mul_vec3(&other.2))
	}
}
*/
// operator overload: vector*matrix - assumes vec is transposed

impl<T:Float> PreMulVec4<T,Vec4<T>> for Matrix44<T> {
	fn pre_mul_vec4(&self,v: &Vec4<T>)->Vec4<T> {
		Vec4(
			v.dot(&self.0),
			v.dot(&self.1),
			v.dot(&self.2),
			v.dot(&self.3))
	}
}

pub fn identity()->Matrix44 {
	Matrix44::identity()
}

//impl<F:Num+Zero+One> Matrix4<Vec4<F>> {
pub fn projection<F:FloatMath=f32>(tan_half_fov:F, aspect:F, znear:F, zfar:F)->Matrix44<F> {
	let xmax=znear * tan_half_fov;
	let xmin=-xmax;
	let ymin=xmin/aspect;
	let ymax=xmax/aspect;
	let width=xmax-xmin;
	let height=ymax-ymin;

	let depth = zfar-znear;
	let q=-(zfar+znear)/depth;
	let two = one::<F>()+one::<F>();
	let qn=-two*(zfar*znear)/depth;
	let w=two*znear/width;
//	let w= w/aspect;
	let h=two*znear/ height;
	let a=(xmax+xmin)/(xmax-xmin);
	let b=(ymax+ymin)/(ymax-ymin);
	let c=-(zfar+znear)/(zfar-znear);
	let d=-two*(zfar*znear)/(zfar-znear);
	
	Matrix44(
		Vec4(w, zero(), zero(), zero()),
		Vec4(zero(), h, zero(), zero()),
		Vec4(a, b, c, -one::<F>()),
		Vec4(zero(), zero(), d, zero()),
	)
}

pub fn rotate_x<F:FloatMath=f32>(a:F)->Matrix44<F> {
	let (s,c)=a.sin_cos(); let one=one(); let zero=zero();
	Matrix44(
		Vector::from_xyzw(one,	zero,	zero,	zero),
		Vector::from_xyzw(zero,	c,		s,	zero),
		Vector::from_xyzw(zero,	-s,		c,	zero),
		Vector::from_xyzw(zero,	zero,	zero,	one))
}
pub fn rotate_y<F:FloatMath=f32>(a:F)->Matrix44<F> {
	let (s,c)=a.sin_cos(); let one=one(); let zero=zero();
	Matrix44(
		Vec4(c,		zero,	s,	zero),
		Vec4(zero,	one,	zero,	zero),
		Vec4(-s,		zero,	c,	zero),
		Vec4(zero,	zero,	zero,	one))
}
pub fn rotate_z<F:FloatMath=f32>(a:F)->Matrix44<F> {
	let (s,c)=a.sin_cos(); let one=one(); let zero=zero();
	Matrix44(
		Vec4(c,		s,	zero,	zero),
		Vec4(-s,		c,	zero,	zero),
		Vec4(zero,	zero,	one,	zero),
		Vec4(zero,	zero,	zero,	one))
}
pub fn rotate_xyz<F:FloatMath=f32>(r:&Vec3<F>)->Matrix44<F> {
	rotate_x(r.x())*rotate_y(r.y())*rotate_z(r.z())
}
pub fn rotate_xzy<F:FloatMath=f32>(r:&Vec3<F>)->Matrix44<F> {
	rotate_x(r.x())*rotate_z(r.z())*rotate_y(r.y())
}
pub fn rotate_zyx<F:FloatMath=f32>(r:&Vec3<F>)->Matrix44<F> {
	rotate_z(r.z())*rotate_y(r.y())*rotate_x(r.x())
}
pub fn rotate_yzx<F:FloatMath=f32>(r:&Vec3<F>)->Matrix44<F> {
	rotate_y(r.y())*rotate_z(r.z())*rotate_x(r.x())
}

pub fn translate_xyz<F:FloatMath=f32>(x:F,y:F,z:F)->Matrix44<F> {
	let one=one(); let zero=zero();
	Matrix44(
		Vec4(one,	zero,	zero,	zero),
		Vec4(zero,	one,	zero,	zero),
		Vec4(zero,	zero,	one,	zero),
		Vec4(x,	y,	z,	one))
}
pub fn translate<F:FloatMath+One=f32>(trans:&Vec3<F>)->Matrix44<F> {
	let one=one(); let zero=zero();
	Matrix44(
		Vec4::<F>(one,	zero,	zero,	zero),
		Vec4::<F>(zero,	one,	zero,	zero),
		Vec4::<F>(zero,	zero,	one,	zero),
		trans.xyz1())
}

pub fn scale_x<F:FloatMath=f32>(scale:F)->Matrix44<F> {
	let one=one(); let zero=zero();
	Matrix44(
		Vec4(scale,	zero,	zero,	zero),
		Vec4(zero,	one,	zero,	zero),
		Vec4(zero,	zero,	one,	zero),
		Vec4(zero,zero,zero,one))
}
pub fn scale_y<F:FloatMath=f32>(scale:F)->Matrix44<F> {
	let one=one(); let zero=zero();
	Matrix44(
		Vec4(one,	zero,	zero,	zero),
		Vec4(zero,	scale,	zero,	zero),
		Vec4(zero,	zero,	one,	zero),
		Vec4(zero,zero,zero,one))
}
pub fn scale_z<F:FloatMath=f32>(scale:F)->Matrix44<F> {
	let one=one(); let zero=zero();
	Matrix44(
		Vec4(one,	zero,	zero,	zero),
		Vec4(zero,	one,	zero,	zero),
		Vec4(zero,	zero,	scale,	zero),
		Vec4(zero,zero,zero,one))
}

pub fn scale<F:FloatMath+One=f32>(scale:F)->Matrix44<F> {
	let one=one(); let zero=zero();
	Matrix44(
		Vec4(scale,	zero,	zero,	zero),
		Vec4(zero,	scale,	zero,	zero),
		Vec4(zero,	zero,	scale,	zero),
		Vec4(zero,zero,zero,one))
}

pub fn scale_vec<F:FloatMath=f32>(s:&Vec3<F>)->Matrix44<F> {
	let one=one(); let zero=zero();
	Matrix44(
		Vec4(s.0,	zero,	zero,	zero),
		Vec4(zero,	s.1,	zero,	zero),
		Vec4(zero,	zero,	s.2,	zero),
		Vec4(zero,zero,zero,one))
}

pub fn projection_frustum<F:Float=f32>(left:F,right:F, bottom:F, top:F, fov_radians:F, aspect:F, fnear:F, ffar:F)->Matrix44<F> {

    let two=one::<F>()+one::<F>();
    let a=(right+left)/(right-left);
    let b=(top+bottom)/(top-bottom);
    let c=-(ffar+fnear)/(ffar-fnear);
    let d=-(two*ffar*fnear/(ffar-fnear));
	Matrix44(
		Vec4(two*fnear/(right-left), zero(), zero(), zero()),
		Vec4(zero(), two*fnear/(top-bottom), zero(), zero()),
		Vec4(a, b, c, -one::<F>()),
		Vec4(zero(), zero(), d, zero()),
	)
}

pub fn projection_default()->Matrix44<f32> {
	let znear = 1.0f32/16.0f32;
	let zfar = 1024.0f32;
	projection_frustum(-0.5f32,0.5f32,-0.5f32,0.5f32, 90.0f32, 1.0f32, 0.5f32,5.0f32)
}


pub fn scale_rotate_translate<F:FloatMath=f32>(s:&Vec3<F>,r:&Vec3<F>,t:&Vec3<F>)->Matrix44<F> {
	translate(t)*rotate_zyx(r)*scale_vec(s)
}

pub fn from_quaternion<F:Float=f32+Copy>(q:&Vec4<F>)->Matrix44<F>{
	let Vec4(qx,qy,qz,qw)=*q;
	let two=one::<F>()+one::<F>();
	let zero=zero::<F>();
	let one=one::<F>();
	let qx2=qx*qx;
	let qy2=qy*qy;
	let qz2=qz*qz;
	Matrix44(
		Vec4(one - two*qy2 - two*qz2,	two*qx*qy - two*qz*qw,	two*qx*qz + two*qy*qw, zero),
		Vec4(two*qx*qy + two*qz*qw,	one - two*qx2 - two*qz2,	two*qy*qz - two*qx*qw, zero),
		Vec4(two*qx*qz - two*qy*qw,	two*qy*qz + two*qx*qw,	one - two*qx2 - two*qy2, zero),
		Vec4(zero,zero,zero,one))
}

// frame represented as 9 values, as in softimage
pub struct SRT<T> {
	scale:Vec3<T>, rotate:Vec3<T>,translate:Vec3<T>
}

impl<T:FloatMath> SRT<T> {
	fn to_matrix(&self)->Matrix44<T> {
		scale_rotate_translate(&self.scale,&self.rotate,&self.translate)
	}
	fn new()->SRT<T> { SRT{scale:Vec3(one(),one(),one()), rotate:Vec3(zero(),zero(),zero()), translate:Vec3(zero(),zero(),zero())}}
}

// combines vector operations with operations aware of a matrix..
//pub trait PreMulMat4<V,OUT> {
//	fn pre_mul_mat4(&self,mat:&Matrix4<V>)->OUT;
//}

pub trait PreMulMat44<T,OUT> {
	fn pre_mul_mat44(&self,mat:&Matrix44<T>)->OUT;
}
pub trait PreMulMat43<T,OUT> {
	fn pre_mul_mat43(&self,mat:&Matrix43<T>)->OUT;
}
pub trait PreMulMat34<T,OUT> {
	fn pre_mul_mat34(&self,mat:&Matrix34<T>)->OUT;
}
pub trait PreMulMat33<T,OUT> {
	fn pre_mul_mat33(&self,mat:&Matrix33<T>)->OUT;
}

impl<T:Float> PreMulMat43<T,Vec3<T>> for Vec3<T> {
	fn pre_mul_mat43(&self, mat:&Matrix43<T>)->Vec3<T> {mat.mul_vec3_w0(self)}
}
impl<T:Float> PreMulMat44<T,Vec4<T>> for Vec4<T> {
	fn pre_mul_mat44(&self, mat:&Matrix44<T>)->Vec4<T> {mat.mul_vec4(self)}
}
impl<T:Float> PreMulMat44<T,Matrix44<T>> for Matrix44<T> {
	fn pre_mul_mat44(&self, mat:&Matrix44<T>)->Matrix44<T> {mat.mul_matrix(self)}
}
impl<T:Float> PreMulMat33<T,Matrix33<T>> for Matrix33<T> {
	fn pre_mul_mat33(&self, mat:&Matrix33<T>)->Matrix33<T> {mat.mul_matrix(self)}
}
/*
impl<T:Float,V:VecMath<T>> PreMulMat4<V,V> for V {
	fn pre_mul_mat4(&self, mat:&Matrix4<V>)->V {mat.mul_vec4(self)}
}

impl<T:Float,V:VecMath<T>> PreMulMat4<V,Matrix4<V>> for Matrix4<V> {
	fn pre_mul_mat4(&self, mat:&Matrix4<V>)->Matrix4<V> {mat.mul_matrix(self)}
}
*/

impl<T:Float,RHS:PreMulMat44<T,OUT>,OUT> Mul<RHS,OUT> for Matrix44<T> {
	fn mul(&self, other:&RHS)->OUT { other.pre_mul_mat44(self) }
}
impl<T:Float,RHS:PreMulMat43<T,OUT>,OUT> Mul<RHS,OUT> for Matrix43<T> {
	fn mul(&self, other:&RHS)->OUT { other.pre_mul_mat43(self) }
}
impl<T:Float,RHS:PreMulMat34<T,OUT>,OUT> Mul<RHS,OUT> for Matrix34<T> {
	fn mul(&self, other:&RHS)->OUT { other.pre_mul_mat34(self) }
}
impl<T:Float,RHS:PreMulMat33<T,OUT>,OUT> Mul<RHS,OUT> for Matrix33<T> {
	fn mul(&self, other:&RHS)->OUT { other.pre_mul_mat33(self) }
}

impl<T:Float+Copy> Matrix43<T> {
	pub fn from_mat44(src:&Matrix44<T>)->Matrix43<T> {
		Matrix43(src.0 .to_vec3(), src.1 .to_vec3(),src.2 .to_vec3(),src.3 .to_vec3())
	}
}

pub trait LookAt<T> {
	fn look_along(&self,axis:&Self,up:&Self)->Matrix44<T>;
	fn look_at(&self,axis:&Self,up:&Self)->Matrix44<T>;
}

impl<T:Float> LookAt<T> for Vec4<T> {
	fn look_along(&self,at:&Vec4<T>,up:&Vec4<T>)->Matrix44<T>{Matrix44::look_along(self,at,up)}
	fn look_at(&self,at:&Vec4<T>,up:&Vec4<T>)->Matrix44<T>{Matrix44::look_at(self,at,up)}
}

impl<T:Float> LookAt<T> for Vec3<T> {
	fn look_along(&self,at:&Vec3<T>,up:&Vec3<T>)->Matrix44<T>{Matrix44::look_along(&self.to_vec4_pos(),&at.to_vec4(),&up.to_vec4())}
	fn look_at(&self,at:&Vec3<T>,up:&Vec3<T>)->Matrix44<T>{Matrix44::look_at(&self.to_vec4_pos(),&at.to_vec4_pos(),&up.to_vec4())}
}



