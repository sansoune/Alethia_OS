#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel::{font::draw_text, BootInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}




#[no_mangle]
pub extern "C" fn _start(boot_info: &BootInfo) -> ! {

    let fb = &boot_info.framebuffer;
    let font = &boot_info.font;

    let text = "hello from kernel!";
    let color = 0xFFFFFF;
    draw_text(&fb, &font, text, 10, 10, color);

    loop {}
}

