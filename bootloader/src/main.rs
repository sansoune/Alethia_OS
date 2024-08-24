#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{load_file::{load_font, load_kernel, open_file}, BootInfo};
use uefi::{helpers::system_table, prelude::*, println, CStr16};
use uefi::table::boot::MemoryType;
use bootloader::frame_buffer::get_frame_buffer;
use bootloader::memory::get_memory_info;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[entry]
fn main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut st).unwrap();
    st
        .stdout()
        .clear()
        .expect("Failed to reset stdout");
    println!("alethia os is booting...");



    let mut kernel_name_buff = [0u16; 12];
    let kernel_name = CStr16::from_str_with_buf("kernel.elf", &mut kernel_name_buff).unwrap();
    let mut kernel_file = open_file(&st, kernel_name);
    let (kernel_add, entry_point_offset) = load_kernel(&st, &mut kernel_file);
        
    let mut memory_map_buffer = [0; 4096 * 4];
    let memory_map = system_table().boot_services().memory_map(&mut memory_map_buffer).expect("failed to get memory map");
    let memory_map = get_memory_info(memory_map.entries());

    let fb = get_frame_buffer(&st).expect("couldn't load the frame buffer");


    let mut font_name_buff = [0u16; 128];
    let font_name = CStr16::from_str_with_buf("font.psf", &mut font_name_buff).unwrap();
    let mut font_file = open_file(&st, font_name);
    let font = load_font(&st, &mut font_file);

    let (_runtime, _) = st.exit_boot_services(MemoryType::LOADER_DATA);

    let bootinfo = BootInfo {
        framebuffer: fb,
        font,
        memory_map,
    };

    
    let entry_point = kernel_add + entry_point_offset;
    
    let kernel_main: extern "sysv64" fn(&BootInfo) -> ! =
    unsafe { core::mem::transmute(entry_point) };

    kernel_main(&bootinfo);

    Status::SUCCESS
}
