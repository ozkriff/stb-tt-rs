extern crate libc;

use libc::{c_int, c_uchar};
use std::io::fs::PathExtensions;

#[link(name = "stb_truetype")]
extern { }

pub mod ffi {
    use libc::{c_int, c_uchar, c_float, c_void};
    use std::ptr;

    pub struct FontInfo {
       userdata: *const c_void,

       // pointer to .ttf file
       data: *const c_uchar,

       // offset of start of font
       fontstart: c_int,

       // number of glyphs, needed for range checking
       numGlyphs: c_int,

       // table locations as offset from start of .ttf
       loca: c_int,
       head: c_int,
       glyf: c_int,
       hhea: c_int,
       hmtx: c_int,
       kern: c_int,

       // a cmap mapping for our chosen character encoding
       index_map: c_int,

       // format needed to map from glyph index to glyph
       indexToLocFormat: c_int,
    }

    impl FontInfo {
        pub fn new() -> FontInfo {
            FontInfo {
                userdata: ptr::null(),
                data: ptr::null(),
                fontstart: 0,
                numGlyphs: 0,
                loca: 0,
                head: 0,
                glyf: 0,
                hhea: 0,
                hmtx: 0,
                kern: 0,
                index_map: 0,
                indexToLocFormat: 0,
            }
        }
    }

    extern {
        pub fn stbtt_GetFontOffsetForIndex(
            data: *const c_uchar,
            index: c_int,
        ) -> c_int;


        pub fn stbtt_InitFont(
            info: *mut FontInfo,
            data2: *const c_uchar,
            fontstart: c_int,
        );

        pub fn stbtt_GetFontVMetrics(
            info: *const FontInfo,
            ascent: *mut c_int,
            descent: *mut c_int,
            lineGap: *mut c_int,
        );

        pub fn stbtt_FindGlyphIndex(
            info: *const FontInfo,
            unicode_codepoint: c_int,
        ) -> c_int;

        pub fn stbtt_GetGlyphHMetrics(
            info: *const FontInfo,
            glyph_index: c_int,
            advanceWidth: *mut c_int,
            eftSideBearing: *mut c_int,
        );

        pub fn stbtt_GetGlyphBitmapBox(
            font: *const FontInfo,
            glyph: c_int,
            scale_x: c_float,
            scale_y: c_float,
            ix0: *mut c_int,
            iy0: *mut c_int,
            ix1: *mut c_int,
            iy1: *mut c_int,
        );

        pub fn stbtt_GetGlyphKernAdvance(
            info: *const FontInfo,
            glyph1: c_int,
            glyph2: c_int,
        );

        pub fn stbtt_MakeGlyphBitmap(
            info: *const FontInfo,
            output: *const c_uchar,
            out_w: c_int,
            out_h: c_int,
            out_stride: c_int,
            scale_x: c_float,
            scale_y: c_float,
            glyph: c_int,
        );

        pub fn stbtt_ScaleForPixelHeight(
            info: *const FontInfo,
            height: c_float,
        ) -> c_float;

        pub fn stbtt_GetGlyphBitmap(
            info: *const FontInfo,
            scale_x: c_float,
            scale_y: c_float,
            glyph: c_int,
            width: *mut c_int,
            height: *mut c_int,
            xoff: *mut c_int,
            yoff: *mut c_int,
        ) -> *const c_uchar;
    }
}

pub struct Font {
    font_info: ffi::FontInfo,
    data: Vec<c_uchar>,
    height: f32,
    scale: f32,
    ascent: i32,
    descent: i32,
    line_gap: i32,
}

impl Font {
    pub fn new(font_path: &Path, height: f32) -> Font {
        let data = {
            use std::io::{BufferedReader, File};
            if !font_path.exists() {
                fail!("Wrong font path: {}", font_path.display());
            }
            let file = File::open(font_path);
            let mut reader = BufferedReader::new(file);
            reader.read_to_end().unwrap()
        };
        let mut font_info = ffi::FontInfo::new();
        unsafe {
            let font_offset = ffi::stbtt_GetFontOffsetForIndex(data.get(0) as *const u8, 0);
            ffi::stbtt_InitFont(&mut font_info, data.get(0) as *const u8, font_offset);
        }
        let scale = unsafe {
            ffi::stbtt_ScaleForPixelHeight(&font_info, height)
        };
        let mut c_ascent: c_int = 0;
        let mut c_descent: c_int = 0;
        let mut c_line_gap: c_int = 0;
        unsafe {
            ffi::stbtt_GetFontVMetrics(&font_info,
                &mut c_ascent, &mut c_descent, &mut c_line_gap);
        }
        let ascent = (c_ascent as f32 * scale) as i32;
        let descent = (c_descent as f32 * scale) as i32;
        let line_gap = (c_line_gap as f32 * scale) as i32;
        Font {
            font_info: font_info,
            data: data,
            height: height,
            scale: scale,
            ascent: ascent,
            descent: descent,
            line_gap: line_gap,
        }
    }
    
    pub fn get_glyph(&self, glyph_index: i32) -> (Vec<c_uchar>, i32, i32, i32, i32) {
        use std::vec::raw::from_buf;
        let mut w = 0;
        let mut h = 0;
        let mut xoff = 0;
        let mut yoff = 0;
        let bitmap = unsafe {
            let scale_x = 0.0;
            let scale_y = self.scale;
            let buf = ffi::stbtt_GetGlyphBitmap(
                &self.font_info,
                scale_x,
                scale_y,
                glyph_index as c_int,
                &mut w,
                &mut h,
                &mut xoff,
                &mut yoff,
            );
            from_buf(buf, (w * h) as uint)
        };
        (bitmap, w as i32, h as i32, xoff as i32, yoff as i32)
    }

    pub fn find_glyph_index(&self, c: char) -> i32 {
        unsafe {
            ffi::stbtt_FindGlyphIndex(&self.font_info, c as c_int) as i32
        }
    }

    pub fn get_glyph_bitmap_box(&self, glyph_index: i32) -> (i32, i32, i32, i32) {
        let scale_x = 0.0;
        let scale_y = self.scale;
        let mut ix0 = 0;
        let mut iy0 = 0;
        let mut ix1 = 0;
        let mut iy1 = 0;
        unsafe {
            ffi::stbtt_GetGlyphBitmapBox(
                &self.font_info,
                glyph_index as c_int,
                scale_x,
                scale_y,
                &mut ix0,
                &mut iy0,
                &mut ix1,
                &mut iy1,
            );
        }
        (ix0 as i32, iy0 as i32, ix1 as i32, iy1 as i32)
    }
}

// vim: set tabstop=4 shiftwidth=4 softtabstop=4 expandtab:
