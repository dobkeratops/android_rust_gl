/*
 * Copyright (C) 2010 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

//BEGIN_INCLUDE(all)
#include <jni.h>
#include <errno.h>
#include <string.h>
#include <stdio.h>

#include <EGL/egl.h>
#include <GLES2/gl2.h>
#include <GLES2/gl2ext.h>



#include <android/sensor.h>
#include <android/log.h>
#include <android_native_app_glue.h>
#include <android/input.h>
#include <android/keycodes.h>

#define LOGI(...) ((void)__android_log_print(ANDROID_LOG_INFO, "native-activity", __VA_ARGS__))
#define LOGW(...) ((void)__android_log_print(ANDROID_LOG_WARN, "native-activity", __VA_ARGS__))

// Dummy functions to patch around missing link informatoin- 
// these are evidently only used in the case of Rust task failiure which will be broken.
// TODO - get the makefile working and delete these.
void _Unwind_GetIP(){}
void _Unwind_SetIP(){}
void _Unwind_SetGR(){}

// statically linked hooks for the main application to interact with the activity cycle.
// user of this framework passes an application object.
typedef void App_t;
App_t* g_pApp;

extern App_t* app_create(int argc, const char** argv, int w, int h);
extern void app_display_create(App_t* borrowed_app_ref);	// creating display
//extern void rust_android_event();			// IO events ?
extern void app_render(App_t* borrowed_app_ref);
extern void app_display_destroy(App_t* borrowed_app_ref);	// losing display
extern void app_destroy(App_t* owned_app_ptr);

/**
 * Our saved state data.
 */
struct saved_state {
    float angle;
    int32_t x;
    int32_t y;
};

struct Vec3f {
	float x,y,z;	// todo: add rotaion info, incase its usefull..
};

#define MAX_TOUCH_POINTERS	12
struct TouchPointer {
	struct Vec3f pos; int32_t active;
};

struct AndroidInput {
	struct AndroidInputSub_s {
		struct TouchPointer pointers[MAX_TOUCH_POINTERS];
		struct Vec3f	accelerometer;
	} curr,prev;
	// TODO: gamepad, keyboard... they do exist
};

struct AndroidInput	g_Input;	// dodgy global for interfacing, todo , rust passes object pointer?

struct AndroidInput	android_get_inputs() {
	return	g_Input;
}

void	AndroidInput_Init(struct AndroidInput* ai) {

	int	i,j;
	

	for (i=0; i<MAX_TOUCH_POINTERS; i++) {
		struct TouchPointer p={ {0.0f,0.0f,0.0f}, 0};
		ai->curr.pointers[i]=p;
	}
	struct Vec3f v={0.0f,0.0f,0.0f};
	ai->curr.accelerometer=v;

	ai->prev=ai->curr;
}
void AndroidInput_Update(struct AndroidInput* ai) {
	ai->prev=ai->curr;
}

void AndroidInput_Dump(struct AndroidInput* ai) {
	char ctrl_info[1024]={0},ctrl[256];
	int	i;
	sprintf(ctrl_info,"acc:%.3f,%.3f,%.3f\t", ai->curr.accelerometer.x,ai->curr.accelerometer.y,ai->curr.accelerometer.z);
	for (i=0; i<MAX_TOUCH_POINTERS; i++) {
		struct TouchPointer* tp=&ai->curr.pointers[i];
		if (tp->active) {
			sprintf(ctrl,"\t(%d:(%.3f,%.3f,%.3f))", i, tp->pos.x,tp->pos.y,tp->pos.z);
			strcat(ctrl_info,ctrl);
		}
	}
	strcat(ctrl_info,"\n");
	LOGI("%s", ctrl_info);
}


/**
 * Shared state for our app.
 */
struct engine {
    struct android_app* app;

    ASensorManager* sensorManager;
    const ASensor* accelerometerSensor;
    ASensorEventQueue* sensorEventQueue;

    int animating;
    EGLDisplay display;
    EGLSurface surface;
    EGLContext context;
    int32_t width;
    int32_t height;

	struct AndroidInput inputs;

    struct saved_state state;
};

void Engine_Init(struct engine* e) {
	AndroidInput_Init(&e->inputs);
}

/**
 * Initialize an EGL context for the current display.
 */

