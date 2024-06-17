#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use uefi::*;
use table::{Boot, SystemTable};
use helpers::init;
use core::panic::PanicInfo;



#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}



#[no_mangle]
pub extern "efiapi" fn efi_main(handle: Handle, mut system_table: SystemTable<Boot>) {
    let _ = init(&mut system_table);
    println!("alethia os is booting...");

    println!("kernel loaded");

    loop {}
}
