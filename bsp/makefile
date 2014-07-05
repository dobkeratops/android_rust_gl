UNAME=$(shell uname)
ifeq ($(UNAME), Darwin)
	RUST_LINK_GL=-C link-args=" -framework Carbon -framework OpenGL -framework GLUT "
else
	RUST_LINK_GL=-C link-args=" -lGL -lGLU -lglut -lXext -lstdc++"
endif

RUSTC=rustc $(RUST_LINK_GL) 

bsp : bsp.rs 
	$(RUSTC) bsp.rs -o bsp --cfg testbed
	./bsp

run: main
	echo "make run"
	./main

#emacs tags; relies on the rust config in .ctags or wherever.
TAGS: 
	ctags -eR *.rs


clean:
	rm TAGS
	rm ./bsp


