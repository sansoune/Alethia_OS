use uefi::println;
use uefi::proto::console::gop::{self, GraphicsOutput};
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams};
use uefi::table::{Boot, SystemTable};

#[repr(C)]
pub enum PixelFormat {
    Rgb,
    Bgr,
}

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

impl FrameBuffer {
    pub fn new(base_addr: *mut u8, size: usize, width: usize, height: usize, pixel_format: PixelFormat, bytes_per_pixel: usize, stride: usize) -> Self {
        FrameBuffer {
            base_addr,
            info: FrameBufferInfo {
                size,
                width,
                height,
                pixel_format,
                bytes_per_pixel,
                stride,
            }
        }
    }
}

pub fn get_frame_buffer(system_table: &SystemTable<Boot>) -> Option<FrameBuffer> {
    println!("{}, {}", file!(), line!());
    let boot_services = system_table.boot_services();
    let gop_handle = boot_services
        .get_handle_for_protocol::<GraphicsOutput>()
        .ok()?;
    let mut gop = unsafe {
        boot_services
            .open_protocol::<GraphicsOutput>(
                OpenProtocolParams {
                    handle: gop_handle,
                    agent: boot_services.image_handle(),
                    controller: None,
                },
                OpenProtocolAttributes::GetProtocol,
            )
            .ok()
    }?;

    let mode_info = gop.current_mode_info();
    let mut frame_buffer = gop.frame_buffer();
    let info = FrameBufferInfo {
        size: frame_buffer.size(),
        width: mode_info.resolution().0,
        height: mode_info.resolution().1,
        pixel_format: match mode_info.pixel_format() {
            gop::PixelFormat::Rgb => PixelFormat::Rgb,
            gop::PixelFormat::Bgr => PixelFormat::Bgr,
            gop::PixelFormat::Bitmask | gop::PixelFormat::BltOnly => {
                panic!("Bitmask and BltOnly framebuffers are not supported")
            }
        },
        bytes_per_pixel: 4,
        stride: mode_info.stride(),
    };

    Some(FrameBuffer {
        base_addr: frame_buffer.as_mut_ptr(),
        info,
    })
}

pub fn write_to_frame_buffer(frame_buffer: &FrameBuffer, x: usize, y: usize, color: u32) {
    let pixel_offset = ((y * frame_buffer.info.stride + x) * frame_buffer.info.bytes_per_pixel) as isize;
    let pixel_ptr = unsafe { frame_buffer.base_addr.offset(pixel_offset) };

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
