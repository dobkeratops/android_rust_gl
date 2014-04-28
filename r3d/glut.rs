/*new file*/  
pub use super::gl::*;
extern
{


	pub fn glutInit(argc:*mut c_int,argc:**c_char);
	pub fn glutInitDisplayMode(mode:GLenum);

	pub fn glutCreateWindow(x:*c_char)->c_int;
	pub fn glutCreateSubWindow(x:*c_char)->c_int;
	pub fn glutDestroyWindow(x:c_int);
	pub fn glutSetWindow(win:c_int);
	pub fn glutGetWindow()->c_int;
	pub fn glutSetWindowTitle(x:*c_char);
	pub fn glutSetIconTitle(x:*c_char);
	pub fn glutReshapeWindow(x:GLint, y:GLint);
	pub fn glutPositionWindow(x:c_int,y:c_int);
	pub fn glutIconifyWindow();
	pub fn glutShowWindow();
	pub fn glutHideWindow();
	pub fn glutPushWindow();
	pub fn glutPopWindow();
	pub fn glutFullScreen();
	pub fn glutPostRedisplay();

	pub fn glutPostWindowRedisplay(  window:c_int );
    pub fn glutSwapBuffers();
    pub fn glutMainLoop(); /* for user loop to poll messages*/

/*
 * Mouse cursor functions, see freeglut_cursor.c
 */
	pub fn glutWarpPointer( x:c_int,y:c_int );
	pub fn glutSetCursor( cursor:c_int);


/*
 * Global callback functions, see freeglut_callbacks.c
 */
	pub fn glutTimerFunc( time_val:c_uint, f:RustTempCFunc/*&fn( v:c_int )*/,  data:c_int );
	pub fn glutIdleFunc(f:RustTempCFunc);// f:&fn() );
	pub fn glutDisplayFunc(f:RustTempCFunc);// f:&fn() );

	pub fn glutGameModeString(s:*c_char );
	pub fn glutEnterGameMode( );
	pub fn glutLeaveGameMode( );
	pub fn glutGameModeGet( query:GLenum  );

	pub fn glutInitWindowPosition(x:GLint,y:GLint);
	pub fn glutInitWindowSize(x:GLint,y:GLint);

	pub fn glutSetKeyRepeat( repeatMode:c_int);

	pub fn glutMouseFunc(f:extern "C" fn(button:c_int, state:c_int,x:c_int, y:c_int));
	pub fn glutMotionFunc(f:extern "C" fn(x:c_int, y:c_int));
	pub fn glutPassiveMotionFunc(f:extern "C" fn(x:c_int, y:c_int));
	pub fn glutEntryFunc(f:extern "C" fn (e:c_int ) );
	pub fn glutKeyboardFunc(f:extern "C" fn(key:c_uchar,x:c_int, y:c_int));
	pub fn glutKeyboardUpFunc(f:extern "C" fn(button:c_uchar, x:c_int, y:c_int));
	pub fn glutSpecialFunc(f:extern "C" fn(button:c_int,x:c_int, y:c_int));
	pub fn glutSpecialUpFunc(f:extern "C" fn(button:c_int,x:c_int, y:c_int));
	pub fn glutReshapeFunc(f:extern "C" fn(x:c_int,y:c_int));
	pub fn glutTabletMotionFunc(f:extern "C" fn( x:c_int, y:c_int ) );
	pub fn glutTabletButtonFunc(f:extern "C" fn( button:c_int, state:c_int, x:c_int, y:c_int ) );
	//pub fn glewInit();

/*
	Text functions
*/
	pub fn glutStrokeCharacter(c:*c_void, c:c_char);
}

#[cfg(not(target_os = "macos"))]
extern {
    pub fn glutMainLoopEvent(); /* for user loop to poll messages*/
}
#[cfg(target_os = "macos")]
extern {
    pub fn glutCheckLoop(); /* for user loop to poll messages*/
}

#[cfg(target_os = "macos")]
pub fn glutMainLoopEvent(){ unsafe {glutCheckLoop();} }

