#![no_std]

use font::Font;
use frame_buffer::FrameBuffer;



pub mod frame_buffer;
pub mod load_file;
pub mod elf;
pub mod font;


#[repr(C)]
pub struct BootInfo {
    pub framebuffer: FrameBuffer,
    pub font: Font,
}
