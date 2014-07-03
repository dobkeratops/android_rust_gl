UNAME=$(shell uname)
ifeq ($(UNAME), Darwin)
	RUST_LINK_GL=-C link-args=" -framework Carbon -framework OpenGL -framework GLUT "
else
	RUST_LINK_GL=-C link-args=" -lGL -lGLU -lglut -lXext -lstdc++"
endif
RUSTC=rustc $(RUST_LINK_GL) 

main : parallel.rs *.rs  
	$(RUSTC) parallel.rs
	./parallel



