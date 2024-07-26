use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::{arch::x86_64::interrupts::pic::send_eoi, print, println};
// use super::gdt::DOUBLE_FAULT_IST_INDEX;

// #[derive(Debug, Clone, Copy)]
// #[repr(u8)]
// pub enum InterruptIndex {
//     Timer = PIC_1_OFFSET,
// }

// impl InterruptIndex {
//     fn as_u8(self) -> u8 {
//         self as u8
//     }

//     fn as_usize(self) -> usize {
//         usize::from(self.as_u8())
//     }
// }

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(0);   
        }
        idt[32].set_handler_fn(timer_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXEPTION: BREAKPOINT\n {:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    print!(".");
    send_eoi();
    // unsafe { PICS.lock().notify_end_of_interrupt(32) };
}