pub use r3d::vecmath::*;
pub use std::cmp::*;

#[deriving(Clone,Show)]
pub struct Matrix4<AXISVEC=Vec3<f32>,POSVEC=AXISVEC> {
	ax:AXISVEC,ay:AXISVEC,az:AXISVEC,pos:POSVEC
}

#[deriving(Clone,Show)]
pub struct Matrix3<AXIS=Vec3<f32>> {
	ax:AXIS,ay:AXIS,az:AXIS
}
impl<T,V:VecOps<T>> Matrix3<V> {
	fn new(ax:&V,ay:&V,az:&V)->Matrix3<V> {
		Matrix3{ax:ax.clone(),ay:ay.clone(),az:az.clone()}
	}
}

impl<T,V:VecOps<T>> Matrix4<V> {
	fn new(ax:&V,ay:&V,az:&V,pos:&V)->Matrix4<V> {
		Matrix4{ax:ax.clone(),ay:ay.clone(),az:az.clone(),pos:pos.clone()}
	}
}

// todo: Write all matrix maths in terms of this interface
// various concrete datastructures could expose a matrix

pub struct Scaling<T>{
	sx:T,
	sy:T,
	sz:T
}


impl<T> Scaling<T>
{
	pub fn new(x:T,y:T,z:T)->Scaling<T> {Scaling{sx:x,sy:y,sz:z} }
}

struct RotateX<T>(T);
struct RotateY<T>(T);
struct RotateZ<T>(T);

pub trait IMatrix3<V> {
	fn axisX(&self)->V;
	fn axisY(&self)->V;
	fn axisZ(&self)->V;
	fn matrix3(&self)->Matrix3<V>;
}

pub trait IMatrix4<V> {
	fn axisW(&self)->V;
	fn pos(&self)->V;
}
/*
impl<T:Float> IMatrix3<Vec3<T>> for RotateX<T> {
	fn matrix3(&self)->Matrix3<Vec3<T>> {
		let angle=*self;
		let (s,c)=sin_cos(&angle);
		Matrix3::new(
			&Vec3::new(one::<T>(),zero::<T>(),zero::<T>()),
			&Vec3::new(zero::<T>(),c.clone(),s.clone()),
			&Vec3::new(zero::<T>(),-s.clone(),c.clone()))
	}
	fn axisX(&self)->Vec3<T> {let m =self.matrix3(); m.ax}
	fn axisY(&self)->Vec3<T> {let m =self.matrix3(); m.ay}
	fn axisZ(&self)->Vec3<T> {let m =self.matrix3(); m.az}
}
impl<T:Float+Clone> IMatrix3<Vec3<T>> for RotateY<T> {
	fn matrix3(&self)->Matrix3<Vec3<T>> {
		let angle=*self;
		let (s,c)=sin_cos(&angle);
		Matrix3::new(
			&Vec3::new(c.clone(),zero::<T>(),s.clone()),
			&Vec3::new(zero::<T>(),one::<T>(),zero::<T>()),
			&Vec3::new(-s.clone(),zero::<T>(),c.clone()))
	}
	fn axisX(&self)->Vec3<T> {let m =self.matrix3(); m.ax}
	fn axisY(&self)->Vec3<T> {let m =self.matrix3(); m.ay}
	fn axisZ(&self)->Vec3<T> {let m =self.matrix3(); m.az}
}
impl<T:Float+Clone> IMatrix3<Vec3<T>> for RotateZ<T> {
	fn matrix3(&self)->Matrix3<Vec3<T>> {
		let angle=self;
		let (s,c)=angle.sin_cos();
		Matrix3::new(
			&Vec3::new(c.clone(),s.clone(),zero::<T>()),
			&Vec3::new(-s.clone(),c.clone(),zero::<T>()),
			&Vec3::new(zero::<T>(),zero::<T>(),one::<T>()))
	}
	fn axisX(&self)->Vec3<T> {let m =self.matrix3(); m.ax}
	fn axisY(&self)->Vec3<T> {let m =self.matrix3(); m.ay}
	fn axisZ(&self)->Vec3<T> {let m =self.matrix3(); m.az}
}
*/
impl<T:Float+Clone+MyOrd> IMatrix3<Vec3<T>> for Scaling<T> {
	fn axisX(&self)->Vec3<T>	{VecNumOps::vfromXYZ(self.sx.clone(),zero::<T>(),zero::<T>())}
	fn axisY(&self)->Vec3<T>	{VecNumOps::vfromXYZ(zero::<T>(),self.sy.clone(),zero::<T>())}
	fn axisZ(&self)->Vec3<T>	{VecNumOps::vfromXYZ(zero::<T>(),zero::<T>(),self.sz.clone())}
	fn matrix3(&self)->Matrix3<Vec3<T>>{Matrix3::new(&self.axisX(),&self.axisY(),&self.axisZ())}
}
impl<T:Float+Clone+MyOrd> IMatrix4<Vec3<T>> for Scaling<T> {
	fn pos(&self)->Vec3<T>	{VecNumOps::vfromXYZ(zero::<T>(),zero::<T>(),zero::<T>())}
	fn axisW(&self)->Vec3<T>	{self.pos()}
}

