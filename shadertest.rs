#![macro_escape]
use r3d::*;
use bsp::*;
use bsprender::*;
use r3d::shaders::*;
use r3d::vertex::*;

pub struct App {
	bsp:Option<Box<Blob<BspHeader>>>,
	bsprender:Option<Box<BspRender>>,
}
struct	RMesh 
{
	vertex_size:GLsizei,
	vbo:GLuint,
	ibo:GLuint,
	num_vertices:GLuint,num_indices:GLuint
}



struct	VertexAttr {
	pos:GLint,color:GLint,norm:GLint,tex0:GLint,tex1:GLint,joints:GLint,weights:GLint,tangent:GLint,binormal:GLint,
}
static g_vertex_attr_empty:VertexAttr=VertexAttr{
	pos:-1,color:-1,norm:-1,tex0:-1,tex1:-1,joints:-1,weights:-1,tangent:-1,binormal:-1
};

//static mut g_vertex_shader_attrib:VertexAttr=g_vertex_attr_empty;

//static mut g_shader_uniforms:UniformTable=g_uniform_table_empty;

// Paired pixel and vertex shaders.




unsafe fn create_texture(filename:String)->GLuint {
	return g_textures[0]
}


fn generate_torus_vertex(ij:uint, num_u:uint, num_v:uint)->Vertex {
	let pi=3.14159265f32;
	let tau=pi*2.0f32;
	let (i,j)=num::div_rem(ij, num_u);
	let fi=i.to_f32().unwrap_or(0.0) * (1.0 / num_u.to_f32().unwrap_or(0.0));
	let fj=j.to_f32().unwrap_or(0.0) * (1.0 / num_v.to_f32().unwrap_or(0.0));

	let rx=0.125f32;
	let ry=rx*0.33f32;
	let pi=3.14159265f32;
	let tau=pi*2.0f32;
	let (sx,cx)=(fi*tau).sin_cos();
	let (sy,cy)=(fj*tau).sin_cos();

	Vertex{
		pos:[(rx+sy*ry)*cx, (rx+sy*ry)*sx, ry*cy],
		color:[1.0,1.0,1.0,1.0],
		norm:[sy*cx, sy*sx, cy],
		tex0:[fi*8.0, fj*2.0],
	}	
}



impl RMesh {
	/// create a grid mesh , TODO - take a vertex generator
	fn new_torus((num_u,num_v):(uint,uint))->RMesh
	{
		// TODO: 2d fill array. from_fn_2f(numi,numj, &|..|->..)
		let strip_indices = (num_u+1)*2 +2;
		let num_indices=(num_v)*strip_indices;
		let indices=Vec::from_fn(num_indices,
			|ij|->GLuint{
				let (j,i1)=num::div_rem(ij, strip_indices);
				let i2=cmp::min(cmp::max(i1-1,0),num_u*2+1); // first,last value is repeated - degen tri.
				let (i,dj)=num::div_rem(i2,2);	// i hope that inlines to >> &
				(((j+dj)%num_v)*num_u+(i % num_u)) as GLuint
			}
		);
		
		let num_vertices=num_u*num_v;
		let vertices=Vec::from_fn(num_vertices,|i|generate_torus_vertex(i,num_u,num_v));

 		unsafe {
			RMesh{
				num_vertices:num_vertices as GLuint,
				num_indices:num_indices as GLuint,
				vertex_size: mem::size_of_val(vertices.get(0)) as GLsizei,
				vbo: create_vertex_buffer(&vertices),
				ibo: create_index_buffer(&indices)
			}
		}

	}
}


//extern void	TestGl_Idle();

//float	angle=0.f;
//GridMesh*	g_pGridMesh;
static mut g_grid_mesh:RMesh=RMesh{
	num_vertices:0,
	num_indices:0,
	vbo:-1,
	ibo:-1,
	vertex_size:0
};

type UniformIndex=GLint;

impl RMesh {
	fn	render_mesh_from_buffer(&self)
	{
		unsafe {
			use r3d::vertex::Vertex;

			let	client_state:[GLenum,..3]=[GL_VERTEX_ARRAY,GL_COLOR_ARRAY,GL_TEXTURE_COORD_ARRAY];
			for &x in client_state.iter() {glEnableClientState(x);};

			glBindBuffer(GL_ARRAY_BUFFER, self.vbo);
			glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.ibo);

			let baseVertex=0 as *const Vertex;
			let	stride=mem::size_of_val(&*baseVertex) as GLsizei;

			glVertexPointer(3, GL_FLOAT, stride,  0u as *const c_void);//(&(*baseVertex).pos[0]) as *f32 as *c_void);
			glColorPointer(4,GL_FLOAT, stride, 12u as *const c_void);//(&(*baseVertex).color[0]) as *f32 as *c_void);
			glTexCoordPointer(2, GL_FLOAT, stride, (12u+16u) as *const c_void);//(&(*baseVertex).tex0[0]) as *f32 as *c_void);
			glDrawElements(GL_TRIANGLE_STRIP, self.num_indices as GLsizei, GL_UNSIGNED_INT,0 as *const c_void);

			for &x in client_state.iter() {glDisableClientState(x);};
		}
	}
}

