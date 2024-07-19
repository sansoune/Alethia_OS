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