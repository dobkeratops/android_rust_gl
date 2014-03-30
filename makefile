RUST_LINK_GL=-C link-args=" -lGL -lGLU -lglut -lXext -lstdc++"
RUSTC=rustc $(RUST_LINK_GL) 

main : main.rs *.rs TAGS 
	$(RUSTC) main.rs


run: main
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


