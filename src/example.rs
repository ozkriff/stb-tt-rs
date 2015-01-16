#![allow(unstable)]

extern crate stb_tt;

fn byte_to_char(n: u8) -> &'static str {
    let chars = [" ", ".", ":", "i", "o", "V", "M", "@"];
    let n = (n >> 5) as usize;
    assert!(n < chars.len());
    chars[n]
}

fn print_char(font: &stb_tt::Font, c: char) {
    let glyph_index = font.find_glyph_index(c);
    let (bitmap, w, h, _, _) = font.get_glyph(glyph_index);
    for j in range(0, h) {
        for i in range(0, w) {
            print!("{}", byte_to_char(bitmap[(j * w + i) as usize]));
        }
        print!("\n");
    }
}

fn main() {
    let font = stb_tt::Font::new(&Path::new("DroidSerif-Regular.ttf"), 30.0);
    print_char(&font, '#');
}

// vim: set tabstop=4 shiftwidth=4 softtabstop=4 expandtab:
