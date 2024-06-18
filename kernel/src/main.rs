#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel::{font::{load_psf1_font, render_text}, frame_buffer::FrameBuffer};


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main(frame_buffer: &FrameBuffer, font_memory: *const u8) {

    let font = load_psf1_font(font_memory);
    render_text(frame_buffer, &font, "hello from kernel", 100, 100, 0xFFFFFF);

    loop {

    }
}
