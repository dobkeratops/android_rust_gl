use super::vectypes::*;
use super::vecmath::*;
pub use std::cmp::*;

#[deriving(Copy,Show)]
pub struct Matrix2<AXIS=Vec2<f32>> (pub AXIS,pub AXIS);

#[deriving(Copy,Show)]
pub struct Matrix3<AXIS=Vec3<f32>> (pub AXIS,pub AXIS,pub AXIS);

#[deriving(Copy,Show)]
pub struct Matrix4<AXISVEC=Vec4<f32>,POSVEC=AXISVEC> (pub AXISVEC,pub AXISVEC,pub AXISVEC,pub POSVEC);

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
	pub fn from_xyz<V:XYZW<T>>(v:&V)->Scaling<T> {Scaling{sx:v.x(),sy:v.y(),sz:v.z()}}
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
	pub fn ax<'a>(&'a self)->&'a V { let Matrix2(ref r,_)=*self; r}
	pub fn ay<'a>(&'a self)->&'a V { let Matrix2(_,ref r)=*self; r}
}

impl<'a, V> GetAxes<V> for  Matrix3<V> {
	fn ax<'a>(&'a self)->&'a V { let Matrix3(ref r,_,_)=*self; r}
	fn ay<'a>(&'a self)->&'a V { let Matrix3(_,ref r,_)=*self; r}
	fn az<'a>(&'a self)->&'a V { let Matrix3(_,_,ref r)=*self; r}
}

// Accessor for axes, by value: may be efficeint to construct axes simultaneously.
// individual acessors may be overriden if its' convenient
pub trait ToAxes<V> {
	fn axis_x(&self)->V {let (x,_,_)=self.axes(); x}
	fn axis_y(&self)->V {let (_,y,_)=self.axes(); y}
	fn axis_z(&self)->V {let (_,_,z)=self.axes(); z}
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

impl<V:Copy> ToAxes<V> for Matrix4<V> {
	fn axes(&self)->(V,V,V) {
		(*self.ax(),*self.ay(),*self.az())
	}
}
impl<T:Float+Copy,V:ToVec3<T>> SetAxes<V> for Matrix4<Vec3<T>> {
	fn set_ax(&mut self, a:V){
		let Matrix4(ref mut ax,_,_,_)=*self;
		*ax=a.to_vec3();
	}
	fn set_ay(&mut self, a:V){
		let Matrix4(_,ref mut ay,_,_)=*self;
		*ay=a.to_vec3();
	}
	fn set_az(&mut self, a:V){
		let Matrix4(_,_,ref mut az,_)=*self;
		*az=a.to_vec3();
	}
}
impl<T:Float+Copy,V:ToVec4<T>> SetAxes<V> for Matrix4<Vec4<T>> {
	fn set_ax(&mut self, a:V){
		let Matrix4(ref mut ax,ref ay,ref az,ref aw)=*self;
		*ax=a.to_vec4();
	}
	fn set_ay(&mut self, a:V){
		let Matrix4(ref ax,ref mut ay,ref az,ref aw)=*self;
		*ay=a.to_vec4();
	}
	fn set_az(&mut self, a:V){
		let Matrix4(ref ax,ref ay,ref mut az,ref aw)=*self;
		*az=a.to_vec4();
	}
}
impl<T:Float+Copy,V:ToVec3<T>> SetAxes<V> for Matrix3<Vec3<T>> {
	fn set_ax(&mut self, a:V){
		let Matrix3(ref mut ax,_,_)=*self;
		*ax=a.to_vec3();
	}
	fn set_ay(&mut self, a:V){
		let Matrix3(_,ref mut ay,_)=*self;
		*ay=a.to_vec3();
	}
	fn set_az(&mut self, a:V){
		let Matrix3(_,_,ref mut az)=*self;
		*az=a.to_vec3();
	}
}
impl<T:Float+Copy,V:ToVec4<T>> SetAxes<V> for Matrix3<Vec4<T>> {
	fn set_ax(&mut self, a:V){
		let Matrix3(ref mut ax,_,_)=*self;
		*ax=a.to_vec4();
	}
	fn set_ay(&mut self, a:V){
		let Matrix3(_,ref mut ay,_)=*self;
		*ay=a.to_vec4();
	}
	fn set_az(&mut self, a:V){
		let Matrix3(_,_,ref mut az)=*self;
		*az=a.to_vec4();
	}
}

impl<V> GetAxes<V> for Matrix4<V> {
	fn ax<'a>(&'a self)->&'a V { let Matrix4(ref r,_,_,_)=*self; r}
	fn ay<'a>(&'a self)->&'a V { let Matrix4( _,ref r,_,_)=*self; r}
	fn az<'a>(&'a self)->&'a V { let Matrix4( _,_,ref r,_)=*self; r}
}

