use super::macros::*;
use super::render::*;
use super::vecmath::*;
use super::formats::*;
use super::gl::*;
use super::ut::*;

def_vertex_attrib!{
	enum VertexAttrib {
		a_pos,a_color,a_norm,a_tex0,a_tex1
	}
}

def_vertex_format!{
	struct Vertex {
		pos:	[f32(GL_FLOAT),..3](a_pos),
		color:	[f32(GL_FLOAT),..4](a_color),
		norm:	[f32(GL_FLOAT),..3](a_norm),
		tex0:	[f32(GL_FLOAT),..2](a_tex0)
	}
}



