use super::macros::*;
use super::render::*;
use super::gl::*;
use super::vertex::*;

pub static mut g_textures:[GLuint,..5]=[0,..5];
pub static mut g_shader_program:GLuint=-1;
pub static mut g_pixel_shader:GLuint=-1;
pub static mut g_vertex_shader:GLuint=-1;


//#define PS_VS_INTERFACE0
static ps_vs_interface0:&'static str="\n\
varying	highp vec4 v_pos;\n\
varying	highp vec4 v_color;\n\
varying	highp vec3 v_norm;\n\
varying	highp vec2 v_tex0;\n\
varying	highp vec3 v_tex1;\n\
varying	highp vec4 v_tangent;\n\
varying	highp vec4 v_binormal;\n\
\n";


//#define PS_VERTEX_FORMAT0
static ps_vertex_format0:&'static str="\n\
attribute vec3 a_pos;\n\
attribute vec2 a_tex0;\n\
attribute vec4 a_color;\n\
attribute vec3 a_norm;\n\
\n";

static g_VS_Default:&'static str="\n\
uniform mat4 uMatProj;\n\
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
static g_VS_PassThru:&'static str="\n\
uniform mat4 uMatProj;\n\
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
static g_VS_Translate2d:&'static str="\n\
uniform mat4 uMatProj;\n\
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
static g_VS_Persp:&'static str="\n\
uniform mat4 uMatProj;\n\
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
static g_PS_ConcatForAndroid:&'static str= "
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
	gl_FragColor =applyFog(v_pos.xyz,surfaceColor*diff*vec4(v_color.xyz,0.0)*2.0+spec);\n\
//	gl_FragColor =vec4(v_norm.xyz,0.0)*0.5+vec4(0.5,0.5,0.5,1.0);\n\
}";


static g_PS_Alpha:&'static str= 
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
	gl_FragColor =applyFog(v_pos.xyz,surfaceColor*diff*vec4(v_color.xyz,0.0)*2.0+spec);\n\
//	gl_FragColor =vec4(v_norm.xyz,0.0)*0.5+vec4(0.5,0.5,0.5,1.0);\n\
}";

// debug shader
static g_PS_Add:&'static str= 
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

static g_PS_Flat:&'static str="\
void main {\n\
	gl_FragColor= mediump vec4(0.0, 1.0, 0.0, 1.0);\n\
}\n\
";

static g_PS_MinimumDebugAndroidCompiler:&'static str= "\
precision mediump float; \n\
void main() \n\
{ \n\
 gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0); \n\
} \n\
";

static g_PS_Tex3_AlphaMul:&'static str="\
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

pub static mut g_uniform_table:Option<UniformTable> =None;

/// Macro to create a shader Uniform table struct, and populate it by querying a shader program
/// TODO: create lazy state struct, allow user to set shader params in a struct and pass to GL..
/// by embedding the type information here aswell.


def_uniform_table!{ 
	struct UniformTable {
		uMatProj,
		uMatModelView,
		uSpecularColor,
		uSpecularDir,
		uAmbient,
		uDiffuseDX,
		uDiffuseDY,
		uDiffuseDZ,
		uFogColor,
		uFogFalloff,
		uLight0PosR,
		uLight0Color,
		uTex0,
		uTex1
	}
}
//pub type VertexShader=GLuint;
//pub type PixelShader=GLuint;
//pub type ShaderProgram=GLuint;


pub unsafe fn	compile_shader_program(
			pixelShaderSource:&Vec<&str>,
			vertexShaderSource:&Vec<&str>)->(PixelShader,VertexShader,ShaderProgram)
{
	let pixel_shader = create_and_compile_shader(GL_FRAGMENT_SHADER, pixelShaderSource);
	let vertex_shader = create_and_compile_shader(GL_VERTEX_SHADER, vertexShaderSource);

	let prog = create_shader_program(pixel_shader,vertex_shader,|prog|VertexAttrib::bind_attribs(prog));

	(pixel_shader,vertex_shader,prog)
}


pub fn	create_shaders()
{
	
	unsafe {
		logi!("create shaders");
		let (vsh,psh,prg)=compile_shader_program( 
						&vec!{get_shader_prefix(1),ps_vs_interface0,g_PS_Alpha},
						&vec!{get_shader_prefix(0), ps_vertex_format0, ps_vs_interface0, g_VS_Persp});
		g_vertex_shader=vsh;
		g_pixel_shader=psh;	
		g_shader_program=prg;
		
		g_uniform_table = Some(UniformTable::new(g_shader_program));
	}
}