impl<V> Matrix4<V> {
	pub fn aw<'a>(&'a self)->&'a V { let Matrix4( _,_,_,ref r)=*self; r}
}
impl<T:Float+Copy,V:XYZW<T>> Pos<V> for Matrix4<V> {
	fn pos(&self)->V { let Matrix4( _,_,_,r)=*self; r}
	fn set_pos(&mut self ,v:&V) { let Matrix4( _,_,_,ref mut r)=*self; *r=v.to_point();}
}

impl<T:Zero+One+Copy> Matrix4<Vec4<T>> {
	pub fn from_vec4(ax:Vec4<T>,ay:Vec4<T>,az:Vec4<T>,aw:Vec4<T>)->Matrix4<Vec4<T>> {
		Matrix4(ax,ay,az,aw)
	}
	pub fn from_vec3(ax:Vec3<T>,ay:Vec3<T>,az:Vec3<T>,aw:Vec3<T>)->Matrix4<Vec4<T>> {
		Matrix4(ax.to_vec4(),ay.to_vec4(),az.to_vec4(),(aw,one()).to_vec4())
	}

	pub fn from_mat33(mat33:&Matrix3<Vec3<T>>)->Matrix4<Vec4<T>> {
		Matrix4::from_mat33_pos(mat33,&Vec4(zero(),zero(),zero(),one()))
	}
	pub fn from_mat33_pos(mat33:&Matrix3<Vec3<T>>,pos:&Vec4<T>)->Matrix4<Vec4<T>> {
		Matrix4(
			Vec4::from_vec3(mat33.ax(),zero()),
			Vec4::from_vec3(mat33.ay(),zero()),
			Vec4::from_vec3(mat33.az(),zero()),
			*pos)
	}
	pub fn from_mat43(mat43:&Matrix4<Vec3<T>>)->Matrix4<Vec4<T>> {
		Matrix4(
			Vec4::from_vec3(mat43.ax(),zero()),
			Vec4::from_vec3(mat43.ay(),zero()),
			Vec4::from_vec3(mat43.az(),zero()),
			Vec4::from_vec3(mat43.aw(),one()))
	}
	pub fn from_mat34_pos(mat33:&Matrix3<Vec4<T>>,pos:&Vec4<T>)->Matrix4<Vec4<T>> {
		Matrix4(*mat33.ax(),*mat33.ay(),*mat33.az(),*pos)
	}
	pub fn mat33(&self)->Matrix3<Vec3<T>> {
		Matrix3(self.ax().xyz(),self.ay().xyz(),self.az().xyz())
	}
	pub fn mat43(&self)->Matrix4<Vec3<T>> {
		Matrix4(self.ax().xyz(),self.ay().xyz(),self.az().xyz(),self.aw().xyz())
	}
	pub fn mat34(&self)->Matrix3<Vec4<T>> {
		Matrix3(*self.ax(),*self.ay(),*self.az())
	}
}

impl<T:Float,V:VecMath<T>> Matrix4<V> {
}

