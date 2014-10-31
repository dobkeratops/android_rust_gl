// Wraps GLUT in an event queue using Rust enums
// hides use of callbacks acessing a global, allows users to poll input events from
// a user implemented mainloop.
// provides a tablet/console oriented MVC framework


//use r3d::Vec2f;

use common::*;

#[deriving(Show,Clone,PartialEq)]
pub struct Window{handle:i32}	// todo - a trait object ?

pub type Buttons=i32;
pub type Key_t=i32;
pub type Modifiers=i32;
pub type Milliseconds=i64;		// milliseconds
pub type ScreenPos=(i32,i32);
pub type Vec2d=(i32,i32);
pub type TouchId=i32;
pub type Frames=i32;
pub type FramesPerSec=f32;

#[deriving(Show,Clone)]
pub enum WinEvent {
	EventNone,
	MouseMotion(Window, Buttons,ScreenPos),
	MouseButtonDown(Window,Buttons,ScreenPos),
	MouseButtonUp(Window,Buttons,ScreenPos),
	KeyDown(Window,Key_t,Modifiers,ScreenPos),
	KeyUp(Window,Key_t,Modifiers,ScreenPos),
	Accelerometer(Window,(i32,i32,i32)),
	JoypadEvent(Window,Vec2d,Vec2d,Buttons),
	MultiTouchDown(Window,(TouchId,ScreenPos)),
	MultiTouchUp(Window,(TouchId,ScreenPos)),
	MultiTouchMove(Window,(TouchId,ScreenPos)),
	WindowMove(Window, ScreenPos),
	WindowResize(Window,ScreenPos,ScreenPos),
	WindowFocusIn(Window),
	WindowFocusOut(Window),
	WindowClose(Window),
	WindowShow(Window),
	WindowHide(Window)
}
/*
impl ToStr for WinEvent {
	fn to_str(&self)->~str {
		format!("{:?}",*self)
	}
}
*/

pub enum Mode {
	Verbose,
	Silent
}
pub enum Placement {
	Fullscreen,
	Default,
	PlaceAt(ScreenPos,ScreenPos)
}
static g_root_window:Window =Window{handle:0};

pub fn init() {
	unsafe {
		g_rustwin.head=0;
		g_rustwin.tail=0;
		init_window();
		init_input();
	}
}
static mut g_init_input:bool=false;
pub fn get_event()->WinEvent {
	unsafe {
		if g_init_input==false {init_input() }
		glut::glutMainLoopEvent(); 
//		println!("pop event from {:?}", g_rustwin);
//		println!("pos={:?}",g_cursor);

		let mut rw=&mut g_rustwin;
		if rw.head==rw.tail {
			EventNone
		} else {
			let ev=rw.event_queue.get(rw.tail).unwrap().clone();
			rw.tail=(rw.tail+1)&255;
			ev
		}
	}
}

/// Sample main loop wrapper. Possible to write a mainloop easier than this.
pub fn run_loop<APP>(process_event:&mut |ev:WinEvent|, on_idle:&mut |win:Window|) {
	loop {
		while {
			let ev=get_event();
			match ev {
				EventNone=>false,
				_=>{(*process_event)(ev);true}
			}
		}
		{};
		(*on_idle)(g_root_window);	// todo - use ...
	}
}
pub fn is_root_window(w:&Window)->bool { *w==g_root_window }
pub fn get_key_state(k:char)->i32 { unsafe { if g_keys[(k as uint) & 255] {1} else {0} } }


pub struct	Joypad {
	sticks:[Vec2f,..2],
	buttons:i32,
	press:i32,
	unpress:i32
	// rumble ? tilt ? accelerometers?
}
impl Joypad {
	pub fn new()->Joypad {
		Joypad{
			sticks:[zero(),zero()],
			buttons:0,
			press:0,
			unpress:0
		}
	}
}


// todo - read actual joypads.


/////////////////////////////////////////////////////


struct	RustWin {
	event_queue:[WinEvent,..256],
	head:uint,
	tail:uint,
	focus_window:Option<Window>,
//	windows:~[Window]
}

