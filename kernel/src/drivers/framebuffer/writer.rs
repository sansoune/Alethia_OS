use super::FrameBuffer;
use core::fmt::Write;
use spin::Mutex;
use lazy_static::lazy_static;
use crate::drivers::font::Font;

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

        crate::drivers::font::draw_char(self.framebuffer, self.font, c, self.x_pos, self.y_pos, self.color);
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

impl Write for Writer {
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

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).expect("printing to serial failed");
    });
}
