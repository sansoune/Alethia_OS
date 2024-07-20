#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel::BootInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}




#[no_mangle]
pub extern "C" fn _start(boot_info: &BootInfo) -> ! {

    let fb = &boot_info.framebuffer;

    for x in 0 .. 200 {
        for y in 0 .. 200 {
            let color = 0x00FF00; // Red in RGB format

            let pixel_offset = ((y * fb.info.stride + x) * fb.info.bytes_per_pixel) as isize;
            let pixel_ptr = unsafe { fb.base_addr.offset(pixel_offset) };

            unsafe {
                *pixel_ptr = (color >> 16) as u8;
                *pixel_ptr.offset(1) = ((color >> 8) & 0xFF) as u8;
                *pixel_ptr.offset(2) = (color & 0xFF) as u8;
            }
        }
    }

    loop {}
}

