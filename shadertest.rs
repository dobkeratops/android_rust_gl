//pub use r3d::landscape::*;
//pub use r3d::mesh::*;
#[feature(globs)];
#[feature(macro_rules)];
#[feature(default_type_params)];
#[allow(unused_variable)];
#[allow(dead_code)];
#[allow(unreachable_code)];
#[allow(unused_unsafe)];
#[allow(unused_mut)];
#[macro_escape];


pub use std::num;
pub use std::vec;
pub use std::mem;
pub use std::cmp;
pub use std::c_str;
pub use std::libc;
use macros::*;

pub use r3d::*;
pub use r3d::matrix::*;
pub use r3d::vecmath::*;
pub use r3d::rawglbinding::*;
pub use std::io;
use gl=r3d::rawglbinding;


// todo, figure out the macro call passing those var args..





/*
enum VertexAttrIndex
{
	VAI_pos=0,
	VAI_color,
	VAI_normal,
	VAI_tex0,
	VAI_tex1,
	VAI_count
}
*/
//GFX gfx;
/*
struct	TestVertex 
{
	pos:[f32,..3],
	color:[f32,..4],
	norm:[f32,..3],
	tex0:[f32,..2]
}
*/

def_vertex_format!(
	struct TestVertex {
		pos:	[f32(GL_FLOAT),..3](::shadertest::VAI_pos),
		color:	[f32(GL_FLOAT),..4](::shadertest::VAI_color),
		norm:	[f32(GL_FLOAT),..3](::shadertest::VAI_norm),
		tex0:	[f32(GL_FLOAT),..2](::shadertest::VAI_tex0)
	}
)

//type TestVertex = TestVertex::TestVertex;

//typedef int IndexType;
//	enum {IndexSize = sizeof(IndexType) };
//typedef	::TestVertex Vertex;
struct	Mesh 
{
	vertex_size:GLsizei,
	vbo:GLuint,
	ibo:GLuint,
	num_vertices:GLuint,num_indices:GLuint
}

static mut g_textures:[GLuint,..5]=[0,..5];
static mut g_shader_program:GLuint=-1;
static mut g_pixel_shader:GLuint=-1;
static mut g_vertex_shader:GLuint=-1;

enum VertexAttrIndex {
	VAI_pos		=0x0000,
	VAI_color,
	VAI_norm,
	VAI_tex0,
	VAI_tex1,
	VAI_joints,
	VAI_weights,
	VAI_tangent,
	VAI_binormal,
	VAI_count
}




unsafe fn get_attrib_location(shader_prog:GLuint, name:&str)->GLint {
	let r=glGetAttribLocation(shader_prog, c_str(name));
	println!("get attrib location {:?}={:?}", name, r);
	r
}
unsafe fn get_uniform_location(shader_prog:GLuint, name:&str)->GLint {
	let r=glGetUniformLocation(shader_prog, c_str(name));
	r
}

unsafe fn	create_and_compile_shader(shader_type:GLenum, source:&~[&str]) ->GLuint
{
	logi!("create_and_compile_shader");
	let	shader = glCreateShader(shader_type );
	logi!("shader={:?}",shader);

	let sources_as_c_str=vec::from_fn(source.len(), |x|c_str(source[x]) );
	let length = vec::from_fn(source.len() , |x|source[x].len() as c_int );
	for i in range(0,source.len()) { logi!("source adr={:?} source len={:?} ",sources_as_c_str[i],length[i]) };
	
	logi!("set shader source..");
	glShaderSource(shader, source.len() as GLsizei, &sources_as_c_str[0], 0 as *c_int/*(&length[0])*/);
	logi!("compile..");
	glCompileShader(shader);
	let	status:c_int=0;
	logi!("get status..");
	glGetShaderiv(shader,GL_COMPILE_STATUS,&status);
	logi!("got status");
	logi!("status = {:?}",status);
	if status==GL_FALSE as GLint
	{
		logi!("failed, getting log..");
		let compile_log:[c_char,..512]=[0 as c_char,..512]; //int len;
	
		let log_len:c_int=0;
		glGetShaderInfoLog(shader, 512,&log_len as *c_int, &compile_log[0]);
		logi!("Compile Shader Failed: logsize={:?}",
				log_len);
		
		logi!("compile shader {:?} failed: \n{:?}\n", shader, 
			c_str::CString::new(&compile_log[0],false).as_str());

		for s in source.iter() { logi!("{:?}",*s) }
		logi!("{:?}",
			match c_str::CString::new(&compile_log[0],false).as_str() {
				Some(s)=>s,
				None=>"couldn't unwrap error lol",
			}
		);
		for i in range(0,log_len) {
			logi!("{:?}",compile_log[i]);
		}
		loop{}

	}	
	else {

		logi!("create shader{:?} - compile suceeded\n",  shader);
	}
	logi!("create shader-done");
	shader
}

struct	VertexAttr {
	pos:GLint,color:GLint,norm:GLint,tex0:GLint,tex1:GLint,joints:GLint,weights:GLint,tangent:GLint,binormal:GLint,
}
static g_vertex_attr_empty:VertexAttr=VertexAttr{
	pos:-1,color:-1,norm:-1,tex0:-1,tex1:-1,joints:-1,weights:-1,tangent:-1,binormal:-1
};

static mut g_vertex_shader_attrib:VertexAttr=g_vertex_attr_empty;

static mut g_shader_uniforms:UniformTable=g_uniform_table_empty;

// Paired pixel and vertex shaders.