static int engine_init_display(struct engine* engine) {
    // initialize OpenGL ES and EGL

    /*
     * Here specify the attributes of the desired configuration.
     * Below, we select an EGLConfig with at least 8 bits per color
     * component compatible with on-screen windows
     */
    const EGLint attribs[] = {
            EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
            EGL_BLUE_SIZE, 8,
            EGL_GREEN_SIZE, 8,
            EGL_RED_SIZE, 8,
			EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
            EGL_NONE
    };
    EGLint w, h, dummy, format;
    EGLint numConfigs;
    EGLConfig config;
    EGLSurface surface;
    EGLContext context;

    EGLDisplay display = eglGetDisplay(EGL_DEFAULT_DISPLAY);

    eglInitialize(display, 0, 0);

    /* Here, the application chooses the configuration it desires. In this
     * sample, we have a very simplified selection process, where we pick
     * the first EGLConfig that matches our criteria */
    eglChooseConfig(display, attribs, &config, 1, &numConfigs);

    /* EGL_NATIVE_VISUAL_ID is an attribute of the EGLConfig that is
     * guaranteed to be accepted by ANativeWindow_setBuffersGeometry().
     * As soon as we picked a EGLConfig, we can safely reconfigure the
     * ANativeWindow buffers to match, using EGL_NATIVE_VISUAL_ID. */
    eglGetConfigAttrib(display, config, EGL_NATIVE_VISUAL_ID, &format);

    ANativeWindow_setBuffersGeometry(engine->app->window, 0, 0, format);

    surface = eglCreateWindowSurface(display, config, engine->app->window, NULL);

	{
		EGLint attribs2[]={
			EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE
		};
	    context = eglCreateContext(display, config, EGL_NO_CONTEXT, attribs2);
	}

    if (eglMakeCurrent(display, surface, surface, context) == EGL_FALSE) {
        LOGW("Unable to eglMakeCurrent");
        return -1;
    } else {
        LOGW("Made context , made current ok");
	}

    eglQuerySurface(display, surface, EGL_WIDTH, &w);
    eglQuerySurface(display, surface, EGL_HEIGHT, &h);
	LOGW("Display: ( %d x %d )",w,h);

    engine->display = display;
    engine->context = context;
    engine->surface = surface;
    engine->width = w;
    engine->height = h;
    engine->state.angle = 0;
    engine->animating=1;

	app_display_create(g_pApp);
    return 0;
}

/**
 * Just the current frame in the display.
 */

static void engine_draw_frame(struct engine* engine) 
{
    if (engine->display == NULL) {
        // No display.
        return;
    }
	app_render(g_pApp);
	AndroidInput_Update(&engine->inputs);	// cache previous controller states for next time..
    eglSwapBuffers(engine->display, engine->surface);
}

/**
 * Tear down the EGL context currently associated with the display.
 */
