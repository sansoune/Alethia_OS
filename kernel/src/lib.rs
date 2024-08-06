#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(asm_const)]
#![feature(naked_functions)]


pub mod drivers;
pub mod arch;

use drivers::framebuffer::writer::init_graphics;
pub use drivers::framebuffer::FrameBuffer;
pub use drivers::font::Font;
pub use drivers::framebuffer::Writer;
pub use arch::x86_64::utils::io::*;
pub use arch::x86_64::utils::hlt::hlt;

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

pub fn init() {
    // init_graphics(fb, font)
    arch::x86_64::gdt::init_gdt();
    arch::x86_64::idt::init_idt();
    arch::x86_64::interrupts::init();
    drivers::timer::init();
}