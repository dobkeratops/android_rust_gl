#include <jni.h>
#include <errno.h>
#include <android/log.h>

#include <EGL/egl.h>
#include <GLES/gl.h>

// from my c++ project.
// define OPENGL_ES 1
#include <GLES2/gl2.h>
#include <GLES2/gl2ext.h>


#define LOGI(...) ((void)__android_log_print(ANDROID_LOG_INFO, "rust", __VA_ARGS__))
#define LOGW(...) ((void)__android_log_print(ANDROID_LOG_WARN, "rust", __VA_ARGS__))

void android_log_print(int level, const char* txt) {
	((void)__android_log_print( level, "rust","%s",  txt));

}

// todo..
#define WARN_UNIMPLEMENTED LOGW("%s unimplemented", __function__)

void glutInit(int argc, char** argv){
}

void glutInitWindowSize(GLint x, GLint y) {
}
void glutInitDisplayMode(GLint a) {
}
void glutInitWindowPosition(GLint x,GLint y) {
}

void glutSetKeyRepeat(GLint a) {
}

GLint glutCreateWindow(const char * a) {
	return	0;
}
void glutPopWindow() {
}

void glutKeyboardFunc(void *a ) {
}
void glutSpecialFunc(void *a) {
}
void glutSpecialUpFunc(void *a) {
}
void glutPassiveMotionFunc(void *a) {
}
void glutMouseFunc(void* f) {
}
void glutMotionFunct(void* f) {
}
void glutTabletMotionFunc(void* f) {
}
void glutMotionFunc(void* f) {
}
void glutTabletButtonFunc(void* f) {
}
void glutMainLoopEvent() {
}
void glutKeyboardUpFunc(void* f) {
}
GLint g_uMatProj;
GLint g_uMatModelView;

const char* g_FixedFunctionVS=
"#version 100			\n"
"precision highp float;	\n"
"varying	highp vec4 v_pos;\n"
"varying	highp vec4 v_color;\n"
"varying	highp vec3 v_norm;\n"
"varying	highp vec2 v_tex0;\n"
"varying	highp vec2 v_tex1;\n"
"attribute vec4 a_pos;\n"
"attribute vec2 a_tex0;\n"
"attribute vec2 a_tex1;\n"
"attribute vec4 a_color;\n"
"attribute vec3 a_norm;\n"
"uniform mat4 uMatProj;\n"
"uniform mat4 uMatModelView;\n"
"void main() {;\n"
"	vec4 posw = a_pos;\n"
"	vec4 epos = uMatModelView * posw;\n"
"	vec3 enorm = (uMatModelView * vec4(a_norm.xyz,0.0)).xyz;\n"
"	vec4 spos=uMatProj * epos;\n"
"	gl_Position = spos;\n"
"	v_pos = epos;\n"
"	v_color = a_color;\n"
"	v_tex0 = a_tex0;\n"
"	v_tex1 = a_tex1;\n"
"	v_norm = enorm;\n"
"}";


const char* g_FixedFunctionPS=
"precision mediump float;\n"
"varying	highp vec4 v_pos;\n"
"varying	highp vec4 v_color;\n"
"varying	highp vec3 v_norm;\n"
"varying	highp vec2 v_tex0;\n"
"varying	highp vec2 v_tex1;\n"
"uniform sampler2D s_Tex0;\n"
"uniform sampler2D s_Tex1;\n"
"void main() { \n"
"	float inva=(v_color.w),a=(1.0-v_color.w);\n"
"	vec4 t0=texture2D(s_Tex0, v_tex0);\n"
"	vec4 t1=texture2D(s_Tex1, v_tex1);\n"
"	gl_FragColor =vec4(0.0,1.0,0.0,1.0);\n"
"}\n";




