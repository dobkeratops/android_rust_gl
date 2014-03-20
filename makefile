RUST_LINK_GL=-C link-args="-lGL -lGLU -lglut -LXext"
RUSTC=rustc $(RUST_LINK_GL) 

main : main.rs *.rs TAGS
	$(RUSTC) main.rs


run: main
	echo "make run"
	./main

#emacs tags; relies on the rust config in .ctags or wherever.
TAGS: 
	ctags -eR *.rs


clean:
	rm ./*.o
	rm ./*.a
	rm ./main


