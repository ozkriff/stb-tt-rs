extern crate gcc;

fn main() {
    gcc::compile_library(
        "libstb_truetype.a", &["stb_truetype.c"]);
}
