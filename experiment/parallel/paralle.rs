-*- mode: compilation; default-directory: "~/android_rust_gl/experiment/parallel/" -*-
Compilation started at Sat Jun 21 19:41:18

make
rustc -C link-args=" -framework Carbon -framework OpenGL -framework GLUT "  parallel.rs
parallel.rs:4:1: 4:8 error: unresolved name `println`.
parallel.rs:4 println("parallel test");
              ^~~~~~~
error: aborting due to previous error
make: *** [main] Error 101

Compilation exited abnormally with code 2 at Sat Jun 21 19:41:18
