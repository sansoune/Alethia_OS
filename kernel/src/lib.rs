#![no_std]

use frame_buffer::FrameBuffer;

pub mod frame_buffer;
pub mod font;

#[repr(C)]
pub struct BootInfo {
    pub framebuffer: FrameBuffer,
}