pub type VertexShader=GLuint;
pub type PixelShader=GLuint;
pub type ShaderProgram=GLuint;

unsafe fn create_texture(filename:~str)->GLuint {
	return g_textures[0]
}

pub unsafe fn c_str(s:&str)->*c_char {
	s.to_c_str().unwrap()
}
extern {pub fn bind_attrib_locations(prog:c_uint);}

unsafe fn	create_shader_program(
			pixelShaderSource:&~[&str],
			vertexShaderSource:&~[&str])->(PixelShader,VertexShader,ShaderProgram)
{

	logi!("create_shader_program");

	let pixelShaderOut = create_and_compile_shader(GL_FRAGMENT_SHADER, pixelShaderSource);
	let vertexShaderOut = create_and_compile_shader(GL_VERTEX_SHADER, vertexShaderSource);	let	prog = glCreateProgram();
	logi!("bind attrib locations");
	
	// assign attribute names before linking

	glBindAttribLocation(prog, VAI_pos as GLuint, c_str("a_pos"));
	glBindAttribLocation(prog, VAI_color as GLuint, c_str("a_color"));
	glBindAttribLocation(prog, VAI_norm as GLuint, c_str("a_norm"));
	glBindAttribLocation(prog, VAI_tex0 as GLuint, c_str("a_tex0"));
	glBindAttribLocation(prog, VAI_tex1 as GLuint, c_str("a_tex1"));
	glBindAttribLocation(prog, VAI_joints as GLuint, c_str("a_joints"));
	glBindAttribLocation(prog, VAI_weights as GLuint, c_str("a_weights"));
	glBindAttribLocation(prog, VAI_tangent as GLuint, c_str("a_tangent"));
	glBindAttribLocation(prog, VAI_binormal as GLuint, c_str("a_binormal"));

	glAttachShader(prog, pixelShaderOut);
	glAttachShader(prog, vertexShaderOut);

	logi!("linking verteshader{:?}, pixelshader{:?} to program{:?}\n", vertexShaderOut, pixelShaderOut, prog);
	glLinkProgram(prog);
	let mut err:GLint=0;
	glGetProgramiv(prog,GL_LINK_STATUS,(&err) as *GLint);
	
	let x=glGetAttribLocation(prog,c_str("a_color"));
	logi!("write,read attrib location in prog {:?} a_color={:?}", prog, x);

	
	if err as GLenum==GL_INVALID_VALUE || err as GLenum==GL_INVALID_OPERATION {
		let mut buffer=[0 as GLchar,..1024];
		let mut len:GLint=0;
		glGetProgramInfoLog(prog,1024,&len,&buffer[0]);
		logi!("link program failed: {:?}",err);
		logi!("{:?}",c_str::CString::new(&buffer[0],false).as_str().unwrap());
	} else {
		logi!("link program status {:?}", err);
	}

	(pixelShaderOut,vertexShaderOut,prog)
}

//TODO: split into default uniforms, default vertex, default vertex-shader-out


//#if OPENGL_ES
//#define SHADER_PREFIX \
//"#version 100\n"\
//"precision highp float;"
//#else
//#define SHADER_PREFIX \
//"#version 120\n"\
//"#define highp\n"\
//"#define mediump\n"\
//"#define lowp\n"
//#endif

// TODO [cfg OPENGL_ES ..]
static shader_prefix_desktop:&'static str=&"\
#version 120	\n\
#define highp	\n\
#define mediump	\n\
#define lowp	\n\
";


static vertex_shader_prefix_gles:&'static str=&"\
#version 100			\n\
precision highp float;	\n\
";

//#version 100			\n\
static pixel_shader_prefix_gles:&'static str=&"\
precision mediump float;	\n\
";


//#define PS_VS_INTERFACE0
static ps_vs_interface0:&'static str=&
"varying	highp vec4 v_pos;\n\
varying	highp vec4 v_color;\n\
varying	highp vec3 v_norm;\n\
varying	highp vec2 v_tex0;\n\
varying	highp vec3 v_tex1;\n\
varying	highp vec4 v_tangent;\n\
varying	highp vec4 v_binormal;\n";


//#define PS_VERTEX_FORMAT0
static ps_vertex_format0:&'static str=&
"attribute vec3 a_pos;\n\
attribute vec2 a_tex0;\n\
attribute vec4 a_color;\n\
attribute vec3 a_norm;\n";

static g_VS_Default:&'static str=
//SHADER_PREFIX
//PS_VERTEX_FORMAT0
//PS_VS_INTERFACE0
&"uniform mat4 uMatProj;\n\
uniform mat4 uMatModelView;\n\
void main() {\n\
	vec4 posw = vec4(a_pos.xyz,1.0);\n\
	vec4 epos = uMatModelView * pos4;\n\
	vec3 enorm = (uMatModelView * vec4(a_norm.xyz,0.0)).xyz;\n\
	vec4 spos=uMatProj * epos;\n\
	gl_Position = spos;\n\
	v_pos = posw;\n\
	v_color = a_color;\n\
	v_tex0 = a_tex0;\n\
	v_tex1 = a_pos.xyz;\n\
	v_norm = enorm;\n\
}";