impl<T,V:VecOps<T>> IMatrix3<V> for Matrix3<V> {
	fn axisX(&self)->V{self.ax.clone()}
	fn axisY(&self)->V{self.ay.clone()}
	fn axisZ(&self)->V{self.az.clone()}
	fn matrix3(&self)->Matrix3<V>{Matrix3::new(&self.axisX(),&self.axisY(),&self.axisZ())}
}

// Matrix axis accessors
impl<T,V:VecOps<T>> IMatrix4<V> for Matrix4<V> {
	fn axisW(&self)->V{self.pos.clone()}
	fn pos(&self)->V{self.pos.clone()}
}

impl<T:Float,V:VecOps<T>> Matrix4<V> {
}
impl<V:VecOps<T>,T:Float+MyOrd=f32> Matrix4<V> {
	pub fn identity()->Matrix4<V>{
		Matrix4::new(
			&VecConsts::vaxis(0),
			&VecConsts::vaxis(1),
			&VecConsts::vaxis(2),
			&VecConsts::vaxis(3))
	}
	pub fn translate(trans:&V)->Matrix4<V>{
		Matrix4::new(
			&VecConsts::vaxis(0),
			&VecConsts::vaxis(1),
			&VecConsts::vaxis(2),
			&trans.clone())
	}
	
	pub fn lookAlong(pos:&V,fwd:&V,up:&V)->Matrix4<V>{
		let az=fwd.vnormalize();
		let ax=az.vcross(up).vnormalize();
		let ay=az.vcross(&ax);
		Matrix4::new(&ax,&ay,&az,pos)
	}
	pub fn look_at(pos:&V,target:&V,up:&V)->Matrix4<V> { Matrix4::lookAlong(pos,&target.vsub(pos),up) }
	pub fn orthonormalize_zyx(self)->Matrix4<V> {
		Matrix4::lookAlong(&self.pos,&self.az,&self.ay)
	}
	pub fn mul_point(&self,pt:&V)->V{
		self.pos.vmad(&self.ax,pt.vx()).vmad(&self.ay,pt.vy()).vmad(&self.az,pt.vz())
	}
	pub fn inv_mul_point(&self,pt:&V)->V{
		let ofs=pt.vsub(&self.pos);
		VecNumOps::vfromXYZ(ofs.vdot(&self.ax),ofs.vdot(&self.ay),ofs.vdot(&self.az))
	}
	pub fn mul_vec3(&self,pt:&V)->V{
		self.ax.vscale(pt.vx()).vmad(&self.ay,pt.vy()).vmad(&self.az,pt.vz())
	}
	pub fn mul_vec4(&self,pt:&V)->V{
		self.ax.vscale(pt.vx()).vmad(&self.ay,pt.vy()).vmad(&self.az,pt.vz()).vmad(&self.pos,pt.vw())
	}
	pub fn mul_matrix(&self,other:&Matrix4<V>)->Matrix4<V> {
		Matrix4::new(
			&self.mul_vec4(&other.ax),
			&self.mul_vec4(&other.ay),
			&self.mul_vec4(&other.az),
			&self.mul_vec4(&other.pos))
	}
}

//impl<F:Num+Zero+One> Matrix4<Vec4<F>> {
pub fn projection<F:Float+MyOrd=f32>(fov_radians:F, aspect:F, znear:F, zfar:F)->Matrix4<Vec4<F>> {
	let xymax=znear * num::tan(fov_radians );
	let ymin=-xymax;
	let xmin=-xymax;
	let width=xymax-xmin;
	let height=xymax-ymin;

	let depth = zfar-znear;
	let q=-(zfar+znear)/depth;
	let two = one::<F>()+one::<F>();
	let qn=-two*(zfar*znear)/depth;
	let w=two*znear/width;
	let w= w/aspect;
	let h=two*znear/ height;
	
	Matrix4::new(
		&Vec4::<F>::new(w, zero::<F>(), zero::<F>(), zero::<F>()),
		&Vec4::<F>::new(zero::<F>(), h, zero::<F>(), zero::<F>()),
		&Vec4::<F>::new(zero::<F>(), zero::<F>(), q, -one::<F>()),
		&Vec4::<F>::new(zero::<F>(), zero::<F>(), qn, zero::<F>()),
	)
}