// Indirection Traits via PreMulMatXX to get Matrix*Matrix and Matrix*Vector
impl<T:Float, OUT, RHS:PreMulMat44<T,OUT> > Mul<RHS,OUT> for Matrix4<Vec4<T>> {
	fn mul(&self, rhs:&RHS)->OUT { rhs.pre_mul_mat44(self) } 
}
impl<T:Float, OUT, RHS:PreMulMat33<T,OUT> > Mul<RHS,OUT> for Matrix3<Vec3<T>> {
	fn mul(&self, rhs:&RHS)->OUT { rhs.pre_mul_mat33(self) } 
}
//impl<T:Float, V:VecMath<T>,OUT, RHS:PreMulMat4<V,OUT> > Mul<RHS,OUT> for Matrix4<V> {
//	fn mul(&self, rhs:&RHS)->OUT { rhs.pre_mul_mat4(self) } 
//}

impl<T:Float> Transpose<Matrix4<Vec4<T>,Vec4<T>>> for Matrix4<Vec4<T>> {
	fn transpose(&self)->Matrix4<Vec4<T>> {
		// todo-SIMD 2x2 shuffles
		Matrix4(
			Vec4(self.ax().x(), self.ay().x(), self.az().x(), self.aw().x()),
			Vec4(self.ax().y(), self.ay().y(), self.az().y(), self.aw().y()),
			Vec4(self.ax().z(), self.ay().z(), self.az().z(), self.aw().z()),
			Vec4(self.ax().w(), self.ay().w(), self.az().w(), self.aw().w())
		)
	}
}
impl<T:Copy+Float> Transpose<Matrix3<Vec4<T>>> for Matrix4<Vec3<T>> {
	fn transpose(&self)->Matrix3<Vec4<T>> {
		// todo-SIMD..
		Matrix3(
			Vec4(self.ax().x(), self.ay().x(), self.az().x(), self.aw().x()),
			Vec4(self.ax().y(), self.ay().y(), self.az().y(), self.aw().y()),
			Vec4(self.ax().z(), self.ay().z(), self.az().z(), self.aw().z()),
		)
	}
}
impl<T:Float> Transpose<Matrix4<Vec3<T>,Vec3<T>>> for Matrix3<Vec4<T>> {
	fn transpose(&self)->Matrix4<Vec3<T>> {
		// todo-SIMD..
		Matrix4(
			Vec3(self.ax().x(), self.ay().x(), self.az().x()),
			Vec3(self.ax().y(), self.ay().y(), self.az().y()),
			Vec3(self.ax().z(), self.ay().z(), self.az().z()),
			Vec3(self.ax().w(), self.ay().w(), self.az().w())
		)
	}
}

impl<T:Float> Transpose<Matrix3<Vec3<T>>> for Matrix3<Vec3<T>> {
	fn transpose(&self)->Matrix3<Vec3<T>> {
		// todo-SIMD..
		Matrix3(
			Vec3(self.ax().x(), self.ay().x(), self.az().x()),
			Vec3(self.ax().y(), self.ay().y(), self.az().y()),
			Vec3(self.ax().z(), self.ay().z(), self.az().z()),
		)
	}
}

impl<V:VecMath<T>+ToVec3<T>+ToVec4<T>,T:Float=f32> Matrix4<V> {
	pub fn identity()->Matrix4<V>{
		Matrix4(
			VecMath::axis(0),
			VecMath::axis(1),
			VecMath::axis(2),
			VecMath::axis(3))
	}
	pub fn translate(trans:&V)->Matrix4<V>{
		Matrix4(
			VecMath::axis(0),
			VecMath::axis(1),
			VecMath::axis(2),
			*trans)
	}	
	pub fn look_along(pos:&V,fwd:&V,up:&V)->Matrix4<V>{
		let az=fwd.normalize();
		let ax=up.cross(&az).normalize();
		let ay=az.cross(&ax);
		Matrix4(ax,ay,az,pos.to_point())
	}
	// Variation of 'look-at' calculation that prioritizes upvector
	pub fn look_up_along(pos:&V,up:&V,fwd:&V)->Matrix4<V>{
		let ay=up.normalize();
		let ax=fwd.cross(up).normalize();
		let az=ax.cross(&ay);
		Matrix4(ax,ay,az,pos.to_point())
	}

