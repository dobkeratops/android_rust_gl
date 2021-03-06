# Copyright (C) 2010 The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)

LOCAL_MODULE    := rust-prebuilt
LOCAL_SRC_FILES := librust_android.a

include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)

LOCAL_MODULE    := native-activity
LOCAL_SRC_FILES := main.c desktop_gl_emu.c

#LOCAL_LDLIBS    := -llog -landroid -lEGL -lGLESv1_CM 
#LOCAL_LDLIBS    :=  -llog -landroid -lEGL -lGLESv2
#.. we tried this as a random attempt based on error messages recomending 'dl log gcc c m'
# further we removed '-lgcc' because it warns us
LOCAL_LDLIBS    := -ldl -llog -lgcc -lc -lm -landroid -lEGL -lGLESv2
LOCAL_STATIC_LIBRARIES +=  android_native_app_glue rust-prebuilt

include $(BUILD_SHARED_LIBRARY)

$(call import-module,android/native_app_glue)