pub fn rotate_x<F:Float+MyOrd=f32>(a:F)->Matrix4<Vec4<F>> {
	let (s,c)=num::sin_cos(a); let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(one,	zero,	zero,	zero),
		&Vec4::<F>::new(zero,	c,		s,	zero),
		&Vec4::<F>::new(zero,	-s,		c,	zero),
		&Vec4::<F>::new(zero,	zero,	zero,	one))
}
pub fn rotate_y<F:Float+MyOrd=f32>(a:F)->Matrix4<Vec4<F>> {
	let (s,c)=num::sin_cos(a); let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(c,		zero,	s,	zero),
		&Vec4::<F>::new(zero,	one,	zero,	zero),
		&Vec4::<F>::new(-s,		zero,	c,	zero),
		&Vec4::<F>::new(zero,	zero,	zero,	one))
}
pub fn rotate_z<F:Float+MyOrd=f32>(a:F)->Matrix4<Vec4<F>> {
	let (s,c)=num::sin_cos(a); let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(c,		s,	zero,	zero),
		&Vec4::<F>::new(-s,		c,	zero,	zero),
		&Vec4::<F>::new(zero,	zero,	one,	zero),
		&Vec4::<F>::new(zero,	zero,	zero,	one))
}
pub fn translate_xyz<F:Float+MyOrd=f32>(x:F,y:F,z:F)->Matrix4<Vec4<F>> {
	let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(one,	zero,	zero,	zero),
		&Vec4::<F>::new(zero,	one,	zero,	zero),
		&Vec4::<F>::new(zero,	zero,	one,	zero),
		&Vec4::<F>::new(x,	y,	z,	one))
}
pub fn translate_vec4<F:Float+MyOrd=f32>(trans:&Vec4<F>)->Matrix4<Vec4<F>> {
	let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(one,	zero,	zero,	zero),
		&Vec4::<F>::new(zero,	one,	zero,	zero),
		&Vec4::<F>::new(zero,	zero,	one,	zero),
		trans)
}
pub fn translate<F:Float+MyOrd=f32>(trans:&Vec3<F>)->Matrix4<Vec4<F>> {
	let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(one,	zero,	zero,	zero),
		&Vec4::<F>::new(zero,	one,	zero,	zero),
		&Vec4::<F>::new(zero,	zero,	one,	zero),
		&Vec4::<F>::new(trans.x,trans.y,trans.z, one))
}



pub fn projection_frustum<F:Float+MyOrd=f32>(left:F,right:F, bottom:F, top:F, fov_radians:F, aspect:F, fnear:F, ffar:F)->Matrix4<Vec4<F>> {

    let two=one::<F>()+one::<F>();
    let a=(right+left)/(right-left);
    let b=(top+bottom)/(top-bottom);
    let c=-(ffar+fnear)/(ffar-fnear);
    let d=-(two*ffar*fnear/(ffar-fnear));
   /*
#define STORE4(D,I, X,Y,Z,W) (D)[0+I]=X; (D)[4+I]=Y; (D)[8+I]=Z; (D)[12+I]=W;
    STORE4(m,0, 2.f*fnear/(right-left), 0.f,0.f,0.f);
    STORE4(m,1, 0.f, 2.f*fnear/(top-bottom), 0.f,0.f);
    STORE4(m,2, a,b,c,-1.f);
    STORE4(m,3, 0.f,0.f,d,0.f);
#undef STORE4
*/
	Matrix4::new(
		&Vec4::<F>::new(two*fnear/(right-left), zero::<F>(), zero::<F>(), zero::<F>()),
		&Vec4::<F>::new(zero::<F>(), two*fnear/(top-bottom), zero::<F>(), zero::<F>()),
		&Vec4::<F>::new(a, b, c, -one::<F>()),
		&Vec4::<F>::new(zero::<F>(), zero::<F>(), d, zero::<F>()),
	)

}



pub type Matrix33<T=f32> =Matrix3<Vec3<T>>;
pub type Matrix43<T=f32> =Matrix4<Vec3<T>>;
pub type Matrix44<T=f32> =Matrix4<Vec4<T>>;
pub struct Matrix33f(Matrix3<Vec3<f32>>);
pub struct Matrix43f(Matrix4<Vec3<f32>>);
pub struct Matrix44f(Matrix4<Vec4<f32>>);