	pub fn look_at(pos:&V,target:&V,up:&V)->Matrix4<V> { Matrix4::look_along(pos,&target.sub(pos),up) }
	pub fn orthonormalize_zyx(&self)->Matrix4<V> {
		Matrix4::look_along(self.aw(),self.az(),self.ay())
	}
	pub fn orthonormalize_yzx(&self)->Matrix4<V> {
		Matrix4::look_up_along(self.aw(),self.ay(),self.az())
	}
	pub fn mul_vec(&self,pt:&V)->V{	// 'vec'=x,y,z,w
		self.ax().scale(pt.x()).macc(self.ay(),pt.y()).macc(self.az(),pt.z()).macc(self.aw(),pt.w())
	}
	pub fn mul_point(&self,pt:&V)->V{	// 'point'=x,y,z,1
		self.aw().macc(self.ax(),pt.x()).macc(self.ay(),pt.y()).macc(self.az(),pt.z())
	}
	pub fn mul_axis(&self,pt:&V)->V{	// 'axis'=x,y,z,0
		self.ax().scale(pt.x()).macc(self.ay(),pt.y()).macc(self.az(),pt.z())
	}
	pub fn inv_mul_point(&self,pt:&V)->V{
		let ofs=pt.sub(self.aw());
		XYZW::from_xyz(ofs.dot(self.ax()),ofs.dot(self.ay()),ofs.dot(self.az()))
	}
	pub fn inv_mul_axis(&self,axis:&V)->V{
		XYZW::from_xyz(axis.dot(self.ax()),axis.dot(self.ay()),axis.dot(self.az()))
	}
	pub fn mul_vec3_w0(&self,pt:&Vec3<T>)->Vec3<T>{
		let Vec3(x,y,z)=*pt;
		self.ax().scale(x).macc(self.ay(),y).macc(self.az(),z).to_vec3()
	}
	pub fn mul_vec3_w1(&self,pt:&Vec3<T>)->Vec3<T>{
		let Vec3(x,y,z)=*pt;
		self.aw().macc(self.ax(),x).macc(self.ay(),y).macc(self.az(),z).to_vec3()
	}
	pub fn mul_vec4(&self,pt:&Vec4<T>)->Vec4<T>{
		let Vec4(x,y,z,w)=*pt;		
		self.ax().scale(x).macc(self.ay(),y).macc(self.az(),z).macc(self.aw(),w).to_vec4()
	}
	pub fn mul_matrix(&self,other:&Matrix4<V>)->Matrix4<V> {
		Matrix4(
			self.mul_vec(other.ax()),
			self.mul_vec(other.ay()),
			self.mul_vec(other.az()),
			self.mul_vec(other.aw()))
	}
	// matrix inverse with assumption of being orthonormalized.
	pub fn inv_orthonormal(&self)->Matrix4<V> {
		let ax=XYZW::from_xyz(self.ax().x(),self.ay().x(),self.az().x(),);
		let ay=XYZW::from_xyz(self.ax().y(),self.ay().y(),self.az().y(),);
		let az=XYZW::from_xyz(self.ax().z(),self.ay().z(),self.az().z(),);
		let aw=self.inv_mul_axis(&-*self.aw());
		Matrix4(ax,ay,az,aw.to_point())
	}
	// todo: full 
}

impl<T:Float> Matrix3<Vec3<T>> {
	pub fn identity()->Matrix3<Vec3<T>>{
		Matrix3(
			VecMath::axis(0),
			VecMath::axis(1),
			VecMath::axis(2))
	}
	pub fn mul_vec3(&self,pt:&Vec3<T>)->Vec3<T>{
		let ax:&Vec3<T>=self.ax();
		self.ax().mul_x(pt).add_mul_y(self.ay(),pt).add_mul_z(self.az(),pt)
	}
	pub fn mul_matrix(&self,other:&Matrix3<Vec3<T>>)->Matrix3<Vec3<T>> {
		Matrix3(
			self.mul_vec3(other.ax()),
			self.mul_vec3(other.ay()),
			self.mul_vec3(other.az()))
	}
}

