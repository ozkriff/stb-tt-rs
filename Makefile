all: stb_tt.rs stb_truetype.c stb_truetype.h example.rs 
	gcc -fPIC -c stb_truetype.c -o libstb_truetype.a
	rustc --crate-type=lib stb_tt.rs
	rustc example.rs -L .
