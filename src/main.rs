#![no_std]
#![no_main]

use alethia_os::{frame_buffer::{get_frame_buffer, write_to_frame_buffer}, load_file::load_file};
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
    
    

    
    if let Some(mut frame_buffer) = get_frame_buffer(&system_table) {
        // print_text(&mut frame_buffer, 100, 100, "hello from frame buffer", 0xFF0000);
        write_to_frame_buffer(&mut frame_buffer, 100, 100, 0xFF0000);
        write_to_frame_buffer(&mut frame_buffer, 101, 100, 0xFF0000);
        write_to_frame_buffer(&mut frame_buffer, 102, 100, 0xFF0000);
        write_to_frame_buffer(&mut frame_buffer, 103, 100, 0xFF0000);
        write_to_frame_buffer(&mut frame_buffer, 104, 100, 0xFF0000);
        write_to_frame_buffer(&mut frame_buffer, 105, 100, 0xFF0000);
        write_to_frame_buffer(&mut frame_buffer, 106, 100, 0xFF0000);
    };
    println!("kernel loaded");

    let kernel_entry = unsafe { core::mem::transmute::<_, fn()>(kernel_memory) };
    kernel_entry();

    Status::SUCCESS
}