#![no_std]
#![no_main]

use alethia_os::frame_buffer::{get_frame_buffer, write_to_frame_buffer};
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
    init(&mut system_table).expect("Failed ro initilize");
    system_table
        .stdout()
        .clear()
        .expect("Failed to reset stdout");
    println!("alethia os is booting...");
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

    loop {}
    // Status::SUCCESS
}

