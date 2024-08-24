#![no_std]

use font::Font;
use frame_buffer::FrameBuffer;
use memory::MemoryMap;



pub mod frame_buffer;
pub mod load_file;
pub mod elf;
pub mod font;
pub mod memory;


#[repr(C)]
pub struct BootInfo {
    pub framebuffer: FrameBuffer,
    pub font: Font,
    pub memory_map: MemoryMap,
}
