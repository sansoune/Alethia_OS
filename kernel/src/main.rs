#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel::frame_buffer::{FrameBuffer, PixelFormat};
use kernel::BootInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn write_to_frame_buffer(frame_buffer: &FrameBuffer, x: usize, y: usize, color: u32) {
    let base_addr = frame_buffer.base_addr as *mut u8;
    let width = frame_buffer.info.width;
    let height = frame_buffer.info.height;
    let bytes_per_pixel = frame_buffer.info.bytes_per_pixel;
    let stride = frame_buffer.info.stride;

    if x >= width || y >= height {
        // Coordinates out of bounds
        return;
    }

    let pixel_offset = ((y * frame_buffer.info.stride + x) * bytes_per_pixel) as isize;
    let pixel_ptr = unsafe { base_addr.offset(pixel_offset) };

    match frame_buffer.info.pixel_format {
        PixelFormat::Rgb => unsafe {
            *pixel_ptr = (color >> 16) as u8; // Red
            *(pixel_ptr.offset(1)) = ((color >> 8) & 0xFF) as u8; // Green
            *(pixel_ptr.offset(2)) = (color & 0xFF) as u8; // Blue
        },
        PixelFormat::Bgr => unsafe {
            *pixel_ptr = (color & 0xFF) as u8; // Blue
            *(pixel_ptr.offset(1)) = ((color >> 8) & 0xFF) as u8; // Green
            *(pixel_ptr.offset(2)) = (color >> 16) as u8; // Red
        },
    }
}

#[repr(C)]
struct FrameBufferInfo {
    framebuffer_addr: u64,
    framebuffer_size: usize,
    horzontal_resolution: usize,
    vertical_resolution: usize,
    pixels_per_scan_line: usize,
}

#[no_mangle]
pub extern "C" fn _start(fb: &FrameBuffer) -> ! {

    for x in 0 .. 100 {
        for y in 0 .. 100 {
            let color = 0xFF0000; // Red in RGB format

            let a = 0x80000000;
            let pixel_offset = ((y * fb.info.stride + x) * 4) as u64;
            let pixel_ptr = unsafe { (a + pixel_offset) as *mut u8 };

            unsafe {
                *pixel_ptr = (color >> 16) as u8;
                *pixel_ptr.offset(1) = ((color >> 8) & 0xFF) as u8;
                *pixel_ptr.offset(2) = (color & 0xFF) as u8;
            }
        }
    }

    loop {}
}

fn draw_square(
    framebuffer: *mut u32,
    horizontal_resolution: usize,
    vertical_resolution: usize,
    pixels_per_scan_line: usize,
    size: usize,
    x_pos: usize,
    y_pos: usize,
    color: u32,
) {
    for y in 0..size {
        for x in 0..size {
            if x_pos + x < horizontal_resolution && y_pos + y < vertical_resolution {
                let pixel_index = (y_pos + y) * pixels_per_scan_line + (x_pos + x);
                unsafe {
                    *framebuffer.add(pixel_index) = color;
                }
            }
        }
    }
}