#define MAX_VERTICES 8192
struct TmpVertex {
	GLfloat x,y,z,w, u0,v0,u1,v1, r,g,b,a,  nx,ny,nz;
};
struct GlEmuState {
	GLint prim;
	int	index;
	GLint	vbo;
	GLint	fixed_function_prog;
	GLint	fixed_function_vs;
	GLint	fixed_function_ps;
	struct TmpVertex vertices[MAX_VERTICES];
	float mat_proj[16];
	float mat_model_view[16];
	GLenum	matrix_mode;
};
struct GlEmuState g_State=(struct GlEmuState){.prim=0,.index=0,.vbo=0,.fixed_function_prog=0};

float* GetMatrix()  {
	if (g_State.matrix_mode==GL_PROJECTION) {
		return g_State.mat_proj;
	} else return g_State.mat_model_view;

}

void ShaderShowError(GLint sh) {
	GLint status;
	glGetShaderiv(sh,GL_COMPILE_STATUS,&status);
	LOGW("compile  result for shader %d =%d",sh, status);
	if (status==GL_FALSE){
		char err[1024];size_t len;
		glGetShaderInfoLog(sh,1024,&len,err);
		LOGW("%s",err);
	}
}
void ShaderCompileSource(GLint sh, const char* src) {
	const char* psource[1]={src};
	glShaderSource(sh, 1,psource,0);
	glCompileShader(sh);
	ShaderShowError(sh);
}
GLint glCompileShaderSources(GLint sh, const char* s0,const char* s1, const char* s2, const char* s3, GLsizei buffersize, char* errTextOut) {
	const char* psource[4]={s0,s1,s2,s3};
	int numSrc=0; if (s0) numSrc++; if (s1) numSrc++; if (s2) numSrc++; if (s3) numSrc++;
	
	glShaderSource(sh, numSrc,psource,0);
	glCompileShader(sh);
	GLint status;
	glGetShaderiv(sh,GL_COMPILE_STATUS,&status);
	if (status==GL_FALSE){
		size_t len;
		glGetShaderInfoLog(sh,buffersize,&len,errTextOut);
		errTextOut[(len<(buffersize-1))?len:(buffersize-1)]=0;
		LOGW("%s",errTextOut);
	}
	return status;
}

void InitState() {
	glMatrixMode(GL_PROJECTION);
	glLoadIdentity();
	glMatrixMode(GL_MODELVIEW);
	glLoadIdentity();
}

void CreateShaders() {
	InitState();
	printf("Creating shaders - fixed function pipe");
	struct GlEmuState* s=&g_State;
	GLint vs,ps,prog;
	s->fixed_function_vs=vs=glCreateShader(GL_VERTEX_SHADER);
	s->fixed_function_ps=ps=glCreateShader(GL_FRAGMENT_SHADER);
	s->fixed_function_prog =prog= prog=glCreateProgram();
	const char* vsource[1]={};

	ShaderCompileSource(vs, g_FixedFunctionVS);
	ShaderCompileSource(ps, g_FixedFunctionPS);

	glBindAttribLocation(prog, 0, "a_pos");
	glBindAttribLocation(prog, 1, "a_col");
	glBindAttribLocation(prog, 2, "a_norm");
	glBindAttribLocation(prog, 3, "a_tex0");
	glBindAttribLocation(prog, 4, "a_tex1");

	glAttachShader(prog,ps);
	glAttachShader(prog,vs);
	glLinkProgram(prog);
	GLint status;
	glGetProgramiv(prog,GL_LINK_STATUS,&status);
	LOGW("link result=%d", status);
	char infolog[1024];size_t len;
	glGetProgramInfoLog(prog, 1024,len, &infolog[0]);
	LOGW("link: %s", infolog);

	g_uMatModelView = glGetUniformLocation(prog, "uMatModelView");
	g_uMatProj = glGetUniformLocation(prog, "uMatProj");
	LOGW("uniforms: %d %d", g_uMatModelView, g_uMatProj);
}


