#![no_std]
#![no_main]


use core::panic::PanicInfo;
use alethia_os::{console, println, uefi::{ImageHandle, SystemTable}};

// #[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}


#[no_mangle]
pub extern "efiapi" fn efi_main(handle: ImageHandle, system_table: *const SystemTable) {
    unsafe { console::SYSTEM_TABLE = system_table; }

    println!("Alethia os!");

    
    loop {
        
    }
}
