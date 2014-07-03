#![allow(non_snake_case_functions)]

pub use libc::{c_char,c_void,c_int,c_uint,c_uchar};
pub use r3d::gl_constants::*;
pub use r3d::gl_h_consts::*;
pub use r3d::glut_h_consts::*;

pub type GLenum=uint;
pub type GLboolean=u8;
pub type GLchar = c_char;
pub type GLvoid = c_void;
pub type GLbyte = i8;
pub type GLshort = i16;
pub type GLushort = u16;
pub type GLint= c_int;
pub type GLubyte= u8;
pub type GLuint= c_uint;
pub type GLsizei= c_int;
pub type GLfloat= f32;
pub type GLclampf= f32;
pub type GLdouble = f64;
pub type GLclampd=f64;	/* double precision float in [0,1] */

pub type RustTempCFunc=*u8;

extern
{
	pub fn hello_from_c(a:c_int,b:c_int);
	
	pub fn run_glut();
	pub fn setup_sub();
	pub fn displayCall();
	pub fn displaySub(txt:*c_char);
	
	pub fn glewInit();
	
	pub fn glCompileShader(shader:GLuint);
	pub fn glShaderSource(shader:GLuint, count:GLsizei, string:**GLchar, length:*GLint);
	pub fn glGetShaderInfoLog(shader:GLuint, maxLength:GLsizei, length:*GLsizei, infoLog:*GLchar);
	pub fn glTexParameterf(target:GLenum,pname:GLenum,param:GLfloat);
	pub fn glTexParameteri(target:GLenum,pname:GLenum,param:GLint);

	pub fn glVertexAttribPointer(index:GLuint, size:GLint, _type:GLenum, normalized:GLboolean,stride:GLsizei, pointer:*GLvoid);
	pub fn glVertexAttribIPointer(index:GLuint, size:GLint, _type:GLenum, stride:GLsizei, pointer:*GLvoid);
	pub fn glVertexAttribLPointer(index:GLuint, size:GLint, _type:GLenum, stride:GLsizei, pointer:*GLvoid);
	pub fn glColorPointer(size:GLint, _type:GLenum, stride:GLsizei, pointer:*GLvoid);
	pub fn glVertexPointer(size:GLint, _type:GLenum, stride:GLsizei, pointer:*GLvoid);
	pub fn glNormalPointer(_type:GLenum, stride:GLsizei, pointer:*GLvoid);
	pub fn glTexCoordPointer(size:GLint, _type:GLenum, stride:GLsizei, pointer:*GLvoid);

	
	pub fn glCreateShader(shaderType:GLenum)->GLuint;
	pub fn glCreateProgram()->GLuint;
	pub fn glBindAttribLocation(prog:GLuint, index:GLuint, name:*GLchar);
	pub fn glAttachShader(program:GLuint, shader:GLuint);
	pub fn glLinkProgram(program:GLuint);
	pub fn glBindBuffer(target:GLenum, buffer:GLuint);
	pub fn glEnableVertexAttribArray(index:GLuint);
	pub fn glActiveTexture(texture:GLenum);
	pub fn glDrawElements(mode:GLenum, count:GLsizei, _type:GLenum, indices:*GLvoid);
	pub fn glDrawArrays(mode:GLenum,first:GLsizei, count:GLsizei,);
	pub fn glUseProgram(prog:GLuint);
	pub fn glGetActiveAttrib(program:GLuint, index:GLuint, bufsize:*GLsizei, length:*GLsizei, size:GLint, _type:*GLenum, name:*GLchar);
	pub fn glGetActiveUniform(program:GLuint, index:GLuint, bufsize:GLsizei, length:*GLsizei, size:*GLint, _type:*GLenum, name:*GLchar); 
	pub fn glGetAttachedShaders(program:GLuint, maxCount:GLsizei, count:GLsizei, shaders:GLuint);
	pub fn glGetAttribLocation(prog:GLuint, name:*GLchar)->GLint;
	pub fn glGetUniformfv(prog:GLuint, location:GLint, params:*GLfloat);
	pub fn glGetUniformiv(prog:GLuint, location:GLint, params:*GLint);
	
	pub fn glGetUniformLocation(prog:GLuint, name:*GLchar)->GLint;
	
	pub fn glUniform1f(location:GLint, v0:GLfloat);
	pub fn glUniform2f(location:GLint, v0:GLfloat, v1:GLfloat);
	pub fn glUniform3f(location:GLint, v0:GLfloat, v1:GLfloat, v2:GLfloat);
	pub fn glUniform4f(location:GLint, v0:GLfloat, v1:GLfloat, v2:GLfloat, v3:GLfloat);
	pub fn glUniform1i(location:GLint, v0:GLint);
	pub fn glUniform2i(location:GLint, v0:GLint, v1:GLint);
	pub fn glUniform3i(location:GLint, v0:GLint, v1:GLint, v2:GLint);
	pub fn glUniform4i(location:GLint, v0:GLint, v1:GLint, v2:GLint, v3:GLint);

	pub fn glUniform1fv(location:GLint, count:GLsizei, v0:*GLfloat);
	pub fn glUniform2fv(location:GLint, count:GLsizei, v0:*GLfloat);
	pub fn glUniform3fv(location:GLint, count:GLsizei, v0:*GLfloat);
	pub fn glUniform4fv(location:GLint, count:GLsizei, v0:*GLfloat);
	pub fn glUniform1iv(location:GLint, count:GLsizei, v0:*GLint);
	pub fn glUniform2iv(location:GLint, count:GLsizei, v0:*GLint);
	pub fn glUniform3iv(location:GLint, count:GLsizei, v0:*GLint);
	pub fn glUniform4iv(location:GLint, count:GLsizei, v0:*GLint);

	pub fn glUniformMatrix2fv(location:GLint, count:GLsizei,transpose:GLboolean, v0:*GLfloat);
	pub fn glUniformMatrix3fv(location:GLint, count:GLsizei,transpose:GLboolean, v0:*GLfloat);
	pub fn glUniformMatrix4fv(location:GLint, count:GLsizei,transpose:GLboolean, v0:*GLfloat);
	pub fn glUniformMatrix4fvARB(location:GLint, count:GLsizei,transpose:GLboolean, v0:*GLfloat);

	pub fn glGetProgramiv(program:GLuint, pname:GLenum, params:*GLint);
	pub fn glGetProgramInfoLog(program:GLuint, maxlength:GLsizei, length:*GLsizei, infoLog:*GLchar);
	pub fn glGetShaderiv(shader:GLuint, pname:GLenum, params:*GLint);
	pub fn glIsProgram(program:GLuint)->GLboolean;
	pub fn glShaderBinary(n:GLsizei, shaers:*GLuint, binary_format:GLenum, binary:*c_void, length:*GLsizei);
	pub fn glIsShader(program:GLuint)->GLboolean;
	pub fn glDeleteProgram(program:GLuint);
	pub fn glDeleteShader(program:GLuint);
	
	pub fn glGenBuffers(num:GLsizei, buffers:*mut GLuint);
	pub fn glDeleteBuffers(num:GLsizei, buffers:*GLuint);
	pub fn glIsBuffer(buffer:GLuint)->GLboolean;

	pub fn glLoadIdentity();
	pub fn glLoadMatrixf(mat:*f32);
	pub fn glMatrixMode(e:GLenum);
	pub fn glClear(e:GLenum);

	pub fn glClearColor(r:GLfloat,g:GLfloat,b:GLfloat,a:GLfloat);
	pub fn glAlphaFunc( func:GLenum , _ref:GLclampf );
	pub fn glBlendFunc( sfactor:GLenum, dfactor:GLenum );
	pub fn glLogicOp( opcode:GLenum );
	pub fn glCullFace( mode:GLenum );
	pub fn glDrawBuffer( mode:GLenum  );
	pub fn glReadBuffer( mode:GLenum  );
	pub fn glEnable(e:GLenum);
	pub fn glDisable(e:GLenum);
	pub fn glEnableClientState( cap:GLenum );  /* 1.1 */
	pub fn glDisableClientState( cap:GLenum );  /* 1.1 */
	pub fn glGetFloatv( pname:GLenum, params:*mut GLfloat );

	pub fn glGetIntegerv( pname:GLenum, params:* mut GLint );
	pub fn glRenderMode( mode:GLenum )->GLint;
	pub fn glGetError()->GLenum;
	pub fn glGetString( name:GLenum  )->*GLubyte;
	pub fn glFinish( );
	pub fn glFlush(  );
	pub fn glHint( target:GLenum, mode:GLenum );
	pub fn glClearDepth(  depth:GLclampd );
	pub fn glClearDepthf(  depth:GLfloat );
	pub fn glDepthFunc(  func:GLenum );
	pub fn glDepthMask( flag:GLboolean  );
	pub fn glDepthRange( near_val:GLclampd , far_v:GLclampd );

	pub fn glRasterPos2f( x:GLfloat, y:GLfloat );
	pub fn glRasterPos2i( x:GLint, y:GLint );


	pub fn glOrtho(x0:GLdouble,y0:GLdouble,z0:GLdouble,x1:GLdouble,y1:GLdouble,z1:GLdouble);
	pub fn gluLookAt(posx:GLdouble,posy:GLdouble,posz:GLdouble,atx:GLdouble,aty:GLdouble,atz:GLfloat, upx:GLdouble,upy:GLdouble,upz:GLdouble);
	pub fn glRotatef(angle:GLfloat,axisx:GLfloat,axisy:GLfloat,axisz:GLfloat);	
	pub fn glTranslatef(x:GLfloat,y:GLfloat,z:GLfloat);
	pub fn glScalef(x:GLfloat,y:GLfloat,z:GLfloat);
	
	pub fn gluPerspective(fovy:GLdouble, aspect:GLdouble, znear:GLdouble, zfar:GLdouble);

	pub fn glGenTextures( n:GLsizei, textures:*mut GLuint );

	pub fn glDeleteTextures( n:GLsizei, textures: *GLuint);

	pub fn glBindTexture( target:GLenum , texture:GLuint );
	

	pub fn glTexImage1D( target:GLenum, level:GLint,
                                    internalFormat:GLint ,
                                    width:GLsizei, border:GLint ,
                                    format:GLenum, _type:GLenum,
                                    pixels: *GLvoid );

	pub fn glTexImage2D( target:GLenum, level:GLint,
                                    internalFormat:GLint ,
                                    width:GLsizei, height:GLsizei ,
                                    border:GLint , format:GLenum , _type:GLenum,
                                    pixels: *GLvoid );
	pub fn glGenerateMipmap(target:GLenum);

	pub fn glGetTexImage( target:GLenum, level:GLint,
                                     format:GLenum, _type:GLenum,
                                     pixels:*mut GLvoid );

	pub fn glDrawPixels( width:GLsizei, height:GLsizei,
                                    format:GLenum, _type:GLenum,
                                    pixels:*GLvoid );
	pub fn glReadPixels( x:GLint,y: GLint,
                                    width:GLsizei, height:GLsizei,
                                    format:GLenum, _type:GLenum,
                                    pixels:*mut GLvoid );

	pub fn glTexEnvf( target:GLenum, pname:GLenum , param:GLfloat );
	pub fn glTexEnvi( target:GLenum, pname:GLenum, param :GLint );

/* 1.1 functions */

	pub fn glBegin(mode:GLenum);
	pub fn glEnd();
	pub fn glColor3f(r:GLfloat,g:GLfloat,b:GLfloat);
	pub fn glColor4f(r:GLfloat,g:GLfloat,b:GLfloat,a:GLfloat);
	pub fn glVertex3f(x:GLfloat,y:GLfloat,z:GLfloat);
	pub fn glTexCoord2f( s:GLfloat , t:GLfloat );
	pub fn glNormal3f(x:GLfloat,y:GLfloat,z:GLfloat);
	pub fn glVertex4f(x:GLfloat,y:GLfloat,z:GLfloat);


	pub fn glBufferData(target:GLenum, size:GLsizei, data:*GLvoid, usage:GLenum);

	// convieniences in C
	pub fn glLoadMatrixAt(mat:*f32, e: GLenum);
	pub fn glCompileShaderSources(sh:GLint, src0:*c_char,opt_src1:*c_char,opt_src2:*c_char,opt_src3:*c_char, buffersize:GLsizei, err_text:*c_char  )->GLint;
	pub fn glSetTexutreLayer(l:GLint, texid:GLint);

	pub fn glColor4fv(_:&(f32,f32,f32,f32));
	pub fn glVertex4fv(_:&(f32,f32,f32,f32));
	pub fn glVertex3fv(_:&(f32,f32,f32));
	pub fn glNormal3fv(_:&(f32,f32,f32));
	pub fn glTexCoord2fv(_:&(f32,f32));
	pub fn glMultiTexCoord2fv(_:&(f32,f32));
}


pub fn glDrawLine(v0:&(f32,f32,f32),v1:&(f32,f32,f32), c:u32) {
	unsafe {glBegin(GL_LINES); glVertex3fv(v0);glVertex3fv(v1); glEnd();}
}

