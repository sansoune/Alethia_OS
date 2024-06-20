#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel::BootInfo;
use kernel::frame_buffer::PixelFormat;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {

    // Draw a red pixel at (100, 100)
    let x = 100;
    let y = 100;
    let color = 0xFF0000; // Red in RGB format

    let fb = &boot_info.framebuffer;
    let pixel_offset = (y * fb.info.stride + x * fb.info.bytes_per_pixel) as isize;
    let pixel_ptr = unsafe { fb.base_addr.offset(pixel_offset) };

    unsafe {
        match fb.info.pixel_format {
            PixelFormat::Rgb => {
                *pixel_ptr = (color >> 16) as u8;
                *pixel_ptr.offset(1) = ((color >> 8) & 0xFF) as u8;
                *pixel_ptr.offset(2) = (color & 0xFF) as u8;
            },
            PixelFormat::Bgr => {
                *pixel_ptr = (color & 0xFF) as u8;
                *pixel_ptr.offset(1) = ((color >> 8) & 0xFF) as u8;
                *pixel_ptr.offset(2) = (color >> 16) as u8;
            },
        }
    }

    loop {

    }
}