void FlushPrim() {
	// todo: test if glMapBuffer is in GLES
	struct GlEmuState* s=&g_State;
	if (!s->vbo) {
		glGenBuffers(1,&s->vbo);
		glBindBuffer(GL_ARRAY_BUFFER,s->vbo);
	}
	if (!s->fixed_function_prog) {
		CreateShaders();
	}

//	GLfloat data[16]={1.f,0.f,0.f,0.f,  0.f,1.f,0.f, 0.f,0.f,1.f, 0.f,0.f,0.f,1.f};
	glUniformMatrix4fv(g_uMatProj,1, GL_FALSE, s->mat_proj);
	glUniformMatrix4fv(g_uMatModelView,1, GL_FALSE, s->mat_model_view);

	glUseProgram(s->fixed_function_prog);
	glBindBuffer(GL_ARRAY_BUFFER,s->vbo);
	glBufferData(GL_ARRAY_BUFFER, sizeof(struct TmpVertex)* s->index, s->vertices, GL_DYNAMIC_DRAW);
	GLsizei stride=sizeof(struct TmpVertex);

	glVertexAttribPointer(0, 4, GL_FLOAT, GL_FALSE, stride, (void*)&((struct TmpVertex*)0)->x);
	glVertexAttribPointer(1, 4, GL_FLOAT, GL_FALSE, stride, (void*)&((struct TmpVertex*)0)->r);
	glVertexAttribPointer(2, 3, GL_FLOAT, GL_FALSE, stride, (void*)&((struct TmpVertex*)0)->nx);
	glVertexAttribPointer(3, 2, GL_FLOAT, GL_FALSE, stride, (void*)&((struct TmpVertex*)0)->u0);
	glVertexAttribPointer(4, 2, GL_FLOAT, GL_FALSE, stride, (void*)&((struct TmpVertex*)0)->u1);
	glDrawArrays(s->prim, 0, s->index);
	s->index=0;
}

void glBegin(GLint a) {
	struct GlEmuState* s=&g_State;
	FlushPrim();
	s->prim=a;
	s->index=0;
}

void glEnd() {
	FlushPrim();
}
struct TmpVertex* GetNextVertex() 
{
	struct GlEmuState* s=&g_State;
	if (s->index >= MAX_VERTICES) {
		FlushPrim();
	}
	struct TmpVertex* vt = s->vertices+s->index++;
	return	vt;
}
struct TmpVertex* GetVertex() 
{
	struct GlEmuState* s=&g_State;
	if (s->index >= MAX_VERTICES) {
		FlushPrim();
	}
	struct TmpVertex* vt = s->vertices+s->index;
	return	vt;
}


void glColor3f(GLfloat r,GLfloat g,GLfloat b) {
	struct TmpVertex* vt=GetVertex();
	vt->r = r; vt->g=g; vt->b = b; vt->a=1.0;
}
void glNormal3f(GLfloat x,GLfloat y,GLfloat z) {
	struct TmpVertex* vt=GetVertex();
	vt->nx = x; vt->ny=y; vt->nz = z; 
}

void glColor4f(GLfloat r,GLfloat g,GLfloat b, GLfloat a) {
	struct TmpVertex* vt=GetVertex();
	vt->r = r; vt->g=g; vt->b = b; vt->a=a;
}

void glTexEnvi(GLenum target, GLenum param, GLint value) {
}
void glVertex2f(GLfloat x,GLfloat y) {
	struct TmpVertex* vt=GetNextVertex();
	vt->x = x; vt->y=y; vt->z = 0.0f; vt->w=1.0f;
}

