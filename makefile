RUST_LINK_GL=-C link-args="foo.o -lrusty  -lGL -lGLU -lglut -lXext -lstdc++"
RUSTC=rustc $(RUST_LINK_GL) 

main : main.rs *.rs foo.o TAGS 
	rustc rusty.rs --crate-type=staticlib -o librusty.a
	$(RUSTC) main.rs

foo.o:foo.cpp
	g++ -c foo.cpp  -std=c++11 -fno-exceptions -fno-rtti

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


