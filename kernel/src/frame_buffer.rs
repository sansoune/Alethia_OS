#[repr(C)]
pub struct FrameBufferInfo {
    pub size: usize,
    pub width: usize,
    pub height: usize,
    pub pixel_format: PixelFormat,
    pub bytes_per_pixel: usize,
    pub stride: usize,
}

#[repr(C)]
pub struct FrameBuffer {
    pub base_addr: *mut u8,
    pub info: FrameBufferInfo,
}

#[repr(C)]
pub enum PixelFormat {
    Rgb,
    Bgr,
}

pub fn put_pixel(framebuffer: &FrameBuffer, x: usize, y: usize, color: u32) {
    let base_addr = framebuffer.base_addr;
    let width = framebuffer.info.width;
    let height = framebuffer.info.height;
    let bytes_per_pixel = framebuffer.info.bytes_per_pixel;
    let stride = framebuffer.info.stride;

    if x >= width || y >= height {
        // Coordinates out of bounds
        return;
    }

    let pixel_offset = ((y * stride + x) * bytes_per_pixel) as isize;
    let pixel_ptr = unsafe { base_addr.offset(pixel_offset) };

    unsafe {
        match framebuffer.info.pixel_format {
            PixelFormat::Rgb => {
                *pixel_ptr = (color >> 16) as u8;
                *pixel_ptr.offset(1) = ((color >> 8) & 0xFF) as u8;
                *pixel_ptr.offset(2) = (color & 0xFF) as u8;
            }
            PixelFormat::Bgr => {
                *pixel_ptr = (color & 0xFF) as u8;
                *pixel_ptr.offset(1) = ((color >> 8) & 0xFF) as u8;
                *pixel_ptr.offset(2) = (color >> 16) as u8;
            }
            _ => {
                // Handle other pixel formats if necessary
            }
        }
    }
}