/// replacement debug vertex shader - dont apply transformations, just view vertices..
static g_VS_PassThru:&'static str=
//SHADER_PREFIX
//PS_VERTEX_FORMAT0
//PS_VS_INTERFACE0
&"uniform mat4 uMatProj;\n\
uniform mat4 uMatModelView;\n\
void main() {\n\
	vec4 posw = vec4(a_pos.xyz,1.0);\n\
	vec4 epos = uMatModelView * posw;\n\
	vec3 enorm = (uMatModelView * vec4(a_norm.xyz,0.0)).xyz;\n\
	vec4 spos=uMatProj * epos;\n\
	gl_Position = vec4(a_pos.xyz,1.0);\n\
	v_pos = epos;\n\
	v_color = a_color;\n\
	v_tex0 = a_tex0;\n\
	v_tex1 = a_pos.xyz;\n\
	v_norm = enorm;\n\
}";
/// replacement debug vertex shader - dont apply perspective, just view translated models
static g_VS_Translate2d:&'static str=
//SHADER_PREFIX
//PS_VERTEX_FORMAT0
//PS_VS_INTERFACE0
&"uniform mat4 uMatProj;\n\
uniform mat4 uMatModelView;\n\
void main() {\n\
	vec4 posw = vec4(a_pos.xyz,1.0);\n\
	vec4 epos = uMatModelView * posw;\n\
	vec3 enorm = (uMatModelView * vec4(a_norm.xyz,0.0)).xyz;\n\
	vec4 spos=uMatProj * epos;\n\
	gl_Position = vec4(a_pos.xyz,1.0)+uMatModelView[3].xyzw;\n\
	v_pos = posw;\n\
	v_color = a_color;\n\
	v_tex0 = a_tex0;\n\
	v_tex1 = a_pos.xyz;\n\
	v_norm = enorm;\n\
}";
static g_VS_Persp:&'static str=
//SHADER_PREFIX
//PS_VERTEX_FORMAT0
//PS_VS_INTERFACE0
&"uniform mat4 uMatProj;\n\
uniform mat4 uMatModelView;\n\
void main() {\n\
	vec4 posw = vec4(a_pos.xyz,1.0);\n\
	vec4 epos = uMatModelView * posw;\n\
	vec3 enorm = (uMatModelView * vec4(a_norm.xyz,0.0)).xyz;\n\
	vec4 spos=uMatProj * epos;\n\
	gl_Position = spos;\n\
	v_pos = posw;\n\
	v_color = a_color;\n\
	v_tex0 = a_tex0;\n\
	v_tex1 = a_pos.xyz;\n\
	v_norm = enorm;\n\
}";


/*
cases:
VSO:
	static scene
	animation,3bone
PS:
	2textures
	3textures
 */

// sanity check debug, checking that the andoir build does this ok..
static g_PS_ConcatForAndroid:&'static str= &"
precision mediump float; \n\
varying	highp vec4 v_pos;\n\
varying	highp vec4 v_color;\n\
varying	highp vec3 v_norm;\n\
varying	highp vec2 v_tex0;\n\
varying	highp vec3 v_tex1;\n\
varying	highp vec4 v_tangent;\n\
varying	highp vec4 v_binormal;\n\
uniform sampler2D uTex0;\n\
uniform sampler2D uTex1;\n\
uniform vec4 uSpecularDir;\n\
uniform float uSpecularPower;\n\
uniform vec4 uSpecularColor;\n\
uniform vec4 uFogColor;\n\
uniform vec4 uFogFalloff;\n\
uniform vec4 uAmbient;\n\
uniform vec4 uDiffuseDX;\n\
uniform vec4 uDiffuseDY;\n\
uniform vec4 uDiffuseDZ;\n\
    \n\
uniform vec4 uLightPos;\n\
uniform vec4 uLightColor;\n\
uniform vec4 uLightFalloff;\n\
vec4 applyFog(vec3 pos, vec4 color){\n\
	return mix(color,uFogColor,  clamp(-uFogFalloff.x-pos.z*uFogFalloff.y,0.0,1.0));\n\
}\n\
vec4 pointlight(vec3 pos, vec3 norm,vec3 lpos, vec4 color, vec4 falloff) {\n\
	vec3 dv=lpos-pos;\n\
	float d2=sqrt(dot(dv,dv));\n\
	float f=clamp( 1.0-(d2/falloff.x),0.0,1.0);\n\
	vec3 lv=normalize(dv);\n\
	return clamp(dot(lv,norm),0.0,1.0) * f*color;\n\
}\n\
void main() { \n\
	float inva=(v_color.w),a=(1.0-v_color.w);\n\
	vec4 t0=texture2D(uTex0, v_tex0);\n\
	vec4 t1=texture2D(uTex1, v_tex0);\n\
	float a0=t0.x*0.4+t0.y*0.6+t0.z*0.25;\n\
	float a1=t1.x*0.4+t1.y*0.6+t1.z*0.25;\n\
	float highlight=max(0.0,dot(v_norm,uSpecularDir.xyz));\n\
		highlight=(highlight*highlight);highlight=highlight*highlight;\n\
	vec4 surfaceColor=mix(t0,t1,v_color.w);\n\
	vec4 surfaceSpec=clamp(4.0*(surfaceColor-vec4(0.5,0.5,0.5,0.0)), vec4(0.0,0.0,0.0,0.0),vec4(1.0,1.0,1.0,1.0));\n\
	vec4 spec=highlight*uSpecularColor*surfaceSpec;\n\
	vec4 diff=uAmbient+v_norm.x*uDiffuseDX+v_norm.y*uDiffuseDY+v_norm.z*uDiffuseDZ;\n\
	float lx=0.5,ly=0.5;\n\
	diff+=pointlight(v_pos.xyz,v_norm.xyz, vec3(lx,ly,-1.0),		vec4(1.0,0.0,0.0,0.0),vec4(1.0,0.0,0.0,0.0));\n\
	diff+=pointlight(v_pos.xyz,v_norm.xyz, vec3(lx,-ly,-1.0), 	vec4(0.0,1.0,0.0,0.0),vec4(1.0,0.0,0.0,0.0));\n\
	diff+=pointlight(v_pos.xyz,v_norm.xyz, vec3(-lx,-ly,-1.0),	vec4(0.0,0.0,1.0,0.0),vec4(1.0,0.0,0.0,0.0));\n\
	diff+=pointlight(v_pos.xyz,v_norm.xyz, vec3(-lx,ly,-1.0), 	vec4(0.5,0.0,0.5,0.0),vec4(1.0,0.0,0.0,0.0));\n\
//	gl_FragColor =applyFog(v_pos.xyz,surfaceColor*diff*vec4(v_color.xyz,0.0)*2.0+spec);\n\
	gl_FragColor =vec4(v_norm.xyz,0.0)*0.5+vec4(0.5,0.5,0.5,1.0);\n\
}";


