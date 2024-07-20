#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{load_file::{load_kernel, open_file}, BootInfo};
use uefi::{prelude::*, println, CStr16};
use bootloader::frame_buffer::{get_frame_buffer, write_to_frame_buffer};

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
        
    

    let fb = get_frame_buffer(&st).expect("couldn't load the frame buffer");
    write_to_frame_buffer(&fb, 100, 100, 0xFF0000);
    println!("entry point offset: {}", entry_point_offset);

    let bootinfo = BootInfo {
        framebuffer: fb,
    };

    // let (_runtime, _) = st.exit_boot_services(MemoryType::LOADER_DATA);
    
    let entry_point = kernel_add + entry_point_offset;

    let kernel_main: extern "sysv64" fn(&BootInfo) -> ! =
        unsafe { core::mem::transmute(entry_point) };
    kernel_main(&bootinfo);

    Status::SUCCESS
}
