#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel::drivers::framebuffer::writer::init_graphics;
use kernel::drivers::framebuffer::writer::WRITER;
use kernel::println;
use kernel:: BootInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("paniced: {}", info);
    loop {}
}




#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {

    let fb = &boot_info.framebuffer;
    let font = &boot_info.font;

    init_graphics(fb, font);
    println!("hello from kernel");
    kernel::init();
    println!("it did not crash");

    loop {}
}