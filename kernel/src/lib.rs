#![no_std]

pub mod drivers;

pub use drivers::framebuffer::FrameBuffer;
pub use drivers::font::Font;
pub use drivers::framebuffer::Writer;

#[repr(C)]
pub struct BootInfo {
    pub framebuffer: FrameBuffer,
    pub font: Font,
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (
        $crate::drivers::framebuffer::writer::_print(format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}