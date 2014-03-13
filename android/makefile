ANDROID_CXX  = /opt/ndk_standalone/bin/arm-linux-androideabi-gcc


ndk_build: jni/librust_android.a
	echo "building ndk sample, including rust code"
	ndk-build
	echo "packaging apk.."
	ant debug
	echo "deploying apk.."
	adb install -r bin/NativeActivity-debug.apk
	echo "Starting apk..."
	adb shell am start -n com.example.native_activity/android.app.NativeActivity


jni/librust_android.a: ../main.rs ../shadertest.rs
	echo "[1] build Rust source as a library .."
	rustc --target=arm-linux-androideabi ../main.rs -C android-cross-path=/opt/ndk_standalone --crate-type=staticlib -o jni/librust_android.a


clean:
	rm libs/armeabi/libnative-activity.so
	rm obj/local/armeabi/objs/native-activity/main.o
	rm jni/*.a