static g_PS_Alpha:&'static str= &
//SHADER_PREFIX
//PS_VS_INTERFACE0
"uniform sampler2D uTex0;\n\
uniform sampler2D uTex1;\n\
uniform vec4 uSpecularDir;\n\
uniform float uSpecularPower;\n\
uniform vec4 uSpecularColor;\n\
uniform vec4 uFogColor;\n\
uniform vec4 uFogFalloff;\n\
uniform vec4 uAmbient;\n\
uniform vec4 uDiffuseDX;\n\
uniform vec4 uDiffuseDY;\n\
uniform vec4 uDiffuseDZ;\n\
    \n\
uniform vec4 uLightPos;\n\
uniform vec4 uLightColor;\n\
uniform vec4 uLightFalloff;\n\
vec4 applyFog(vec3 pos, vec4 color){\n\
	return mix(color,uFogColor,  clamp(-uFogFalloff.x-pos.z*uFogFalloff.y,0.0,1.0));\n\
}\n\
vec4 pointlight(vec3 pos, vec3 norm,vec3 lpos, vec4 color, vec4 falloff) {\n\
	vec3 dv=lpos-pos;\n\
	float d2=sqrt(dot(dv,dv));\n\
	float f=clamp( 1.0-(d2/falloff.x),0.0,1.0);\n\
	vec3 lv=normalize(dv);\n\
	return clamp(dot(lv,norm),0.0,1.0) * f*color;\n\
}\n\
void main() { \n\
	float inva=(v_color.w),a=(1.0-v_color.w);\n\
	vec4 t0=texture2D(uTex0, v_tex0);\n\
	vec4 t1=texture2D(uTex1, v_tex0);\n\
	float a0=t0.x*0.4+t0.y*0.6+t0.z*0.25;\n\
	float a1=t1.x*0.4+t1.y*0.6+t1.z*0.25;\n\
	float highlight=max(0.0,dot(v_norm,uSpecularDir.xyz));\n\
		highlight=(highlight*highlight);highlight=highlight*highlight;\n\
	vec4 surfaceColor=mix(t0,t1,v_color.w);\n\
	vec4 surfaceSpec=clamp(4.0*(surfaceColor-vec4(0.5,0.5,0.5,0.0)), vec4(0.0,0.0,0.0,0.0),vec4(1.0,1.0,1.0,1.0));\n\
	vec4 spec=highlight*uSpecularColor*surfaceSpec;\n\
	vec4 diff=uAmbient+v_norm.x*uDiffuseDX+v_norm.y*uDiffuseDY+v_norm.z*uDiffuseDZ;\n\
	float lx=0.5,ly=0.5;\n\
	diff+=pointlight(v_pos.xyz,v_norm.xyz, vec3(lx,ly,-1.0),		vec4(1.0,0.0,0.0,0.0),vec4(1.0,0.0,0.0,0.0));\n\
	diff+=pointlight(v_pos.xyz,v_norm.xyz, vec3(lx,-ly,-1.0), 	vec4(0.0,1.0,0.0,0.0),vec4(1.0,0.0,0.0,0.0));\n\
	diff+=pointlight(v_pos.xyz,v_norm.xyz, vec3(-lx,-ly,-1.0),	vec4(0.0,0.0,1.0,0.0),vec4(1.0,0.0,0.0,0.0));\n\
	diff+=pointlight(v_pos.xyz,v_norm.xyz, vec3(-lx,ly,-1.0), 	vec4(0.5,0.0,0.5,0.0),vec4(1.0,0.0,0.0,0.0));\n\
//	gl_FragColor =applyFog(v_pos.xyz,surfaceColor*diff*vec4(v_color.xyz,0.0)*2.0+spec);\n\
	gl_FragColor =vec4(v_norm.xyz,0.0)*0.5+vec4(0.5,0.5,0.5,1.0);\n\
}";

