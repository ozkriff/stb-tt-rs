all: src/stb_tt.rs src/example.rs libstb_truetype.a
	rustc --crate-type=lib src/stb_tt.rs
	rustc src/example.rs -L .

libstb_truetype.a: stb_truetype.c stb_truetype.h
	gcc -fPIC -c stb_truetype.c -o libstb_truetype.a
