extern crate gcc;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    gcc::compile_library(
        "libstb_truetype.a", &["stb_truetype.c"]);
    println!("cargo:rustc-flags=-L native={}", out_dir);
}
