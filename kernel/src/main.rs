#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel::drivers::framebuffer::writer::init_graphics;
use kernel::drivers::framebuffer::writer::WRITER;
use kernel::hlt;
use kernel::println;
use kernel::serial_println;
use kernel:: BootInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("paniced: {}", info);
    hlt();
}




#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {

    let fb = &boot_info.framebuffer;
    let font = &boot_info.font;

    init_graphics(fb, font);
    println!("hello from kernel");
    serial_println!("the os booted");
    kernel::init();
    // x86_64::instructions::interrupts::int3();

    //triger a page fault
    // unsafe {
    //     *(0x11deadbeef as *mut u8) = 42;
    // };

    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // // trigger a stack overflow
    // stack_overflow();
    println!("it did not crash");

    hlt();
}