// operator overload: vector*matrix - assumes vec is transposed

impl<T:Float> PreMulVec4<T,Vec4<T>> for Matrix4<Vec4<T>> {
	fn pre_mul_vec4(&self,v: &Vec4<T>)->Vec4<T> {
		Vec4(
			v.dot(self.ax()),
			v.dot(self.ay()),
			v.dot(self.az()),
			v.dot(self.aw()))
	}
}

pub fn identity()->Matrix4<Vec4<f32>> {
	Matrix4::<Vec4<f32>>::identity()
}

//impl<F:Num+Zero+One> Matrix4<Vec4<F>> {
pub fn projection<F:FloatMath=f32>(tan_half_fov:F, aspect:F, znear:F, zfar:F)->Matrix4<Vec4<F>> {
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
	
	Matrix4(
		Vec4(w, zero(), zero(), zero()),
		Vec4(zero(), h, zero(), zero()),
		Vec4(a, b, c, -one::<F>()),
		Vec4(zero(), zero(), d, zero()),
	)
}

pub fn rotate_x<F:FloatMath=f32>(a:F)->Matrix4<Vec4<F>> {
	let (s,c)=a.sin_cos(); let one=one(); let zero=zero();
	Matrix4(
		XYZW::from_xyzw(one,	zero,	zero,	zero),
		XYZW::from_xyzw(zero,	c,		s,	zero),
		XYZW::from_xyzw(zero,	-s,		c,	zero),
		XYZW::from_xyzw(zero,	zero,	zero,	one))
}
pub fn rotate_y<F:FloatMath=f32>(a:F)->Matrix4<Vec4<F>> {
	let (s,c)=a.sin_cos(); let one=one(); let zero=zero();
	Matrix4(
		Vec4(c,		zero,	s,	zero),
		Vec4(zero,	one,	zero,	zero),
		Vec4(-s,		zero,	c,	zero),
		Vec4(zero,	zero,	zero,	one))
}
pub fn rotate_z<F:FloatMath=f32>(a:F)->Matrix4<Vec4<F>> {
	let (s,c)=a.sin_cos(); let one=one(); let zero=zero();
	Matrix4(
		Vec4(c,		s,	zero,	zero),
		Vec4(-s,		c,	zero,	zero),
		Vec4(zero,	zero,	one,	zero),
		Vec4(zero,	zero,	zero,	one))
}
pub fn rotate_xyz<F:FloatMath=f32, V:XYZW<F>=Vec3<F>>(r:&V)->Matrix4<Vec4<F>> {
	rotate_x(r.x())*rotate_y(r.y())*rotate_z(r.z())
}
pub fn rotate_xzy<F:FloatMath=f32,V:VecMath<F>=Vec4<F>>(r:&V)->Matrix4<Vec4<F>> {
	rotate_x(r.x())*rotate_z(r.z())*rotate_y(r.y())
}
pub fn rotate_zyx<F:FloatMath=f32, V:VecMath<F>=Vec4<F>>(r:&V)->Matrix4<Vec4<F>> {
	rotate_z(r.z())*rotate_y(r.y())*rotate_x(r.x())
}
pub fn rotate_yzx<F:FloatMath=f32 , V:XYZW<F>=Vec4<F>>(r:&V)->Matrix4<Vec4<F>> {
	rotate_y(r.y())*rotate_z(r.z())*rotate_x(r.x())
}