int g_Random=0;
void glVertex3f(GLfloat x,GLfloat y ,GLfloat z) {
	struct TmpVertex* vt=GetNextVertex();
	if (g_Random) {
		static int rnd=0x1241; rnd^=(rnd>>12); rnd^=rnd<<13; rnd+=02412412; rnd*=112412;
		x=((rnd&0xffff)-0x8000)*(1.0/4096.f);
		rnd^=(rnd>>12); rnd^=rnd<<13; rnd+=02412412; rnd*=112412;
		y=((rnd&0xffff)-0x8000)*(1.0/4096.f);
		rnd^=(rnd>>12); rnd^=rnd<<13; rnd+=02412412; rnd*=112412;
		z=((rnd&0xffff)-0x8000)*(1.0/4096.f);
	}
	vt->x = x; vt->y=y; vt->z = z; vt->w=1.0f;
}
void glVertex4f(GLfloat x,GLfloat y ,GLfloat z,GLfloat w) {
	struct TmpVertex* vt=GetNextVertex();
	vt->x = x; vt->y=y; vt->z = z; vt->w=w;
}

typedef struct Vec2f { float x,y;} Vec2f;
typedef struct Vec3f { float x,y,z;} Vec3f;
typedef struct Vec4f { float x,y,z,w;} Vec4f;
void glVertexC(Vec3f pos, Vec4f color) {
}
void glutSwapBuffers() {
}
void glutIdleFunc(void* f) {
}
GLint glutGet(GLenum value) {
	return 0;
}
void glutPostRedisplay() {
}
void gluLookAt(double px, double py, double pz, double atx, double aty, double atz, double upx, double upy, double upz) {
}
void gluPerspective(double xmin, double xmax, double ymin, double ymax, double zmin, double zmax, double fov, double aspect) {
}

void glGenBuffersARB(GLsizei num, GLuint* out) {
	glGenBuffers(num,out);
}
void glBindBufferARB(GLenum type, GLuint buffer) {
	glBindBuffer(type, buffer);
}
void glShadeModel(GLenum mode ){
}
//void glVertexPointer(void*) {
//}
void glRotatef(GLfloat angle, GLfloat x,GLfloat y, GLfloat z) {
}
void glScalef(GLfloat x,GLfloat y, GLfloat z) {
}
void glTranslatef(GLfloat x,GLfloat y, GLfloat z) {
}
void glLoadIdentity() {
	float* pmat = GetMatrix();
	int	i;
	for (i=0; i<16; i++) {
		pmat[i] =((i&3)==(i>>2))?1.0f:0.0f;
	}
}
void glLoadMatrix(const GLfloat* m) {
	float* pmat = GetMatrix();
	int	i;
	for (i=0; i<16; i++) {
		pmat[i] =m[i];
	}
}

void glLoadMatrixAt(const GLfloat* f, GLenum mode) {
	glMatrixMode(mode);
	glLoadMatrix(f);
}
void glMatrixMode(GLenum mode) {
	g_State.matrix_mode=mode;
}
void glDrawPixels(GLsizei w, GLsizei h, GLenum fmt, GLenum type, const GLvoid* data) {
}
void glRasterPos2f(GLfloat x, GLfloat y) {
}
void glutReshapeWindow(GLint x, GLint y, GLint w, GLint h){
}
void glutDisplayFunc(void* f) {
}

void glutMainLoop() {
}
void glDrawBuffer(GLenum b){
}


void glTexEnvf(GLenum target, GLenum param, GLfloat value) {
}

void glVertexPointer(GLint dim, GLenum type, GLsizei stride, const GLvoid* offset) {
	// todo - its just glVertexAttribPointer on GL ES
}
void glColorPointer(GLint dim, GLenum type, GLsizei stride, const GLvoid* offset) {
	// todo - its just glVertexAttribPointer on GL ES
}
void glNormalPointer(GLenum type, GLsizei stride, const GLvoid* offset) {
	// todo - its just glVertexAttribPointer on GL ES
}
void glTexCoordPointer(GLint dim, GLenum type, GLsizei stride, const GLvoid* offset) {
	// todo - its just glVertexAttribPointer on GL ES
}
void glEnableClientState(GLenum index) {
}

void glDisableClientState(GLenum index) {
}
void glUniformMatrix4fvARB(GLint uniform, GLsizei count, GLboolean transpose, const GLfloat* values) {
	glUniformMatrix4fv(uniform, count, transpose, values);
}