static  mut g_rustwin:RustWin=RustWin{
	event_queue:[EventNone,..256],
	head:0,
	tail:0,
	focus_window:None,
//	windows:~[]
};
/*
unsafe fn get_rust_win<'a>()->*mut RustWin {
	println!("rustwin={:?}", g_rustwin);
	match g_rustwin { None=>ptr::mut_null(), Some(ref x)=>
		{	let rw=&**x as *RustWin as *mut RustWin;
			println!("rustwin ret={:?}", rw);
			rw
		}
	}
}
*/
fn push_event(ev:WinEvent) {
	unsafe {
//		let rw = get_rust_win();
//		println!("rustwin={:?}", rw);
//		println!("rustwin evq={:?}", (*rw).event_queue);
//		println!("event={:?}", ev);
//		println!("push_event rustwin={:?}",g_rustwin);

//		(*rw).event_queue.push_front(ev);
		let rw=&mut g_rustwin;
//		println!("push event{:?}",ev);
		rw.event_queue[rw.head]=ev;
		rw.head=(rw.head+1)&255;
		if rw.head==rw.tail { rw.head=(rw.head-1)&255}

	}
}

fn ev_println(s:String) {
	// silent
}

fn curr_win()->Window {
	unsafe {match g_rustwin.focus_window {Some(w)=>w, None=>g_root_window}}
}


static mut g_prev_buttons:i32=0;
static mut g_curr_buttons:i32=0;
static mut g_key_modifiers:i32=0;
static mut g_cursor:ScreenPos=(0,0);
static mut g_drag_start:ScreenPos=(0,0);
static mut g_prev_cursor:ScreenPos=(0,0);
static mut g_keys:[bool,..256]=[false,..256];

pub struct	MouseState {
	pos:ScreenPos,
	delta:ScreenPos, 
	drag_start:ScreenPos,
	buttons:Buttons,
	press:Buttons,
	unpress:Buttons
}

fn sub2d<T:Num>((x0,y0):(T,T),(x1,y1):(T,T))->(T,T){ (x0-x1,y0-y1)}
fn add2d<T:Num>((x0,y0):(T,T),(x1,y1):(T,T))->(T,T){ (x0+x1,y0+y1)}

pub fn peek_mouse()->MouseState{
	unsafe {
		MouseState{
			pos:g_cursor,
			delta:sub2d(g_cursor,g_prev_cursor),
			drag_start:g_drag_start,
			buttons:g_curr_buttons,
			press:(g_curr_buttons^g_prev_buttons)&g_curr_buttons,
			unpress:(g_curr_buttons^g_prev_buttons)&g_prev_buttons,
		} 
	}
}

pub fn get_mouse()->MouseState{
	unsafe { 
		let ms=peek_mouse();
		g_prev_buttons=g_curr_buttons;
		g_prev_cursor=g_cursor;
		ms
	}
}

fn special_key_to_mask(k:c_int)->i32 {
	println!("special key: {}",k);
	0
}