pub fn translate_xyz<F:FloatMath=f32>(x:F,y:F,z:F)->Matrix4<Vec4<F>> {
	let one=one(); let zero=zero();
	Matrix4(
		Vec4(one,	zero,	zero,	zero),
		Vec4(zero,	one,	zero,	zero),
		Vec4(zero,	zero,	one,	zero),
		Vec4(x,	y,	z,	one))
}
pub fn translate<V:XYZW<F>,F:FloatMath+One=f32>(trans:&V)->Matrix4<Vec4<F>> {
	let one=one(); let zero=zero();
	Matrix4(
		Vec4(one,	zero,	zero,	zero),
		Vec4(zero,	one,	zero,	zero),
		Vec4(zero,	zero,	one,	zero),
		Vec4(trans.x(),trans.y(),trans.z(),one))
}

pub fn scale_x<V:XYZW<F>,F:FloatMath=f32>(scale:F)->Matrix4<Vec4<F>> {
	let one=one(); let zero=zero();
	Matrix4(
		Vec4(scale,	zero,	zero,	zero),
		Vec4(zero,	one,	zero,	zero),
		Vec4(zero,	zero,	one,	zero),
		Vec4(zero,zero,zero,one))
}
pub fn scale_y<V:XYZW<F>,F:FloatMath=f32>(scale:F)->Matrix4<Vec4<F>> {
	let one=one(); let zero=zero();
	Matrix4(
		Vec4(one,	zero,	zero,	zero),
		Vec4(zero,	scale,	zero,	zero),
		Vec4(zero,	zero,	one,	zero),
		Vec4(zero,zero,zero,one))
}
pub fn scale_z<V:XYZW<F>,F:FloatMath=f32>(scale:F)->Matrix4<Vec4<F>> {
	let one=one(); let zero=zero();
	Matrix4(
		Vec4(one,	zero,	zero,	zero),
		Vec4(zero,	one,	zero,	zero),
		Vec4(zero,	zero,	scale,	zero),
		Vec4(zero,zero,zero,one))
}

pub fn scale<F:FloatMath+One=f32>(scale:F)->Matrix4<Vec4<F>> {
	let one=one(); let zero=zero();
	Matrix4(
		Vec4(scale,	zero,	zero,	zero),
		Vec4(zero,	scale,	zero,	zero),
		Vec4(zero,	zero,	scale,	zero),
		Vec4(zero,zero,zero,one))
}

pub fn scale_vec<V:XYZW<F>,F:FloatMath=f32>(s:&V)->Matrix4<Vec4<F>> {
	let one=one(); let zero=zero();
	Matrix4(
		Vec4(s.x(),	zero,	zero,	zero),
		Vec4(zero,	s.y(),	zero,	zero),
		Vec4(zero,	zero,	s.z(),	zero),
		Vec4(zero,zero,zero,one))
}

pub fn projection_frustum<F:Float=f32>(left:F,right:F, bottom:F, top:F, fov_radians:F, aspect:F, fnear:F, ffar:F)->Matrix4<Vec4<F>> {

    let two=one::<F>()+one::<F>();
    let a=(right+left)/(right-left);
    let b=(top+bottom)/(top-bottom);
    let c=-(ffar+fnear)/(ffar-fnear);
    let d=-(two*ffar*fnear/(ffar-fnear));
	Matrix4(
		Vec4(two*fnear/(right-left), zero(), zero(), zero()),
		Vec4(zero(), two*fnear/(top-bottom), zero(), zero()),
		Vec4(a, b, c, -one::<F>()),
		Vec4(zero(), zero(), d, zero()),
	)
}

pub fn projection_default()->Matrix4<Vec4<f32>> {
	let znear = 1.0f32/16.0f32;
	let zfar = 1024.0f32;
	projection_frustum(-0.5f32,0.5f32,-0.5f32,0.5f32, 90.0f32, 1.0f32, 0.5f32,5.0f32)
}


pub fn scale_rotate_translate<F:FloatMath=f32>(s:&Vec3<F>,r:&Vec3<F>,t:&Vec3<F>)->Matrix4<Vec4<F>> {
	translate(t)*rotate_zyx(r)*scale_vec(s)
}

pub fn from_quaternion<F:Float=f32+Copy>(q:&Vec4<F>)->Matrix4<Vec4<F>>{
	let Vec4(qx,qy,qz,qw)=*q;
	let two=one::<F>()+one::<F>();
	let zero=zero::<F>();
	let one=one::<F>();
	let qx2=qx*qx;
	let qy2=qy*qy;
	let qz2=qz*qz;
	Matrix4(
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
	fn to_matrix(&self)->Matrix4<Vec4<T>> {
		scale_rotate_translate(&self.scale,&self.rotate,&self.translate)
	}
	fn new()->SRT<T> { SRT{scale:Vec3(one(),one(),one()), rotate:Vec3(zero(),zero(),zero()), translate:Vec3(zero(),zero(),zero())}}
}

// combines vector operations with operations aware of a matrix..
pub trait PreMulMat4<V,OUT> {
	fn pre_mul_mat4(&self,mat:&Matrix4<V>)->OUT;
}

pub trait PreMulMat44<T,OUT> {
	fn pre_mul_mat44(&self,mat:&Matrix4<Vec4<T>>)->OUT;
}
pub trait PreMulMat43<T,OUT> {
	fn pre_mul_mat43(&self,mat:&Matrix4<Vec3<T>>)->OUT;
}
pub trait PreMulMat33<T,OUT> {
	fn pre_mul_mat33(&self,mat:&Matrix3<Vec3<T>>)->OUT;
}

impl<T:Float> PreMulMat43<T,Vec3<T>> for Vec3<T> {
	fn pre_mul_mat43(&self, mat:&Matrix4<Vec3<T>>)->Vec3<T> {mat.mul_vec3_w0(self)}
}
// Multiplying a Matrix44 ..

impl<T:Float> PreMulMat44<T,Vec4<T>> for Vec4<T> {
	fn pre_mul_mat44(&self, mat:&Matrix4<Vec4<T>>)->Vec4<T> {mat.mul_vec4(self)}
}
impl<T:Float> PreMulMat44<T,Matrix4<Vec4<T>>> for Matrix4<Vec4<T>> {
	fn pre_mul_mat44(&self, mat:&Matrix4<Vec4<T>>)->Matrix4<Vec4<T>> {mat.mul_matrix(self)}
}
impl<T:Float> PreMulMat33<T,Matrix3<Vec3<T>>> for Matrix3<Vec3<T>> {
	fn pre_mul_mat33(&self, mat:&Matrix3<Vec3<T>>)->Matrix3<Vec3<T>> {mat.mul_matrix(self)}
}
/*
impl<T:Float,V:VecMath<T>> PreMulMat4<V,V> for V {
	fn pre_mul_mat4(&self, mat:&Matrix4<V>)->V {mat.mul_vec4(self)}
}

impl<T:Float,V:VecMath<T>> PreMulMat4<V,Matrix4<V>> for Matrix4<V> {
	fn pre_mul_mat4(&self, mat:&Matrix4<V>)->Matrix4<V> {mat.mul_matrix(self)}
}
*/

/*
impl<T:MyFloat,RHS,OUT> Mul<RHS,OUT> for Matrix4<Vec4<T>>    {
	fn mul(&self, other:&RHS)->OUT { other.vpre_mul_mat44(self) }
}



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
impl<T:Float+Copy> Matrix4<Vec3<T>> {
	pub fn from_mat44(src:&Matrix4<Vec4<T>>)->Matrix4<Vec3<T>> {
		let Matrix4(ax,ay,az,aw)=*src;
		Matrix4(ax.to_vec3(), ay.to_vec3(),az.to_vec3(),aw.to_vec3())
	}
}

pub type Matrix33<T=f32> =Matrix3<Vec3<T>>;
pub type Matrix43<T=f32> =Matrix4<Vec3<T>>;
pub type Matrix44<T=f32> =Matrix4<Vec4<T>>;
pub struct Matrix33f(Matrix3<Vec3<f32>>);
pub struct Matrix43f(Matrix4<Vec3<f32>>);
pub struct Matrix44f(Matrix4<Vec4<f32>>);



