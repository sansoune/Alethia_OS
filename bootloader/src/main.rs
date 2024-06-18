#![no_std]
#![no_main]

use bootloader::{frame_buffer::{get_frame_buffer, write_to_frame_buffer, FrameBuffer}, load_file::load_file};
use proto::media::file::{File, FileInfo};
use core::panic::PanicInfo;
use helpers::init;
use table::{Boot, SystemTable};
use uefi::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // initilization
    init(&mut system_table).expect("Failed ro initilize");

    //setuping the screen 
    system_table
        .stdout()
        .clear()
        .expect("Failed to reset stdout");
    println!("alethia os is booting...");

    //loading kernel
    let mut kernel = load_file(&system_table, cstr16!("kernel.elf"));

    //getting the kernel info
    let mut small_buffer = [0u8; 128];
    let kernel_info = kernel.get_info::<FileInfo>(&mut small_buffer).expect("Failed to get file info");

    //getting the size of kernel
    let kernel_size = kernel_info.file_size();
    println!("File size: {} bytes", kernel_size);

    //allocating the memroy needed for the kernel
    let kernel_memory = system_table.boot_services().allocate_pool(table::boot::MemoryType::LOADER_DATA, kernel_size as usize).expect("Failed to allocate memory for kernel");

    let kernel_buffer = unsafe { core::slice::from_raw_parts_mut(kernel_memory, kernel_size as usize) };
    kernel.read(kernel_buffer).expect("failed to read kernel");

    kernel.close();
    
    //loading font
    let font_file = load_file(&system_table, cstr16!("font.psf"));
    let mut small_buffer = [0u8; 128];
    let font_info =  font_file.get_info::<FileInfo>(&mut small_buffer).expect("Failed to get font file info");

    //allocating memory
    let font_size = font_info.file_size() as usize;
    let font_memory = system_table.boot_services().allocate_pool(table::boot::MemoryType::LOADER_DATA, font_size).expect("Failed to allocate memory for font");

    //reading font file into memory
    let font_memory_slice = unsafe { core::slice::from_raw_parts_mut(font_memory, font_size) };
    font_file.read(font_memory_slice).expect("Faled to read font file");

    
    let  frame_buffer = get_frame_buffer(&system_table).expect("failed to get frame buffer");

    let kernel_entry: extern "C" fn(&FrameBuffer, *const u8) = unsafe { core::mem::transmute::<_, fn()>(kernel_memory) };
    kernel_entry(&frame_buffer, font_memory);

    Status::SUCCESS
}