// debug shader
static g_PS_Add:&'static str= &
//SHADER_PREFIX
//PS_VS_INTERFACE0
"uniform sampler2D s_Tex0;\n\
uniform sampler2D s_Tex1;\n\
uniform vec4 uSpecularDir;\n\
uniform float uSpecularPower;\n\
uniform vec4 uSpecularColor;\n\
uniform vec4 uAmbient;\n\
uniform vec4 uDiffuseDX;\n\
uniform vec4 uDiffuseDY;\n\
uniform vec4 uDiffuseDZ;\n\
void main() { \n\
	float inva=(v_color.w),a=(1.0-v_color.w);\n\
	vec4 t0=texture2D(s_Tex0, v_tex0);\n\
	vec4 t1=texture2D(s_Tex1, v_tex0);\n\
	float a0=t0.x*0.4+t0.y*0.6+t0.z*0.25;\n\
	float a1=t1.x*0.4+t1.y*0.6+t1.z*0.25;\n\
	float highlight=max(0.0,dot(v_norm,uSpecularDir.xyz));\n\
	highlight=(highlight*highlight);highlight=highlight*highlight;\n\
	vec4 surfaceColor=t0+(t1-vec4(0.5f,0.5f,0.5f,0.0f))*v_color.w;\n\
	vec4 surfaceSpec=clamp(4.0*(surfaceColor-Vec4(0.5,0.5,0.5,0.0)), vec4(0.0,0.0,0.0,0.0),vec4(1.0,1.0,1.0,1.0));\n\
	vec4 spec=highlight*uSpecularColor*surfaceSpec;\n\
	vec4 diff=uAmbient+vso_norm.x*uDiffuseDX+vso_norm.y*uDiffuseDY+vso_norm.z*uDiffuseDZ;\n\
	gl_FragColor =surfaceColor*diff*vec4(v_color.xyz,0.0)*2.0+spec;\n\
}";

static g_PS_Flat:&'static str=&"\
void main {\n\
	gl_FragColor= mediump vec4(0.0, 1.0, 0.0, 1.0);\n\
}\n\
";

static g_PS_MinimumDebugAndroidCompiler:&'static str= &"\
precision mediump float; \n\
void main() \n\
{ \n\
 gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0); \n\
} \n\
";

static g_PS_Tex3_AlphaMul:&'static str=&"\
uniform sampler2D s_tex0;\n\
uniform sampler2D s_tex1;\n\
uniform sampler2D s_tex2;\n\
uniform vec4 uSpecularDir;\n\
uniform float uSpecularPower;\n\
uniform vec4 uSpecularColor;\n\
uniform vec4 uAmbient;\n\
uniform vec4 uDiffuseDX;\n\
uniform vec4 uDiffuseDY;\n\
uniform vec4 uDiffuseDZ;\n\
void main() { \n\
	float inva=(v_color.w),a=(1.0-v_color.w);\n\
	vec4 t0=texture2D(s_Tex0, v_tex0);\n\
	vec4 t1=texture2D(s_Tex1, v_tex0);\n\
	float a0=t0.x*0.4+t0.y*0.6+t0.z*0.25;\n\
	float a1=t1.x*0.4+t1.y*0.6+t1.z*0.25;\n\
	float highlight=max(0.0,dot(v_norm,uSpecularDir.xyz));\n\
	highlight=(highlight*highlight);highlight=highlight*highlight;\n\
	vec4 surfaceColor=mix(t0,t1, v_color.w*t1.a);\n\
	vec4 surfaceSpec=clamp(4.0*(surfaceColor-Vec4(0.5,0.5,0.5,0.0)), vec4(0.0,0.0,0.0,0.0),vec4(1.0,1.0,1.0,1.0));\n\
	vec4 spec=highlight*uSpecularColor*surfaceSpec;\n\
	vec4 diff=uAmbient+vso_norm.x*uDiffuseDX+vso_norm.y*uDiffuseDY+vso_norm.z*uDiffuseDZ;\n\
	gl_FragColor =surfaceColor*diff*Vec4(v_color.xyz,0.0)*2.0+spec;\n\
}\
";


struct UniformTable {
	mat_proj:UniformIndex,
	mat_model_view:UniformIndex,
	mat_model_view_proj:UniformIndex,
	mat_color:UniformIndex,
	mat_env_map:UniformIndex,
	tex0:UniformIndex,
	tex1:UniformIndex,
	cam_in_obj:UniformIndex,
	ambient:UniformIndex,
	diffuse_dx:UniformIndex,
	diffuse_dy:UniformIndex,
	diffuse_dz:UniformIndex,
	specular_color:UniformIndex,
	specular_dir:UniformIndex,
	sky_dir:UniformIndex,
	test_vec_4:UniformIndex,
	fog_color:UniformIndex,
	fog_falloff:UniformIndex,
	light0_pos_r:UniformIndex,
	light0_color:UniformIndex,
}
static g_uniform_table_empty:UniformTable= UniformTable{
	mat_proj:-1,
	mat_model_view:-1,
	mat_model_view_proj:-1,
	mat_color:-1,
	mat_env_map:-1,
	tex0:-1,
	tex1:-1,
	cam_in_obj:-1,
	ambient:-1,
	diffuse_dx:-1,
	diffuse_dy:-1,
	diffuse_dz:-1,
	specular_color:-1,
	specular_dir:-1,
	sky_dir:-1,
	test_vec_4:-1,
	fog_color:-1,
	fog_falloff:-1,
	light0_pos_r:-1,
	light0_color:-1,
};

