#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{load_file::{load_font, load_kernel, open_file}, BootInfo};
use uefi::{helpers::system_table, prelude::*, println, table::boot::{MemoryDescriptor, MemoryMapIter}, CStr16};
use uefi::table::boot::MemoryType;
use bootloader::frame_buffer::get_frame_buffer;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn get_memory_info(memory_map: MemoryMapIter) -> (u64, u64) {
    let mut lower_memory: u64 = 0;
    let mut upper_memory: u64 = 0;

    for dec in memory_map {
        if dec.ty == MemoryType::CONVENTIONAL {
            let start = dec.phys_start;
            let end = start + dec.page_count * 4096;

            if start < 0x100000 {
                lower_memory += end.min(0x100000) - start;
            }
            if end > 0x100000 {
                upper_memory += end - start.max(0x100000);
            }
        }
    }

    (lower_memory, upper_memory)
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
    let (loawer_memory, upper_memory) = get_memory_info(memory_map.entries());
    println!("lower: {:#x}, upper: {:#x}", loawer_memory, upper_memory);

    let fb = get_frame_buffer(&st).expect("couldn't load the frame buffer");


    let mut font_name_buff = [0u16; 128];
    let font_name = CStr16::from_str_with_buf("font.psf", &mut font_name_buff).unwrap();
    let mut font_file = open_file(&st, font_name);
    let font = load_font(&st, &mut font_file);

    let (_runtime, _) = st.exit_boot_services(MemoryType::LOADER_DATA);

    let bootinfo = BootInfo {
        framebuffer: fb,
        font,
    };

    
    let entry_point = kernel_add + entry_point_offset;
    
    let kernel_main: extern "sysv64" fn(&BootInfo) -> ! =
    unsafe { core::mem::transmute(entry_point) };

    kernel_main(&bootinfo);

    Status::SUCCESS
}