fn safe_set_uniform1i(loc:GLint, value:GLint) {
	// todo - validate
	unsafe {	
//		glUniform1i(loc, value);
	}
}
fn safe_set_uniform(loc:GLint, pvalue:&Vec4<f32>) {
	// todo - validate
	unsafe {	
		glUniform4fv(loc, 1, pvalue.ref0());
	}
}


static g_fog_color:Vec4<f32> =Vec4(0.25,0.5,0.5,1.0);
impl RMesh {
	unsafe fn	render_mesh_shader(&self)  {
		
		let clientState:[GLenum,..3]=[GL_VERTEX_ARRAY,GL_COLOR_ARRAY,GL_TEXTURE_COORD_ARRAY];

		glBindBuffer(GL_ARRAY_BUFFER, self.vbo);
		glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.ibo);


		match g_uniform_table {
			Some(ref ut)=>{
				safe_set_uniform1i(ut.uTex0, 0);
				safe_set_uniform1i(ut.uTex1, 1);
				safe_set_uniform(ut.uSpecularDir, &Vec4(0.032,0.707f32,0.707f32,0.0f32));
				safe_set_uniform(ut.uSpecularColor, &Vec4(1.0f32,0.75f32,0.5f32,0.0f32));
				safe_set_uniform(ut.uAmbient, &Vec4(0.25f32,0.25f32,0.25f32,1.0f32));
				safe_set_uniform(ut.uDiffuseDX, &Vec4(0.0f32,0.0f32,0.25f32,1.0f32));
				safe_set_uniform(ut.uDiffuseDY, &Vec4(0.5f32,0.5f32,0.5f32,1.0f32));
				safe_set_uniform(ut.uDiffuseDZ, &Vec4(0.25f32,0.0f32,0.0f32,1.0f32));
				safe_set_uniform(ut.uFogColor, &g_fog_color);
				safe_set_uniform(ut.uFogFalloff, &Vec4(0.5f32,0.25f32,0.0f32,0.0f32));
			},
			None=>io::println("error no uniform table!\n")
		}

		glActiveTexture(GL_TEXTURE0+0);
		glBindTexture(GL_TEXTURE_2D, g_textures[2]);
		glActiveTexture(GL_TEXTURE0+1);
		glBindTexture(GL_TEXTURE_2D, g_textures[1]);

		Vertex::set_vertex_attrib();

		glDrawElements(GL_TRIANGLE_STRIP, self.num_indices as GLsizei, GL_UNSIGNED_INT,0 as *const c_void);
	}
}

static mut g_angle:f32=0.0f32;
static mut g_frame:int=0;

static g_num_torus:int = 128;
/// render a load of meshes in a lissajous curve

