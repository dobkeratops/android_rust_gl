use r3d::*;
use bsprender::*;
use r3d::shaders::*;
use r3d::vertex::*;
use r3d::geom::Entity;

pub struct Camera {
	pub ent:Entity,
}

pub struct FlyMode {
	pub cam:Camera
}
impl FlyMode {
	pub fn new()->FlyMode {
		FlyMode{
			cam:Camera{
				ent:Entity{
					matrix:matrix::translate(&Vec4(2.0f32,2.0f32,-2.0f32,1.0f32)),
//			*matrix::rotate_y(123912.0f32)
//			*matrix::rotate_z(123.0f32),
					vel:Vec3(1.0f32,0.0f32,0.0f32),
				}
			}
		}
	}
}

impl ::r3d::Screen for FlyMode {
	fn update(&mut self)->NextScreen {
		let dt=1.0f32/60.0f32;
		let ent=&mut self.cam.ent;
		let newpos=ent.pos()+ent.vel*dt;
		ent.set_pos(newpos);
		Continue
	}
	fn render(&self) {
		::render_clear();

		let cam=&self.cam;
		let mproj = matrix::projection(1.0f32,1.0f32,0.1f32,1024.0f32);
//(1.0f32,1.0f32,-0.1f32,100.0f32);
		let mati = matrix::identity()*matrix::translate_xyz(-2.0f32,-2.0f32,-2.0f32);
		dump!(cam.ent.matrix);
		let mat_to_cam = cam.ent.matrix.inv_orthonormal();
		dump!(mat_to_cam);
		dump!(mati);
		gl_matrix_projection(&mproj);
		gl_matrix_modelview(&mat_to_cam);
		draw_ground_grid();
//		::shadertest::render_spinning_lisajous();
	}

}







