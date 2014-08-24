ANDROID_CXX  = /opt/ndk_standalone/bin/arm-linux-androideabi-gcc

#see jni/Android.mk aswell
ndk_build: jni/librust_android.a
	echo "building ndk sample, including rust code"
	ndk-build
	echo "packaging apk.."
	ant debug
	echo "deploying apk.."
	adb install -r bin/NativeActivity-debug.apk
	echo "Starting apk..."
	adb shell am start -n com.example.native_activity/android.app.NativeActivity


#TODO - this was -C android_cross_path=/opt/ndk_standalone.
#we've changed it to -C linker=...  
#we still get errors linking with _Unwind_... stuff. we dont know why
#

jni/librust_android.a: ../main.rs ../shadertest.rs
	echo "[1] build Rust source as a library .."
#	rustc --target=arm-linux-androideabi ../main.rs --android-cross-path=/opt/ndk_standalone --crate-type=staticlib -o jni/librust_android.a
	rustc --target=arm-linux-androideabi ../main.rs  -C linker=/opt/ndk_standalone/bin/arm-linux-androideabi-gcc --crate-type=staticlib  -o jni/librust_android.a


clean:
	rm jni/*.a
	rm libs/armeabi/libnative-activity.so
	rm obj/local/armeabi/objs/native-activity/main.o