static void engine_term_display(struct engine* engine) {
    if (engine->display != EGL_NO_DISPLAY) {
        eglMakeCurrent(engine->display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
        if (engine->context != EGL_NO_CONTEXT) {
            eglDestroyContext(engine->display, engine->context);
        }
        if (engine->surface != EGL_NO_SURFACE) {
            eglDestroySurface(engine->display, engine->surface);
        }
        eglTerminate(engine->display);
    }
    engine->animating = 1;
    engine->display = EGL_NO_DISPLAY;
    engine->context = EGL_NO_CONTEXT;
    engine->surface = EGL_NO_SURFACE;
	app_display_destroy(g_pApp);
}

/**
 * Process the next input event.
 */

#define INPUT_LOG //
static int32_t engine_handle_input(struct android_app* app, AInputEvent* event) {
    struct engine* engine = (struct engine*)app->userData;
	int aiet=AInputEvent_getType(event);
	INPUT_LOG("****INPUT EVENT:%d", aiet);


    if (aiet == AINPUT_EVENT_TYPE_MOTION) {

		int aiact= AMotionEvent_getAction(event);
		INPUT_LOG("****MOTION_EVENT_ACTION:%x", aiact);
		int act=aiact & AMOTION_EVENT_ACTION_MASK;
		int aiacti=(aiact & AMOTION_EVENT_ACTION_POINTER_INDEX_MASK)/(AMOTION_EVENT_ACTION_MASK+1);
//		ASSERT(AMOTION_EVENT_ACTION_MASK==0xff);
		{

			int num_pointers = AMotionEvent_getPointerCount(event);
			int	i;
		
//			for (i=0; i<num_pointers; i++) {
//				engine->inputs.curr.pointers[i].active=0;
//			}
			for (i=0; i<num_pointers; i++) {
				int id = AMotionEvent_getPointerId(event, i);
	

				float x=AMotionEvent_getX(event, i);
				float y=AMotionEvent_getY(event, i);
				float z=AMotionEvent_getPressure(event, i);
				struct TouchPointer p={{x,y,z},1};
				if (id >=0 && id<MAX_TOUCH_POINTERS) {
					engine->inputs.curr.pointers[id]=p;
				}
			}
			// Handle ACTION_UPs - overwrites '.active' set above.
			if (act==AMOTION_EVENT_ACTION_POINTER_UP) {
				if (aiacti >=0 && aiacti<MAX_TOUCH_POINTERS) {
					engine->inputs.curr.pointers[aiacti].active=0;
				}
			} else if (act==AMOTION_EVENT_ACTION_UP) {
				// primary pointer up = all...
				int	i;
				for (i=0; i<MAX_TOUCH_POINTERS; i++) {
					engine->inputs.curr.pointers[i].active=0;
				}
			}
		}	
       return 1;	
    }
	else INPUT_LOG("input event unhandled=%d",aiet);

    return 0;
}

/**
 * Process the next main command.
 */
static void engine_handle_cmd(struct android_app* app, int32_t cmd) {
    struct engine* engine = (struct engine*)app->userData;
    switch (cmd) {
        case APP_CMD_SAVE_STATE:
            // The system has asked us to save our current state.  Do so.
            engine->app->savedState = malloc(sizeof(struct saved_state));
            *((struct saved_state*)engine->app->savedState) = engine->state;
            engine->app->savedStateSize = sizeof(struct saved_state);
            break;
        case APP_CMD_INIT_WINDOW:
            // The window is being shown, get it ready.
            if (engine->app->window != NULL) {
                engine_init_display(engine);
                engine_draw_frame(engine);
            }
            break;
        case APP_CMD_TERM_WINDOW:
            // The window is being hidden or closed, clean it up.
            engine_term_display(engine);
            break;
        case APP_CMD_GAINED_FOCUS:
            // When our app gains focus, we start monitoring the accelerometer.
            if (engine->accelerometerSensor != NULL) {
                ASensorEventQueue_enableSensor(engine->sensorEventQueue,
                        engine->accelerometerSensor);
                // We'd like to get 60 events per second (in us).
                ASensorEventQueue_setEventRate(engine->sensorEventQueue,
                        engine->accelerometerSensor, (1000L/60)*1000);
            }
            break;
        case APP_CMD_LOST_FOCUS:
            // When our app loses focus, we stop monitoring the accelerometer.
            // This is to avoid consuming battery while not being used.
            if (engine->accelerometerSensor != NULL) {
                ASensorEventQueue_disableSensor(engine->sensorEventQueue,
                        engine->accelerometerSensor);
            }
            // Also stop animating.
//            engine->animating = 0;
            engine_draw_frame(engine);
            break;
    }
}

/**
 * This is the main entry point of a native application that is using
 * android_native_app_glue.  It runs in its own thread, with its own
 * event loop for receiving input events and doing other things.
 */
 
// todo: supposedly even this is junk that can be eliminated by using the 
// "activity cycle"(?) directly ..(engine_handle_cmd, wherever android calls that..)

void android_main(struct android_app* state) {
	LOGI("******************************************");
	LOGI("******************************************");
	LOGI("******************************************");
	LOGI("******************************************");
    struct engine engine;
	Engine_Init(&engine);
    // Make sure glue isn't stripped.
    app_dummy();

	g_pApp=app_create(0,(const char**) 0,  engine.width, engine.height);
    memset(&engine, 0, sizeof(engine));
    state->userData = &engine;
    state->onAppCmd = engine_handle_cmd;
    state->onInputEvent = engine_handle_input;
    engine.app = state;

    // Prepare to monitor accelerometer
    engine.sensorManager = ASensorManager_getInstance();
    engine.accelerometerSensor = ASensorManager_getDefaultSensor(engine.sensorManager,
            ASENSOR_TYPE_ACCELEROMETER);
    engine.sensorEventQueue = ASensorManager_createEventQueue(engine.sensorManager,
            state->looper, LOOPER_ID_USER, NULL, NULL);

    if (state->savedState != NULL) {
        // We are starting with a previous saved state; restore from it.
        engine.state = *(struct saved_state*)state->savedState;
    }
    // loop waiting for stuff to do.

    while (1) {
        // Read all pending events.
        int ident;
        int events;
        struct android_poll_source* source;

        // If not animating, we will block forever waiting for events.
        // If animating, we loop until all events are read, then continue
        // to draw the next frame of animation.
        while ((ident=ALooper_pollAll(engine.animating ? 0 : -1, NULL, &events,
                (void**)&source)) >= 0) {

            // Process this event.
            if (source != NULL) {
                source->process(state, source);
            }

            // If a sensor has data, process it now.
            if (ident == LOOPER_ID_USER) {
                if (engine.accelerometerSensor != NULL) {
                    ASensorEvent event;
                    int had_sensor=0;
                    while (ASensorEventQueue_getEvents(engine.sensorEventQueue,
                            &event, 1) > 0) {
						struct Vec3f v={event.acceleration.x, event.acceleration.y, event.acceleration.z};
						engine.inputs.curr.accelerometer=v;
//						if (!engine.animating) {
//							static int delay=0; delay++;
//							if (!(delay & 31)) {
//			                    LOGI("accelerometer: x=%f y=%f z=%f",
//			                        event.acceleration.x, event.acceleration.y,
//				                    event.acceleration.z);
//							}
//						}
                    }
                }
            }

            // Check if we are exiting.
            if (state->destroyRequested != 0) {
				goto quit;
            }
        }

        if (engine.animating) {
			g_Input = engine.inputs;
            engine_draw_frame(&engine);
        }
    }
quit:
	engine_term_display(&engine);
	app_destroy(g_pApp);
}

//END_INCLUDE(all)
