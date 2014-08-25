#![macro_escape]
use super::gl::*;
use super::glut::*;
use super::ut::*;

/// Defines a vertex structure with embedded  attribute index annotations & GL type enums; 
/// generates an corresponding function to set gl vertex attribute data.
/// TODO: change that to createy a data-table.
macro_rules! def_vertex_format{
	( struct $layout_name:ident {
			$($element:ident : [$elem_type:ident($elem_enum:expr),..$elem_count:expr]( $elem_index:expr)  ),*  
		}
	)=>(
//		mod $layout_name {
			pub struct $layout_name {
				$( pub $element: [$elem_type ,.. $elem_count],)*
			}
			impl $layout_name {
				pub fn set_vertex_attrib() {
					use r3d::gl::{GLuint,GLfloat,GLsizei,glVertexAttribPointer,glEnableVertexAttribArray};
					use r3d::gl_h_consts::{GL_FLOAT,GL_FALSE};
					use std::intrinsics::size_of;
					use libc::c_void;
					$( unsafe {
							let base_vertex = 0 as *const $layout_name;
							glEnableVertexAttribArray($elem_index as GLuint);
							glVertexAttribPointer(
								$elem_index as GLuint, 
								$elem_count,
								$elem_enum,	// todo: type -> GL type.
								GL_FALSE, 
								size_of::<$layout_name>() as GLsizei,
								&(*base_vertex).$element as *const GLfloat as *const c_void,
							);
						}
					);*
				}
			}
//		}	
	)
}

macro_rules! def_vertex_attrib(
	( enum $attrib_group_name:ident { $($attr_name:ident),* } ) =>(
		pub enum $attrib_group_name {
			$($attr_name),*
		}
		impl $attrib_group_name {
			pub fn bind_attribs(prog:GLuint) {
				use r3d::gl::{GLuint,GLfloat,GLsizei,glBindAttribLocation};
				unsafe {
					$(glBindAttribLocation(prog, $attr_name as GLuint, c_str( stringify!($attr_name) ) );
					)*
				}
			}
		}
	)
)

macro_rules! def_uniform_table {
	(struct $uniform_table:ident { $($uniform_name:ident),* }) => (
		pub struct $uniform_table {
			$( pub $uniform_name : GLint),*
		}

		// TODO: this could be purely data-driven for data-linked shaders,
		// but we want to expose it conviniently to rust code..

		impl $uniform_table {
			pub fn new(prog:GLuint)->$uniform_table {
				$uniform_table {
				$(
					$uniform_name : {
						
						let x=unsafe{ get_uniform_location(prog,  stringify!($uniform_name))};
						logi!("\t {}.{}={}",stringify!($uniform_table),stringify!($uniform_name),x); 
						x 
					}	
				),*
				}
			}
		}
	)
}


// todo, figure out the macro call passing those var args..




pub unsafe fn create_buffer(size:GLsizei, data:*const c_void, buffer_type:GLenum)->GLuint {
	let mut id:GLuint=0;
	glGenBuffers(1,&mut id);
	glBindBuffer(buffer_type,id);
	
	glBufferData(buffer_type, size, data, GL_STATIC_DRAW);
	// error..
	glBindBuffer(buffer_type,0);
	id
}

// Foo bar baz

pub unsafe fn create_vertex_buffer_from_ptr(size:GLsizei, data:*const c_void)->GLuint {
	create_buffer(size,data,GL_ARRAY_BUFFER)
}
pub unsafe fn create_index_buffer_from_ptr(size:GLsizei, data:*const c_void)->GLuint {
	create_buffer(size,data,GL_ELEMENT_ARRAY_BUFFER)
}

pub unsafe fn create_vertex_buffer<T>(data:&Vec<T>)->GLuint {
	create_buffer(data.len()as GLsizei *mem::size_of::<T>() as GLsizei, as_void_ptr(&data[0]), GL_ARRAY_BUFFER)
}
pub unsafe fn create_index_buffer<T>(data:&Vec<T>)->GLuint {
	create_buffer(data.len()as GLsizei *mem::size_of::<T>() as GLsizei, as_void_ptr(&data[0]), GL_ELEMENT_ARRAY_BUFFER)
}


pub unsafe fn get_attrib_location(shader_prog:GLuint, name:&str)->GLint {
	let r=glGetAttribLocation(shader_prog, c_str(name));
	println!("get attrib location {:?}={:?}", name, r);
	r
}
pub unsafe fn get_uniform_location(shader_prog:GLuint, name:&str)->GLint {
	let r=glGetUniformLocation(shader_prog, c_str(name));
	r
}

