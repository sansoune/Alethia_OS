#![no_std]
#![no_main]


use core::panic::PanicInfo;
use alethia_os::uefi::{Handle, SystemTable};
use alethia_os::println;

// #[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[no_mangle]
pub extern "efiapi" fn efi_main(handle: Handle, system_table: *const SystemTable) {
    alethia_os::console::init(system_table);

    println!("Alethia os!");


    loop {
        
    }
}