extern "C" fn tablet_motion(x: c_int,y: c_int) {
	println!("TabletMotion {}",(x,y));
}
extern "C" fn tablet_button(button:c_int,state:c_int,x:c_int,y:c_int) {
	println!("TabletButton:{} pos {}",(button,state),(x,y));
}
extern "C" fn mouse(button:c_int,state:c_int,x:c_int,y:c_int) {
	let mask = 1<<(button as uint);
	unsafe {
		if state==0 { g_drag_start=g_cursor; }
		g_cursor = (x,y);
		g_curr_buttons=match state{ 0=>g_curr_buttons|mask, _=>g_curr_buttons& !mask  };
		push_event(
			match state {
				0=>MouseButtonUp(curr_win(),button,(x,y)),
				_=>MouseButtonDown(curr_win(),button,(x,y)),
			}
		);
	}
	ev_println(format!("MouseButton:{} at {}",(button,state),(x,y)));
}
extern "C" fn motion(x:c_int,y:c_int){
	unsafe {
		g_cursor=(x,y);
		ev_println(format!("MousePos:{}",(x,y)));
		push_event(MouseMotion(curr_win(), g_curr_buttons,(x,y)));
	}
}
extern "C" fn passive_motion(x:c_int,y:c_int){
	unsafe {
		g_cursor=(x,y);
		ev_println(format!("MousePos:{}",(x,y)));
		push_event(MouseMotion(curr_win(), g_curr_buttons, (x,y)));
	}
}
extern "C" fn keyboard(k:c_uchar,x:c_int,y:c_int){
	unsafe {
		ev_println(format!("keyDown:{} at{}",k,(x,y)));
		g_keys[k as uint]=true;
		push_event(KeyDown(curr_win(), k as Key_t,g_key_modifiers,(x,y)));
		
	}
}
extern "C" fn keyboard_up(k:c_uchar,x:c_int,y:c_int){
	unsafe {
		ev_println(format!("keyUp:{} at {}",k,(x,y)));
		g_keys[k as uint]=false;
		push_event(KeyUp(curr_win(),k as Key_t,g_key_modifiers, (x,y)));
	}
}
extern "C" fn special(k:c_int,x:c_int,y:c_int) {
	ev_println(format!("specialKeyDown:{} at{}",k,(x,y)));
	// todo - translate special key into modifiers, thru glut enum
	unsafe {
		g_key_modifiers|=special_key_to_mask(k);
		push_event(KeyDown(curr_win(),k as Key_t,g_key_modifiers, (x,y)));
	}
}
extern "C" fn special_up(k:c_int,x:c_int,y:c_int) {
	ev_println(format!("specialKeyUp:{} at {}",k,(x,y)));
	// todo - translate special key into modifiers, thru glut enum
	unsafe{
		g_key_modifiers&=special_key_to_mask(k);
		push_event(KeyUp(curr_win(), k as Key_t,g_key_modifiers,(x,y)));
	}
}

pub fn init_input() {
	unsafe {
		g_init_input=true;
		glut::glutMouseFunc(mouse);
		glut::glutMotionFunc(motion);
		glut::glutPassiveMotionFunc(passive_motion);
		glut::glutKeyboardFunc(keyboard);
		glut::glutKeyboardUpFunc(keyboard_up);
		glut::glutSpecialFunc(special);
		glut::glutSpecialUpFunc(special_up);
		glut::glutTabletMotionFunc(tablet_motion);
		glut::glutTabletButtonFunc(tablet_button);
	}
}

fn init_window()
{
	unsafe {
		glut::glutInit(&mut 0 as *mut c_int,0 as *const *const c_char);
		glut::glutInitDisplayMode(GLUT_RGB|GLUT_DOUBLE|GLUT_DEPTH|GLUT_MULTISAMPLE);
		glut::glutInitWindowSize(640,480);
		glut::glutInitWindowPosition(0,0);
		glut::glutSetKeyRepeat(0);
		//	c_glut_init();
		glut::glutCreateWindow(c_str("rust window"));
		glut::glutPopWindow();
		glut::glutIdleFunc(null_func as *const u8);
        glut::glutDisplayFunc(null_func as *const u8);
		gl::glEnable(GL_DEPTH_TEST);
	}
}


pub enum ScreenChange {
	ScContinue,
	ScReplace(Box<Screen+ 'static>),
	ScPush(Box<Screen+ 'static>),
	ScRoot(Box<Screen+ 'static>),
	ScPop,
	ScCycleNext,
	ScCyclePrev
}
 
pub trait Screen {
	fn display_create(&mut self){}
	fn display_destroy(&mut self){}
	fn on_select(&mut self){}
	fn on_deselect(&mut self){}
	fn render(&self)	{}
	fn update(&mut self)->ScreenChange	{ScContinue}
	fn win_event(&mut self, ev: WinEvent)->ScreenChange{ScContinue}
	fn dump(&self){}
}


extern "C" fn null_func() {}
