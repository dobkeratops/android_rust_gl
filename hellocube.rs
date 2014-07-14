use testcommon::*;
use tc=testcommon;

pub struct HelloCube<APP> {
	rotation:f32,
	timer:int,
}

impl<APP:rw::App> HelloCube<APP> {
	pub fn new()->HelloCube<APP> {
		HelloCube{rotation:0.0f32,timer:-100}
	}	
}

impl<APP:rw::App> rw::ViewController<APP> for HelloCube<APP> {
	fn win_event(&mut self,_:&mut APP,ev:&rw::WinEvent) {
	}
	fn update(&mut self, _:&mut APP, inp:&mut rw::InputSys)->rw::NextViewController<APP> {
		::std::io::println(format!("hello cube {:?}",self.timer));
		self.rotation+=1.0f32;
		self.timer+=1;
		if self.timer==0 {self.timer=-100;rw::CycleNext}else{rw::Continue}
	}
	fn render(&self, _:&APP, rs:&mut rw::RenderSys) {
		tc::glut_clear_perspective_view();

		::render_set_spinning_matrix(self.rotation);
		//displaySub(c_str("helloFromRust"));
		::render_test_stuff();
		::render_test_cube();
		::render_mesh_simple(&::makeTestMesh());
		//::render_blit_mem_test();
		::render_grid_xz(-1.0f32, (20,20),1.0, 0.9);
		
		::gl_cross_at((-10.0,1.0,10.0), (0.0,1.0,0.5,1.0),1.0);
		::gl_cross_at((10.0,1.0,-10.0), (0.0,0.5,1.0,1.0),1.0);
	}
	fn dump(&self)->~str { format!("hello cube {:?}", self.rotation) }
}




