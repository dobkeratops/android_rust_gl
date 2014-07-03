pub use r3d::vecmath::*;
pub use std::cmp::*;


#[deriving(Clone,Show)]
pub struct Matrix4<AXISVEC=Vec3<f32>,POSVEC=AXISVEC> {
	pub ax:AXISVEC,pub ay:AXISVEC,pub az:AXISVEC,pub pos:POSVEC
}

#[deriving(Clone,Show)]
pub struct Matrix3<AXIS=Vec3<f32>> {
	pub ax:AXIS,pub ay:AXIS,pub az:AXIS
}
impl<T,V:VecMath<T>> Matrix3<V> {
	fn new(ax:&V,ay:&V,az:&V)->Matrix3<V> {
		Matrix3{ax:ax.clone(),ay:ay.clone(),az:az.clone()}
	}
}

impl<T,V:VecMath<T>> Matrix4<V> {
	fn new(ax:&V,ay:&V,az:&V,pos:&V)->Matrix4<V> {
		Matrix4{ax:ax.clone(),ay:ay.clone(),az:az.clone(),pos:pos.clone()}
	}
}

// todo: Write all matrix maths in terms of this interface
// various concrete datastructures could expose a matrix

pub struct Scaling<T>{
	pub sx:T,
	pub sy:T,
	pub sz:T
}


impl<T> Scaling<T>
{
	pub fn new(x:T,y:T,z:T)->Scaling<T> {Scaling{sx:x,sy:y,sz:z} }
}

struct RotateX<T>(T);
struct RotateY<T>(T);
struct RotateZ<T>(T);

pub trait IMatrix3<V> {
	fn axis_x(&self)->V;
	fn axis_y(&self)->V;
	fn axis_z(&self)->V;
	fn matrix3(&self)->Matrix3<V>;
}

pub trait IMatrix4<V> {
	fn axis_w(&self)->V;
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
impl<T:Float> IMatrix3<Vec3<T>> for Scaling<T> {
	fn axis_x(&self)->Vec3<T>	{VecNum::from_xyz(self.sx.clone(),zero::<T>(),zero::<T>())}
	fn axis_y(&self)->Vec3<T>	{VecNum::from_xyz(zero::<T>(),self.sy.clone(),zero::<T>())}
	fn axis_z(&self)->Vec3<T>	{VecNum::from_xyz(zero::<T>(),zero::<T>(),self.sz.clone())}
	fn matrix3(&self)->Matrix3<Vec3<T>>{Matrix3::new(&self.axis_x(),&self.axis_y(),&self.axis_z())}
}
impl<T:Float> IMatrix4<Vec3<T>> for Scaling<T> {
	fn pos(&self)->Vec3<T>	{VecNum::from_xyz(zero::<T>(),zero::<T>(),zero::<T>())}
	fn axis_w(&self)->Vec3<T>	{self.pos()}
}

impl<T,V:VecMath<T>> IMatrix3<V> for Matrix3<V> {
	fn axis_x(&self)->V{self.ax.clone()}
	fn axis_y(&self)->V{self.ay.clone()}
	fn axis_z(&self)->V{self.az.clone()}
	fn matrix3(&self)->Matrix3<V>{Matrix3::new(&self.axis_x(),&self.axis_y(),&self.axis_z())}
}

// Matrix axis accessors
impl<T,V:VecMath<T>> IMatrix4<V> for Matrix4<V> {
	fn axis_w(&self)->V{self.pos.clone()}
	fn pos(&self)->V{self.pos.clone()}
}

impl<T:Float,V:VecMath<T>> Matrix4<V> {
}
impl<V:VecMath<T>,T:Float=f32> Matrix4<V> {
	pub fn identity()->Matrix4<V>{
		Matrix4::new(
			&VecConsts::axis(0),
			&VecConsts::axis(1),
			&VecConsts::axis(2),
			&VecConsts::axis(3))
	}
	pub fn translate(trans:&V)->Matrix4<V>{
		Matrix4::new(
			&VecConsts::axis(0),
			&VecConsts::axis(1),
			&VecConsts::axis(2),
			&trans.clone())
	}
	
