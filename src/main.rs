#![no_std]
#![no_main]


use core::panic::PanicInfo;
use alethia_os::uefi::{ImageHandle, SystemTable};

// #[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "efiapi" fn efi_main(handle: ImageHandle, system_table: *const SystemTable) {
    let string =  "helllo world!\n\r";
    let output_protocol = unsafe { &*(*system_table).output };

    for character in string.chars() {
        let mut buffer:[u16;1] = [0];
        let utf16 = character.encode_utf16(&mut buffer);
        
        (output_protocol.output_string)(output_protocol, &utf16[0]);
        
    }

    let string_arr = ['h' as u16, 'i' as u16, '!' as u16, '\n' as u16, '\0' as u16];

    
    (output_protocol.output_string)(output_protocol, &string_arr[0]);
    


    loop {
        
    }
}