//map_shader_params(VertexAttr* vsa,UniformTable* su,int prog)
fn map_shader_params(prog:GLuint)->(VertexAttr,UniformTable)
{
	// read attrib back from shader
	// at the minute we've preset these from VAI indices,
	// but leave this path for datadriven approch later

	// todo: rustic macro
	unsafe {
		(
			VertexAttr{
				pos: get_attrib_location(prog, &"a_pos"),
				color: get_attrib_location(prog, &"a_color"),
				norm: get_attrib_location(prog, &"a_norm"),
				tex0: get_attrib_location(prog, &"a_tex0"),
				tex1: get_attrib_location(prog, &"a_tex1"),
				joints: get_attrib_location(prog, &"a_joints"),
				weights: get_attrib_location(prog, &"a_weights"),
				tangent: get_attrib_location(prog, &"a_binormal"),
				binormal: get_attrib_location(prog, &"a_tangent")

			},
			UniformTable{
				mat_proj:get_uniform_location(prog,&"uMatProj"),
				mat_model_view:get_uniform_location(prog,&"uMatModelView"),
				specular_color:get_uniform_location(prog,&"uSpecularColor"),
				specular_dir:get_uniform_location(prog,&"uSpecularDir"),
				ambient:get_uniform_location(prog,&"uAmbient"),
				diffuse_dx:get_uniform_location(prog,&"uDiffuseDX"),
				diffuse_dy:get_uniform_location(prog,&"uDiffuseDY"),
				diffuse_dz:get_uniform_location(prog,&"uDiffuseDZ"),
				fog_color:get_uniform_location(prog,&"uFogColor"),
				fog_falloff:get_uniform_location(prog,&"uFogFalloff"),
				light0_pos_r:get_uniform_location(prog,&"uLight0PosR"),
				light0_color:get_uniform_location(prog,&"uLight0Color"),
				..g_uniform_table_empty
			}
		)
	}	
}

#[cfg(target_os = "android")]
fn get_shader_prefix(is_ps:int)->&'static str {
	if is_ps==0 {vertex_shader_prefix_gles} else {pixel_shader_prefix_gles}
}

#[cfg(not(target_os = "android"))]
fn get_shader_prefix(is_ps:int)->&'static str {
	shader_prefix_desktop
}

fn	create_shaders()
{
	
	unsafe {
		logi!("create shaders");
		let (vsh,psh,prg)=create_shader_program( 
//						&~[g_PS_ConcatForAndroid], // this works!
//						&~[g_PS_MinimumDebugAndroidCompiler],
//						&~[get_shader_prefix(1),ps_vs_interface0,g_PS_Flat], //PS_Alpha
						&~[get_shader_prefix(1),ps_vs_interface0,g_PS_Alpha],
						&~[get_shader_prefix(0), ps_vertex_format0, ps_vs_interface0, g_VS_Persp]);
		g_vertex_shader=vsh;
		g_pixel_shader=psh;	
		g_shader_program=prg;
		
		let (vs, su)=map_shader_params(g_shader_program);
		println!("vs={:?}",vs);
		println!("su={:?}",su);
		g_vertex_shader_attrib=vs;
		g_shader_uniforms=su;
	}
}

fn generate_torus_vertex(ij:uint, num_u:uint, num_v:uint)->TestVertex {
	let pi=3.14159265f32;
	let tau=pi*2.0f32;
	let (i,j)=num::div_rem(ij, num_u);
	let fi=i.to_f32().unwrap_or(0.0) * (1.0 / num_u.to_f32().unwrap_or(0.0));
	let fj=j.to_f32().unwrap_or(0.0) * (1.0 / num_v.to_f32().unwrap_or(0.0));

	let rx=0.125f32;
	let ry=rx*0.33f32;
	let pi=3.14159265f32;
	let tau=pi*2.0f32;
	let (sx,cx)=num::sin_cos(fi*tau);
	let (sy,cy)=num::sin_cos(fj*tau);

	TestVertex{
		pos:[(rx+sy*ry)*cx, (rx+sy*ry)*sx, ry*cy],
		color:[1.0,1.0,1.0,1.0],
		norm:[sy*cx, sy*sx, cy],
		tex0:[fi*8.0, fj*2.0],
	}	
}

unsafe fn create_buffer(size:GLsizei, data:*c_void, buffer_type:GLenum)->GLuint {
	let mut id:GLuint=0;
	glGenBuffers(1,&mut id);
	glBindBuffer(buffer_type,id);
	
	glBufferData(buffer_type, size, data, GL_STATIC_DRAW);
	// error..
	glBindBuffer(buffer_type,0);
	id
}

unsafe fn create_vertex_buffer_from_ptr(size:GLsizei, data:*c_void)->GLuint {
	create_buffer(size,data,GL_ARRAY_BUFFER)
}
unsafe fn create_index_buffer_from_ptr(size:GLsizei, data:*c_void)->GLuint {
	create_buffer(size,data,GL_ELEMENT_ARRAY_BUFFER)
}
unsafe fn create_vertex_buffer<T>(data:&~[T])->GLuint {
	create_buffer(data.len()as GLsizei *mem::size_of::<T>() as GLsizei, as_void_ptr(&data[0]), GL_ARRAY_BUFFER)
}
unsafe fn create_index_buffer<T>(data:&~[T])->GLuint {
	create_buffer(data.len()as GLsizei *mem::size_of::<T>() as GLsizei, as_void_ptr(&data[0]), GL_ELEMENT_ARRAY_BUFFER)
}


