
use crate::uefi::{TextOutputProtocol, SystemTable};
use core::fmt::{self, Write};


pub static mut SYSTEM_TABLE: *const SystemTable = 0 as *const SystemTable;

pub fn init(system_table: *const SystemTable) {
    unsafe{ SYSTEM_TABLE = system_table}
}


#[allow(dead_code)]
struct Output {
    output_protocol: *const TextOutputProtocol,
}

impl Write for Output {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let output_protocol = unsafe { &*self.output_protocol };

        let mut result = Ok(());

        for character in s.chars() {
            let mut buffer: [u16; 2] = [0, 0];
            let utf16 = character.encode_utf16(&mut buffer);
            for u16_char in utf16.iter() {
                let status = (output_protocol.output_string)(output_protocol, u16_char);
                if status != 0 {
                    result = Err(fmt::Error);
                    break;
                }
            }
        }
        result
    }

}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (
        $crate::console::_print(format_args!($($arg)*));
    );
}

#[macro_export]
macro_rules! println {
    () => {
        &crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let output_protocol = unsafe { &*(*SYSTEM_TABLE).output };
    let mut output = Output {
        output_protocol,
    };
    output.write_fmt(args).unwrap();
}
