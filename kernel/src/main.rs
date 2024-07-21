#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel::{font::init_graphics, println, BootInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}




#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {

    let fb = &boot_info.framebuffer;
    let font = &boot_info.font;

    init_graphics(fb, font);
    println!("hello from kernel");

    

    loop {}
}

