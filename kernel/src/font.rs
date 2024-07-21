use crate::frame_buffer::{FrameBuffer, put_pixel};

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

pub fn draw_text(fb: &FrameBuffer, font: &Font, text: &str, x: usize, y: usize, color: u32) {
    let char_width = 8; // PSF1 fonts are typically 8 pixels wide
    // let char_height = font.header.char_size as usize; // The height is defined in the header

    for (i, ch) in text.chars().enumerate() {
        let x_offset = x + i * char_width;
        draw_char(fb, font, ch, x_offset, y, color);
    }
}