pub unsafe fn	create_and_compile_shader(shader_type:GLenum, source:&Vec<&str>) ->GLuint
{
	use std::iter::Range;

	logi!("create_and_compile_shader")
	let	shader_id = glCreateShader(shader_type );
	dump!(shader_id);

	let sources_as_c_str=Vec::from_fn(source.len(), |x|c_str(source[x]) );
	let length:Vec<c_int> = Vec::from_fn(source.len() , |x|source[x].len() as c_int );
	let src_len:uint=source.len();
	let mut iter_len: ::std::iter::Range<uint> =::std::iter::range(0u,src_len as uint);

	let mut it = range(0,src_len); let x: Option<uint> = it.next(); println!("{}", x);

//	for i in iter_len { 
	loop {
		match iter_len.next() {
			Some(i)=>{
				let s=sources_as_c_str[i];
				let len=length[i];
				logi!("source adr={} source len={} ",s,len) 
			},
			None=> break
		}
	};
	
	glShaderSource(shader_id, source.len() as GLsizei, &sources_as_c_str[0], 0 as *const c_int/*(&length[0])*/);
	glCompileShader(shader_id);
	let	mut sh_status:c_int=0;
	glGetShaderiv(shader_id,GL_COMPILE_STATUS,&mut sh_status);
	dump!(sh_status);
	if sh_status==GL_FALSE as GLint
	{
		logi!("failed, getting log..");
		let mut compile_log:[c_char,..512]=[0 as c_char,..512]; //int len;
	
		let mut log_len:c_int=0;
		glGetShaderInfoLog(shader_id, 512,&mut log_len as *mut c_int, &mut compile_log[0]);
		logi!("Compile Shader Failed: logsize={:?}",
				log_len);
		logi!("compile shader {:?} failed: \n{:?}\n", shader_id, 
			c_str::CString::new(&compile_log[0],false).as_str());

		for s in source.iter() { logi!("{:?}",*s) }
		logi!("{:?}",
			match c_str::CString::new(&compile_log[0],false).as_str() {
				Some(s)=>s,
				None=>"couldn't unwrap error lol",
			}
		);
		loop{}
	}	
	else {
		logi!("create shader{:?} - compile suceeded\n",  shader_id);
	}
	logi!("create shader-done");
	shader_id
}

pub type VertexShader=GLuint;
pub type PixelShader=GLuint;
pub type ShaderProgram=GLuint;




// TODO [cfg OPENGL_ES ..]
static shader_prefix_desktop:&'static str="\
#version 120	\n\
#define highp	\n\
#define mediump	\n\
#define lowp	\n\
";


static vertex_shader_prefix_gles:&'static str="\
#version 100			\n\
precision highp float;	\n\
";

//#version 100			\n\
static pixel_shader_prefix_gles:&'static str="\
precision mediump float;	\n\
";


#[cfg(target_os = "android")]
pub fn get_shader_prefix(is_ps:int)->&'static str {
	if is_ps==0 {vertex_shader_prefix_gles} else {pixel_shader_prefix_gles}
}

#[cfg(not(target_os = "android"))]
pub fn get_shader_prefix(is_ps:int)->&'static str {
	shader_prefix_desktop
}

pub fn check_shader_error(prog:ShaderProgram) {
	unsafe {
		let mut err:GLint=0;
		glGetProgramiv(prog,GL_LINK_STATUS,(&mut err) as *mut GLint);

		if err as GLenum==GL_INVALID_VALUE || err as GLenum==GL_INVALID_OPERATION  {
			let mut buffer=[0 as GLchar,..1024];
			let mut len:GLint=0;
			glGetProgramInfoLog(prog,1024,&mut len,&mut buffer[0]);
			logi!("link program failed: {:?}",err);
			logi!("{:?}",c_str::CString::new(&buffer[0],false).as_str().unwrap());
		} else {
			logi!("link program status {:?}", err);
		}
	}
}

pub fn create_shader_program(ps:PixelShader,vs:VertexShader,f_bind_attribs:|p:ShaderProgram|)->ShaderProgram {
	unsafe {
		let	prog = glCreateProgram();
		glAttachShader(prog, ps);
		glAttachShader(prog, vs);

	logi!("linking verteshader{:?}, pixelshader{:?} to program{:?}\n", vs, ps, prog);
		f_bind_attribs(prog);
 		glLinkProgram(prog);
		let x=glGetAttribLocation(prog,c_str("a_color"));
		logi!("write,read attrib location in prog {:?} a_color={:?}", prog, x);

		check_shader_error(prog);
		prog
	}
}

pub fn render_glut_init(arg_argc:c_int, argv:*const*const c_char) {
	unsafe{
		let mut argc=arg_argc;
		glutInit((&mut argc) as *mut c_int,0 as *const *const c_char );
		glutInitDisplayMode(GLUT_DEPTH | GLUT_DOUBLE | GLUT_RGBA);
		glutInitWindowSize(1280,800);
		let win=verify!(glutCreateWindow(c_str("Rust ShaderTest")) isnt 0);
		glutIdleFunc(null_func as *const u8);
        glutDisplayFunc(null_func as *const u8); 
		glDrawBuffer(GL_BACK);
		glutReshapeWindow(1024,1024);
		glEnable(GL_DEPTH_TEST);
	}
}

pub fn render_clear()
{
	unsafe {
//		glClearColor(g_fog_color.x()+(g_angle*2.0).sin(),g_fog_color.y(),g_fog_color.z(),g_fog_color.w());
		glClearColor(0.5f32,0.5f32,0.5f32,1.0f32);

		glClearDepthf(1.0f32);
		glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);
		glEnable(GL_DEPTH_TEST);
		glDepthMask(GL_TRUE);
		glDepthFunc(GL_LEQUAL);

		glEnable(GL_CULL_FACE);
	}
}

extern "C" fn null_func() {
}



