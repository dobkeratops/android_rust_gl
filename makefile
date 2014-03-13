main : main.rs *.rs TAGS
	rustc main.rs

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


