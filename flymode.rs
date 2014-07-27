use r3d::*;
use bsprender::*;
use r3d::shaders::*;
use r3d::vertex::*;
use r3d::geom::Entity;
use rustwin::*;

pub struct Camera {
	pub ent:Entity,
	pub angvel:Vec3,
}

pub struct FlyMode {
	pub cam:Camera
}
impl FlyMode {
	pub fn new()->FlyMode {
		FlyMode{
			cam:Camera::new()
		}
	}
}

impl Camera{

	pub fn new()->Camera {
		Camera{
			ent:Entity{
				matrix:matrix::translate(&Vec4(2.0f32,2.0f32,-2.0f32,1.0f32)),
				vel:Vec3(1.0f32,0.0f32,0.0f32),
			},
			angvel:Vec3(0.0f32,0.0f32,0.0f32),
		}
	}
	pub fn view_matrix(&self)->Matrix4 {
		self.ent.matrix.inv_orthonormal()
	}
	pub fn update(&mut self, dt:f32) {
		let ax = self.ent.matrix.ax().to_vec3();
		let ay = self.ent.matrix.ay().to_vec3();
		let az = self.ent.matrix.az().to_vec3();
		let mut pos=self.ent.pos();
		let mut vel=self.ent.vel;

		let move_acc=5.0f32;
		let rot_acc=0.1f32;
		let dx = (-get_key_state('a')+get_key_state('d')) as f32 * move_acc;
		let dz = (-get_key_state('w')+get_key_state('s')) as f32 * move_acc;
		let dy = (-get_key_state('q')+get_key_state('e')) as f32 * move_acc;

		let rx = self.angvel.x()*0.9f32+(get_key_state('j')-get_key_state('l')) as f32 * rot_acc;
		let ry = self.angvel.y()*0.9f32+(get_key_state('k')-get_key_state('i')) as f32 * rot_acc;
		let rz = self.angvel.z()*0.9f32+(get_key_state('u')-get_key_state('o')) as f32 * rot_acc;

		vel=vel*0.95f32 + ax * dx + az * dz + ay*dy;
		self.ent.vel=vel;
		self.angvel=Vec3(rx,ry,rz);

		self.ent.matrix = Matrix4::look_along(
			&(pos+vel*dt).to_vec4_pos(),
			&(az+ax*rx*dt+ay*ry*dt).to_vec4(),
			&Vec3(zero(),one(),zero()).to_vec4());

		//rotation speed



	}
}

impl ::r3d::Screen for FlyMode {
	fn update(&mut self)->NextScreen {
		let dt=1.0f32/60.0f32;
		let cam=&mut self.cam;
		cam.update(dt);
//		let newpos=ent.pos()+ent.vel*dt;
//		ent.set_pos(newpos);
		
		Continue
	} 
	fn render(&self) {
		::render_clear();

		let cam=&self.cam;
		let mproj = matrix::projection(1.0f32,1.0f32,0.1f32,1024.0f32);
//(1.0f32,1.0f32,-0.1f32,100.0f32);
		let mati = matrix::identity()*matrix::translate_xyz(-2.0f32,-2.0f32,-2.0f32);
		dump!(cam.ent.matrix);
		let mat_to_cam = cam.view_matrix();
		dump!(mat_to_cam);
		dump!(mati);
		gl_matrix_projection(&mproj);
		gl_matrix_modelview(&mat_to_cam);
	//	draw_ground_grid();
		draw_grid_xz(4.0f32,32,0x000000u32);
//		::shadertest::render_spinning_lisajous();
	}

}