impl Mesh {
	/// create a grid mesh , TODO - take a vertex generator
	fn new_torus((num_u,num_v):(uint,uint))->Mesh
	{
		// TODO: 2d fill array. from_fn_2f(numi,numj, &|..|->..)
		let strip_indices = (num_u+1)*2 +2;
		let num_indices=(num_v)*strip_indices;
		let indices=vec::from_fn(num_indices,
			|ij|->GLuint{
				let (j,i1)=num::div_rem(ij, strip_indices);
				let i2=cmp::min(cmp::max(i1-1,0),num_u*2+1); // first,last value is repeated - degen tri.
				let (i,dj)=num::div_rem(i2,2);	// i hope that inlines to >> &
//				println!("indexx{:?} i={:?} j={:?}", ij, i, j);
				(((j+dj)%num_v)*num_u+(i % num_u)) as GLuint
			}
		);
		
		let num_vertices=num_u*num_v;
		let vertices=vec::from_fn(num_vertices,|i|generate_torus_vertex(i,num_u,num_v));

 		unsafe {
			Mesh{
				num_vertices:num_vertices as GLuint,
				num_indices:num_indices as GLuint,
				vertex_size: mem::size_of_val(&vertices[0]) as GLsizei,
				vbo: create_vertex_buffer(&vertices),
				ibo: create_index_buffer(&indices)
			}
		}

	}
}


//extern void	TestGl_Idle();

//float	angle=0.f;
//GridMesh*	g_pGridMesh;
static mut g_grid_mesh:Mesh=Mesh{
	num_vertices:0,
	num_indices:0,
	vbo:-1,
	ibo:-1,
	vertex_size:0
};


type UniformIndex=GLint;


impl Mesh {
	fn	render_mesh_from_buffer(&self)
	{
		unsafe {
			use shadertest::TestVertex;

			let	client_state:[GLenum,..3]=[GL_VERTEX_ARRAY,GL_COLOR_ARRAY,GL_TEXTURE_COORD_ARRAY];
			for &x in client_state.iter() {glEnableClientState(x);};

			glBindBuffer(GL_ARRAY_BUFFER, self.vbo);
			glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.ibo);

			let baseVertex=0 as *TestVertex;
			let	stride=mem::size_of_val(&*baseVertex) as GLsizei;

			glVertexPointer(3, GL_FLOAT, stride,  0 as *c_void);//(&(*baseVertex).pos[0]) as *f32 as *c_void);
			glColorPointer(4,GL_FLOAT, stride, 12 as *c_void);//(&(*baseVertex).color[0]) as *f32 as *c_void);
			glTexCoordPointer(2, GL_FLOAT, stride, (12+16) as *c_void);//(&(*baseVertex).tex0[0]) as *f32 as *c_void);
			glDrawElements(GL_TRIANGLE_STRIP, self.num_indices as GLsizei, GL_UNSIGNED_INT,0 as *c_void);

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
		glUniform4fv(loc, 1, &pvalue.x as *GLfloat);
	}
}

unsafe fn as_void_ptr<T>(ptr:&T)->*c_void {
	ptr as *T as *c_void
}

//Vec4 g_FogColor=Vec4::<f32>::new(0.25,0.5,0.5,1.0);

//vertex_layout!(struct MyVertex{pos:[f32=GL_FLOAT,..3] = 0 })


static g_fog_color:Vec4<f32> =Vec4{x:0.25,y:0.5,z:0.5,w:1.0};
impl Mesh {
	unsafe fn	render_mesh_shader(&self)  {
		
		let clientState:[GLenum,..3]=[GL_VERTEX_ARRAY,GL_COLOR_ARRAY,GL_TEXTURE_COORD_ARRAY];

		glBindBuffer(GL_ARRAY_BUFFER, self.vbo);
		glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.ibo);

		let vsa=&g_vertex_shader_attrib;

//		glEnableVertexAttribArray(VAI_pos as GLuint);
//		glEnableVertexAttribArray(VAI_color as GLuint);
//		glEnableVertexAttribArray(VAI_tex0 as GLuint);
//		glEnableVertexAttribArray(VAI_norm as GLuint);

		safe_set_uniform1i(g_shader_uniforms.tex0, 0);
		safe_set_uniform1i(g_shader_uniforms.tex1, 1);
		safe_set_uniform(g_shader_uniforms.specular_dir, &Vec4::new(0.032,0.707f32,0.707f32,0.0f32));
		safe_set_uniform(g_shader_uniforms.specular_color, &Vec4::new(1.0f32,0.75f32,0.5f32,0.0f32));
		safe_set_uniform(g_shader_uniforms.ambient, &Vec4::new(0.25f32,0.25f32,0.25f32,1.0f32));
		safe_set_uniform(g_shader_uniforms.diffuse_dx, &Vec4::new(0.0f32,0.0f32,0.25f32,1.0f32));
		safe_set_uniform(g_shader_uniforms.diffuse_dy, &Vec4::new(0.5f32,0.5f32,0.5f32,1.0f32));
		safe_set_uniform(g_shader_uniforms.diffuse_dz, &Vec4::new(0.25f32,0.0f32,0.0f32,1.0f32));
		safe_set_uniform(g_shader_uniforms.fog_color, &g_fog_color);
		safe_set_uniform(g_shader_uniforms.fog_falloff, &Vec4::new(0.5f32,0.25f32,0.0f32,0.0f32));

		glActiveTexture(GL_TEXTURE0+0);
		glBindTexture(GL_TEXTURE_2D, g_textures[2]);
		glActiveTexture(GL_TEXTURE0+1);
		glBindTexture(GL_TEXTURE_2D, g_textures[1]);

//		glVertexAttribPointer(VAI_pos as GLuint,	3,GL_FLOAT, GL_FALSE, stride, &((*baseVertex).pos[0]) as *f32 as *c_void);
		// todo: Rustic struct element offset macro
		let baseVertex=0 as *TestVertex;

