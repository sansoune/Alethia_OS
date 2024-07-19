use core::iter::Iterator;
use crate::frame_buffer::{FrameBuffer, PixelFormat};
use core::assert;

#[repr(C)]
pub struct PSF1Header {
    magic: u16,
    mode: u8,
    char_size: u8,
}

pub struct PSF1Font {
    pub header: &'static PSF1Header,
    pub glyph_buffer: &'static [u8],
}

pub fn load_psf1_font(font_memory: *const u8) -> PSF1Font {
    let header = unsafe { &*(font_memory as *const PSF1Header) };
    assert!(header.magic == 0x0436, "Invalid PSF1 font file magic number");

    let glyph_buffer = unsafe {
        core::slice::from_raw_parts(
            font_memory.add(core::mem::size_of::<PSF1Header>()),
            header.char_size as usize * 256,
        )
    };

    PSF1Font { header, glyph_buffer }
}

// pub fn render_text(frame_buffer: &FrameBuffer, font: &PSF1Font, text: &str, x: usize, y: usize, color: u32) {
//     for (i, c) in text.chars().enumerate() {
//         render_char(frame_buffer, font, c, x + i * 8, y, color); // Assuming each character is 8 pixels wide
//     }
// }

// fn render_char(frame_buffer: &FrameBuffer, font: &PSF1Font, c: char, x: usize, y: usize, color: u32) {
//     let glyph = &font.glyph_buffer[c as usize * font.header.char_size as usize..];

//     for (i, row) in glyph.iter().enumerate() {
//         for bit in 0..8 {
//             if (row & (1 << bit)) != 0 {
//                 write_to_frame_buffer(frame_buffer, x + 7 - bit, y + i, color);
//             }
//         }
//     }
// }

// pub fn write_to_frame_buffer(frame_buffer: &FrameBuffer, x: usize, y: usize, color: u32) {
//     let pixel_offset = ((y * frame_buffer.info.stride + x) * frame_buffer.info.bytes_per_pixel) as isize;
//     let pixel_ptr = unsafe { frame_buffer.base_addr.offset(pixel_offset) };

//     match frame_buffer.info.pixel_format {
//         PixelFormat::Rgb => unsafe {
//             *pixel_ptr = (color >> 16) as u8; // Red
//             *(pixel_ptr.offset(1)) = ((color >> 8) & 0xFF) as u8; // Green
//             *(pixel_ptr.offset(2)) = (color & 0xFF) as u8; // Blue
//         },
//         PixelFormat::Bgr => unsafe {
//             *pixel_ptr = (color & 0xFF) as u8; // Blue
//             *(pixel_ptr.offset(1)) = ((color >> 8) & 0xFF) as u8; // Green
//             *(pixel_ptr.offset(2)) = (color >> 16) as u8; // Red
//         },
//     }
// }