#[no_mangle]
pub extern "C" fn	app_render(app:&mut App) 
{
	//logw("render noswap");

	unsafe {

		assert!(g_resources_init==true)		//logi!("render_no_swap"); // once..
		g_angle+=0.0025f32;

		glClearColor(g_fog_color.x()+(g_angle*2.0).sin(),g_fog_color.y(),g_fog_color.z(),g_fog_color.w());

		glClearDepthf(1.0f32);
		glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);
		glEnable(GL_DEPTH_TEST);
		glDepthMask(GL_TRUE);
		glDepthFunc(GL_LEQUAL);

		glEnable(GL_CULL_FACE);
		let matI = matrix::identity();
		let matP = matrix::projection_frustum(-0.5f32,0.5f32,-0.5f32,0.5f32, 90.0f32, 1.0f32, 0.5f32,5.0f32);
		gl_matrix_projection(&matP);


		let pi=3.14159265f32;
		let tau=pi*2.0f32;

		let r0 = 1.0f32;
		let r1 = 0.5f32;
		let sda=0.25f32;
		let mut a0=g_angle*1.1f32+0.1f32;
		let mut a1=g_angle*1.09f32+1.5f32;
		let mut a2=g_angle*1.05f32+0.5f32;
		let mut a3=g_angle*1.11f32;
		let mut a4=g_angle*1.11f32+0.7f32;
		let mut a5=g_angle*1.105f32;
		let da0=tau*0.071f32*sda;
		let da1=tau*0.042f32*sda;
		let da2=tau*0.081f32*sda;
		let da3=tau*0.091f32*sda;
		let da4=tau*0.153f32*sda;
		let da5=tau*0.1621f32*sda;

		// render spinning tori
		for i in range(0,g_num_torus) {

			let matT = matrix::translate_xyz(
				a0.cos()*r0+a3.cos()*r1, 
				a1.cos()*r0+a4.cos()*r1, 
				a2.cos()*r0+a5.cos()*r1 -2.0*r0);

			let rot_x = matrix::rotate_x(a0);
			let rot_y = matrix::rotate_x(a1*0.245f32);
			let rot_xy=rot_x.mul_matrix(&rot_y);
			let rot_trans = matT.mul_matrix(&rot_xy);
	
			let matMV = matT;	// toodo - combine rotation...
			//io::println(format!("{:?}", g_shader_program));

			gl_matrix_projection(&matP);
			gl_matrix_modelview(&rot_trans);

			glUseProgram(g_shader_program);
			match g_uniform_table {
				Some(ref ut)=>{
					glUniformMatrix4fvARB(ut.uMatProj, 1,  GL_FALSE, matP.ax().ref0());
					glUniformMatrix4fvARB(ut.uMatModelView, 1, GL_FALSE, rot_trans.ax().ref0());
				},
				None=>{assert!(false,"no shader uniforms")}
			}

			g_grid_mesh.render_mesh_shader();

			a0+=da0;a1+=da1;a2+=da2;a3+=da3;a4+=da4;a5+=da5;

			if (i & 15) == 0{
				draw_cross(0.2f32);
			}
		}

		// render bsp level
		match app.bsprender {
			Some(ref x)=>{
				glUseProgram(0);
				let rot_x = matrix::rotate_x(a0);
				let rot_y = matrix::rotate_x(a1*0.245f32);
				let rot_xy=rot_x.mul_matrix(&rot_y);
				let trans=matrix::translate(&Vec4(0.0f32,0.0f32,-1.0f32,1.0f32));
				let rt=trans*rot_xy;
				gl_matrix_modelview(&rt);
				x.render();
			}
			None=>{}
		}


		g_frame+=1;
	}
}


fn	create_textures() {
//	static_assert(sizeof(GLuint)==sizeof(int));
	unsafe {
		glGenTextures(1,&mut g_textures[0]);
		glBindTexture(GL_TEXTURE_2D,g_textures[0]);
		glTexEnvi( GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_MODULATE as GLint);
		glTexParameteri( GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR  as GLint);
		glTexParameteri( GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR  as GLint);
		glTexParameteri( GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT  as GLint);
		glTexParameteri( GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT  as GLint);



		let	(usize,vsize)=(256,256);
		let buffer = Vec::<u32>::from_fn(usize*vsize,|index|{
				let (i,j)=num::div_rem(index,usize);
				(i+j*256+255*256*256) as u32
			});
		for i in range(0 as GLint,8 as GLint) {
			glTexImage2D(GL_TEXTURE_2D, i, GL_RGB as GLint, usize as GLint,vsize as GLint, 0, GL_RGB, GL_UNSIGNED_BYTE, buffer.get(0)as*const _ as*const c_void);
			glGenerateMipmap(GL_TEXTURE_2D);
		}
		glBindTexture(GL_TEXTURE_2D,0);
		for i in range(1u,5u) { g_textures[i as uint]=g_textures[0]}
//		g_textures[0]=
	
//		g_textures[1] = get_texture(&"data/rocktile.tga");
//		g_textures[4] = get_texture(&"data/pebbles_texture.tga");
//		g_textures[3] = get_texture(&"data/grass.tga");
//		g_textures[2] = get_texture(&"data/cliffs.tga");
	}
}

static mut g_resources_init:bool=false;

#[no_mangle]
pub extern "C" fn app_display_create(app:&mut App) {
	unsafe {
		logi!("shadertest Create Resources \n");
		create_shaders();
		create_textures();
		g_grid_mesh = RMesh::new_torus((16,16)); //new GridMesh(16,16);
		g_resources_init=true;
		let bsp:Option<Box<Blob<BspHeader>>> =app.bsp.take();
		if bsp.is_some() {
			app.bsprender=Some(box BspRender::new(bsp.unwrap()));
			
		}
	}
}
#[no_mangle]
pub extern "C" fn app_display_destroy(_:&mut App) {
	unsafe {
		g_resources_init=false;
	}
}

#[no_mangle]
pub extern "C" fn app_destroy(_:Box<App>) {
}


#[no_mangle]
pub extern "C" fn app_create(argc:c_int, argv:*const *const c_char, w:c_int,h:c_int)->Box<App> {
	box App{
		bsp: Some(box Blob::<BspHeader>::read(&Path::new("data/e1m1.bsp"))),
		bsprender:None
	}
}



