use crate::drivers::framebuffer::{put_pixel, FrameBuffer};

#[repr(C)]
pub struct PSFHeader {
    pub magic: u16,
    pub mode: u8,
    pub char_size: u8,
}

pub struct Font {
    pub header: PSFHeader,
    pub glyphs: &'static [u8],
}

pub fn draw_char(fb: &FrameBuffer, font: &Font, ch: char, x: usize, y: usize, color: u32) {
    let glyph_index = ch as usize; // Assuming ASCII input
    let glyph_size = font.header.char_size as usize;
    let glyph_offset = glyph_index * glyph_size;

    let glyph = &font.glyphs[glyph_offset..glyph_offset + glyph_size];

    for (row, byte) in glyph.iter().enumerate() {
        for bit in 0..8 {
            if (byte & (1 << (7 - bit))) != 0 {
                let px = x + bit;
                let py = y + row;
                put_pixel(fb, px, py, color);
            }
        }
    }
}