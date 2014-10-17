#![feature(import_shadowing)]
//use r3d;
use r3d::*;
use bsp::bsp::{BspHeader,Blob};
use bsprender::*;
use rustwin::*;
use shadertest::*;

pub struct Camera {
	pub ent: Entity,
	pub angvel:Vec3,
}

pub struct FlyMode {
	cam:Camera,
	bsp:Option<Box<Blob<BspHeader>>>,
	bsprender:Option<Box<BspRender>>,
}

impl FlyMode {
	pub fn new()->FlyMode {
		FlyMode{
			cam:Camera::new(),
			bsp: Some(box Blob::<BspHeader>::read(&Path::new("data/e1m1.bsp"))),
			bsprender:None,
		}
	}
}

impl Camera{

	pub fn new()->Camera {
		Camera{
			ent: Entity{
				matrix:matrix::translate(&Vec3(0.0f32,0.0f32,0.0f32)),
				vel:Vec3(1.0f32,0.0f32,0.0f32),
			},
			angvel:Vec3(0.0f32,0.0f32,0.0f32),
		}
	}
	pub fn view_matrix(&self)->Matrix44 {
		self.ent.matrix.inv_orthonormal()
	}
	pub fn update(&mut self, dt:f32) {
		let ax = self.ent.matrix.0.to_vec3();
		let ay = self.ent.matrix.1.to_vec3();
		let az = self.ent.matrix.2.to_vec3();
		let mut pos=self.ent.pos();
		let mut vel=self.ent.vel;

		let move_acc=2.0f32;
		let rot_acc=0.1f32;

		fn ctrl_axis(a:char,b:char)->f32 { (get_key_state(b)-get_key_state(a)) as f32}
		let dx = ctrl_axis('a','d') * move_acc;
		let dz = ctrl_axis('w','s') * move_acc;
		let dy = ctrl_axis('q','e') * move_acc;

		let rx = self.angvel.0*0.95f32+ctrl_axis('l','j') * rot_acc;
		let ry = self.angvel.1*0.95f32+ctrl_axis('i','k') * rot_acc;
		let rz = self.angvel.2*0.95f32+ctrl_axis('o','u') * rot_acc;

		vel=vel*0.98f32 + ax * dx + az * dz + ay*dy;
		self.ent.vel=vel;
		self.angvel=Vec3(rx,ry,rz);

		self.ent.matrix = Matrix44::look_along(
			&(pos+vel*dt).to_vec4().set_w(one()),
			&(az+ax*rx*dt+ay*ry*dt).to_vec4(),
			&Vec3(zero(),one(),zero()).to_vec4());
		//rotation speed
	}
}

impl Screen for FlyMode {

	fn display_create(&mut self) {
		match self.bsp.take() {
			Some(bsp)=>{self.bsprender=Some(box BspRender::new(bsp));},
			_=>{},
		}
	}
	fn update(&mut self)->ScreenChange {
		let dt=1.0f32/60.0f32;
		let cam=&mut self.cam;
		cam.update(dt);
//		let newpos=ent.pos()+ent.vel*dt;
//		ent.set_pos(newpos);
		
		ScContinue
	} 
	fn render(&self) {
		::render_clear();

		let cam=&self.cam;
		let mat_proj = matrix::projection(1.0f32,1.0f32,0.1f32,2048.0f32);
//(1.0f32,1.0f32,-0.1f32,100.0f32);
		let mati = matrix::identity()*matrix::translate_xyz(-2.0f32,-2.0f32,-2.0f32);
		let mat_to_cam = cam.view_matrix();
		gl_matrix_projection(&mat_proj);
		gl_matrix_modelview(&mat_to_cam);
	//	draw_ground_grid();
		draw_grid_xz(4.0f32,32,0x000000u32);
//		::shadertest::render_spinning_lisajous();
		::render_from_at(&mat_proj,&mat_to_cam, &matrix::identity(), &self.bsprender);

	}

}







