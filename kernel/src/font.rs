use lazy_static::lazy_static;
use spin::Mutex;
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

pub struct Writer {
    x_pos: usize,
    y_pos: usize,
    framebuffer: &'static FrameBuffer,
    font: &'static Font,
    color: u32,
}

impl Writer {
    pub fn new(fb: &'static FrameBuffer, font: &'static Font) -> Self {
        Writer {
            x_pos: 0,
            y_pos: 0,
            framebuffer: fb,
            font,
            color: 0xFFFFFF, // Default to white
        }
    }

    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    fn new_line(&mut self) {
        self.x_pos = 0;
        self.y_pos += self.font.header.char_size as usize;
        if self.y_pos >= self.framebuffer.info.height {
            //TODO: implement scrolling
            self.y_pos = 0;
        }
    }

    fn write_char(&mut self, c: char) {
        if self.x_pos + 8 >= self.framebuffer.info.width {
            self.new_line();
        }

        draw_char(self.framebuffer, self.font, c, self.x_pos, self.y_pos, self.color);
        self.x_pos += 8;
    }

    pub fn write_string(&mut self, string: &str) {
        for c in string.chars() {
            if c == '\n' {
                self.new_line()
            } else {
                self.write_char(c);
            }
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(
        unsafe { &*FRAMEBUFFER },
        unsafe { &*FONT }
    ));
}

static mut FRAMEBUFFER: *const FrameBuffer = core::ptr::null();
static mut FONT: *const Font = core::ptr::null();

pub fn init_graphics(fb: &'static FrameBuffer, font: &'static Font) {
    unsafe {
        FRAMEBUFFER = fb;
        FONT = font;
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (
        $crate::font::_print(format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}