	pub fn look_along(pos:&V,fwd:&V,up:&V)->Matrix4<V>{
		let az=fwd.normalize();
		let ax=az.cross(up).normalize();
		let ay=az.cross(&ax);
		Matrix4::new(&ax,&ay,&az,pos)
	}
	pub fn look_at(pos:&V,target:&V,up:&V)->Matrix4<V> { Matrix4::look_along(pos,&target.sub(pos),up) }
	pub fn orthonormalize_zyx(self)->Matrix4<V> {
		Matrix4::look_along(&self.pos,&self.az,&self.ay)
	}
	pub fn mul_point(&self,pt:&V)->V{
		self.pos.mad(&self.ax,pt.x()).mad(&self.ay,pt.y()).mad(&self.az,pt.z())
	}
	pub fn inv_mul_point(&self,pt:&V)->V{
		let ofs=pt.sub(&self.pos);
		VecNum::from_xyz(ofs.dot(&self.ax),ofs.dot(&self.ay),ofs.dot(&self.az))
	}
	pub fn mul_vec3(&self,pt:&V)->V{
		self.ax.scale(pt.x()).mad(&self.ay,pt.y()).mad(&self.az,pt.z())
	}
	pub fn mul_vec4(&self,pt:&V)->V{
		self.ax.scale(pt.x()).mad(&self.ay,pt.y()).mad(&self.az,pt.z()).mad(&self.pos,pt.w())
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
pub fn projection<F:FloatMath=f32>(fov_radians:F, aspect:F, znear:F, zfar:F)->Matrix4<Vec4<F>> {
	let xymax=znear * fov_radians.tan();
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

pub fn rotate_x<F:FloatMath=f32>(a:F)->Matrix4<Vec4<F>> {
	let (s,c)=a.sin_cos(); let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(one,	zero,	zero,	zero),
		&Vec4::<F>::new(zero,	c,		s,	zero),
		&Vec4::<F>::new(zero,	-s,		c,	zero),
		&Vec4::<F>::new(zero,	zero,	zero,	one))
}
pub fn rotate_y<F:FloatMath=f32>(a:F)->Matrix4<Vec4<F>> {
	let (s,c)=a.sin_cos(); let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(c,		zero,	s,	zero),
		&Vec4::<F>::new(zero,	one,	zero,	zero),
		&Vec4::<F>::new(-s,		zero,	c,	zero),
		&Vec4::<F>::new(zero,	zero,	zero,	one))
}
pub fn rotate_z<F:FloatMath=f32>(a:F)->Matrix4<Vec4<F>> {
	let (s,c)=a.sin_cos(); let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(c,		s,	zero,	zero),
		&Vec4::<F>::new(-s,		c,	zero,	zero),
		&Vec4::<F>::new(zero,	zero,	one,	zero),
		&Vec4::<F>::new(zero,	zero,	zero,	one))
}
pub fn translate_xyz<F:FloatMath=f32>(x:F,y:F,z:F)->Matrix4<Vec4<F>> {
	let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(one,	zero,	zero,	zero),
		&Vec4::<F>::new(zero,	one,	zero,	zero),
		&Vec4::<F>::new(zero,	zero,	one,	zero),
		&Vec4::<F>::new(x,	y,	z,	one))
}
pub fn translate_vec4<F:FloatMath=f32>(trans:&Vec4<F>)->Matrix4<Vec4<F>> {
	let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(one,	zero,	zero,	zero),
		&Vec4::<F>::new(zero,	one,	zero,	zero),
		&Vec4::<F>::new(zero,	zero,	one,	zero),
		trans)
}
pub fn translate<F:Float=f32>(trans:&Vec3<F>)->Matrix4<Vec4<F>> {
	let one=one::<F>(); let zero=zero::<F>();
	Matrix4::new(
		&Vec4::<F>::new(one,	zero,	zero,	zero),
		&Vec4::<F>::new(zero,	one,	zero,	zero),
		&Vec4::<F>::new(zero,	zero,	one,	zero),
		&Vec4::<F>::new(trans.x,trans.y,trans.z, one))
}

pub fn projection_frustum<F:Float=f32>(left:F,right:F, bottom:F, top:F, fov_radians:F, aspect:F, fnear:F, ffar:F)->Matrix4<Vec4<F>> {

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

// combines vector operations with operations aware of a matrix..
pub trait PreMulMat44<T> {
	fn pre_mul_mat44(&self,mat:&Matrix4<Vec4<T>>)->Self;
}
pub trait PreMulMat43<T> {
	fn pre_mul_mat43(&self,mat:&Matrix4<Vec3<T>>)->Self;
}
pub trait PreMulMat33<T> {
	fn pre_mul_mat33(&self,mat:&Matrix3<Vec3<T>>)->Self;
}

impl<T:Float> PreMulMat43<T> for Vec3<T> {
	fn pre_mul_mat43(&self, mat:&Matrix4<Vec3<T>>)->Vec3<T> {mat.mul_vec3(self)}
}
impl<T:Float> PreMulMat44<T> for Vec4<T> {
	fn pre_mul_mat44(&self, mat:&Matrix4<Vec4<T>>)->Vec4<T> {mat.mul_vec4(self)}
}

/*
impl<T:MyFloat,RHS,OUT> Mul<RHS,OUT> for Matrix4<Vec4<T>> {
	fn mul(&self, other:&RHS)->OUT { other.vpre_mul_mat44(self) }
}
impl<T:MyFloat,RHS,OUT> Mul<RHS,OUT> for Matrix4<Vec3<T>> {
	fn mul(&self, other:&RHS)->OUT { other.vpre_mul_mat43(self) }
}
impl<T:MyFloat,RHS,OUT> Mul<RHS,OUT> for Matrix3<Vec4<T>> {
	fn mul(&self, other:&RHS)->OUT { other.vpre_mul_mat34(self) }
}
impl<T:MyFloat,RHS,OUT> Mul<RHS,OUT> for Matrix3<Vec3<T>> {
	fn mul(&self, other:&RHS)->OUT { other.vpre_mul_mat33(self) }
}
*/



pub type Matrix33<T=f32> =Matrix3<Vec3<T>>;
pub type Matrix43<T=f32> =Matrix4<Vec3<T>>;
pub type Matrix44<T=f32> =Matrix4<Vec4<T>>;
pub struct Matrix33f(Matrix3<Vec3<f32>>);
pub struct Matrix43f(Matrix4<Vec3<f32>>);
pub struct Matrix44f(Matrix4<Vec4<f32>>);



