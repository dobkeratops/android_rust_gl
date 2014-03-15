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
void glBegin(GLint a) {
}
void glEnd(GLint a) {
}
void glColor3f(GLfloat r,GLfloat g,GLfloat b) {
}

void glColor4f(GLfloat r,GLfloat g,GLfloat b, GLfloat a) {
}

void glTexEnvi(GLenum target, GLenum param, GLint value) {
}
void glVertex3f(GLfloat x,GLfloat y ,GLfloat z) {
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
}
void glMatrixMode(GLenum mode) {
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


