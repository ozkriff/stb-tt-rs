extern crate gcc;

use std::default::Default;

fn main() {
    gcc::compile_library(
        "libstb_truetype.a", &Default::default(), &["stb_truetype.c"]);
}
