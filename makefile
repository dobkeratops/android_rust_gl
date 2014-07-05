UNAME=$(shell uname)
ifeq ($(UNAME), Darwin)
	RUST_LINK_GL=-C link-args=" -framework Carbon -framework OpenGL -framework GLUT "
else
	RUST_LINK_GL=-C link-args=" -lGL -lGLU -lglut -lXext -lstdc++"
endif
RUSTC=rustc $(RUST_LINK_GL) 

main : main.rs *.rs r3d/*.rs 
	$(RUSTC) main.rs


run: main
	echo "make run"
	./main

debug: main
	echo "make run"
	./main

#emacs tags; relies on the rust config in .ctags or wherever.
TAGS: 
	ctags -e -f $@ *.rs --options=$(RUST_SRC)/etc/ctags.rust --languages=+rust --recurse *

clean:
	rm ./*.o
	rm ./*.a
	rm ./main
	rm TAGS