		TestVertex::set_vertex_attrib();

/*
		glVertexAttribPointer(VAI_pos as GLuint,	3,GL_FLOAT, GL_FALSE, self.vertex_size, as_void_ptr(&(*baseVertex).pos)); 
		glVertexAttribPointer(VAI_color as GLuint,	4,GL_FLOAT, GL_FALSE, self.vertex_size, as_void_ptr(&(*baseVertex).color)); 
		glVertexAttribPointer(VAI_tex0 as GLuint,	2,GL_FLOAT, GL_FALSE, self.vertex_size, as_void_ptr(&(*baseVertex).tex0)); 
		glVertexAttribPointer(VAI_norm as GLuint,	3,GL_FLOAT, GL_FALSE, self.vertex_size, as_void_ptr(&(*baseVertex).norm)); 
*/
		glDrawElements(GL_TRIANGLE_STRIP, self.num_indices as GLsizei, GL_UNSIGNED_INT,0 as *c_void);
	}
}

static mut g_angle:f32=0.0f32;
static mut g_frame:int=0;

static g_num_torus:int = 256;
/// render a load of meshes in a lissajous

pub fn	render_no_swap() 
{
	//logw("render noswap");

	unsafe {

		assert!(g_resources_init==true)		//logi!("render_no_swap"); // once..
		g_angle+=0.0025f32;

//		glDrawBuffer(GL_BACK);
		glClearColor(g_fog_color.x+sin(g_angle*2.0),g_fog_color.y,g_fog_color.z,g_fog_color.w);

		glClearDepthf(1.0f32);
		glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);
		glEnable(GL_DEPTH_TEST);
		glDepthMask(GL_TRUE);
		glDepthFunc(GL_LEQUAL);

		glEnable(GL_CULL_FACE);
//		glFrontFace(GL_CCW);
		let matI = Matrix4::<Vec4>::identity();
		let matP = matrix::projection_frustum(-0.5f32,0.5f32,-0.5f32,0.5f32, 90.0f32, 1.0f32, 0.5f32,5.0f32);

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

		for i in range(0,g_num_torus) {

			let matT = matrix::translate_xyz(
				num::cos(a0)*r0+num::cos(a3)*r1, 
				num::cos(a1)*r0+num::cos(a4)*r1, 
				num::cos(a2)*r0+num::cos(a5)*r1 -2.0*r0);

			let rot_x = matrix::rotate_x(a0);
			let rot_y = matrix::rotate_x(a1*0.245f32);
			let rot_xy=rot_x.mul_matrix(&rot_y);
			let rot_trans = matT.mul_matrix(&rot_xy);
	
			let matMV = matT;	// toodo - combine rotation...
			//io::println(format!("{:?}", g_shader_program));

			glUseProgram(g_shader_program);
			glUniformMatrix4fvARB(g_shader_uniforms.mat_proj, 1,  GL_FALSE, &matP.ax.x);
			glUniformMatrix4fvARB(g_shader_uniforms.mat_model_view, 1, GL_FALSE, &rot_trans.ax.x);

			g_grid_mesh.render_mesh_shader();

			a0+=da0;a1+=da1;a2+=da2;a3+=da3;a4+=da4;a5+=da5;
		}

		g_frame+=1;
	}
}

fn idle() 
{
	unsafe {
		glutPostRedisplay();
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
		let buffer:~[u32] = vec::from_fn(usize*vsize,|index|{
				let (i,j)=num::div_rem(index,usize);
				(i+j*256+255*256*256) as u32
			});
		for i in range(0 as GLint,8 as GLint) {
			glTexImage2D(GL_TEXTURE_2D, i, GL_RGB as GLint, usize as GLint,vsize as GLint, 0, GL_RGB, GL_UNSIGNED_BYTE, as_void_ptr(&buffer[0]));
		}
		glBindTexture(GL_TEXTURE_2D,0);
	
//		g_textures[1] = get_texture(&"data/rocktile.tga");
//		g_textures[4] = get_texture(&"data/pebbles_texture.tga");
//		g_textures[3] = get_texture(&"data/grass.tga");
//		g_textures[2] = get_texture(&"data/cliffs.tga");
	}
}


// todo - turn this into lazy init.
pub fn create_static_resources() {
	::std::io::println("dummy create, its done lazy now");
}
static mut g_resources_init:bool=false;
pub fn create_resources() {
	
	unsafe {
		logi!("shadertest Create Resources \n");
		create_shaders();
		create_textures();
		g_grid_mesh = Mesh::new_torus((16,16)); //new GridMesh(16,16);
		g_resources_init=true;
	}
}
pub fn destroy_resources() {
	unsafe {
		g_resources_init=false;
	}
}

// standalone draw inner loop
pub fn render_and_swap() {
	render_no_swap();
	unsafe {
		glFlush();
		glutSwapBuffers();
	}
}

// desktop main.

#[cfg(not(target_os = "android"))]
pub fn shadertest_main()
{
	unsafe {
		let mut argc:c_int=0;
		let argv:~[*c_char]=~[];
		glutInit((&mut argc) as *mut c_int,0 as **c_char );

		::macros::test();

		glutInitDisplayMode(GLUT_DEPTH | GLUT_DOUBLE | GLUT_RGBA);
		let win=verify!(glutCreateWindow(c_str("Rust ShaderTest")) isnt 0);
//		glewInit(); //TODO- where the hell is glewInit. -lGLEW isn't found
		create_resources();
		glDrawBuffer(GL_BACK);
		glutReshapeWindow(1024,1024);
		glutDisplayFunc(render_and_swap as *u8);
		glutIdleFunc(idle as *u8);
		glEnable(GL_DEPTH_TEST);

//		logi!("{}",g_grid_mesh);

		glutMainLoop();
	}
}
