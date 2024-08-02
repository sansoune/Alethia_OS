use crate::{arch::x86_64::interrupts::pic::send_eoi, print};

extern "x86-interrupt" fn timer_interrupt_handler()
{
    print!(".");
    send_eoi(0);
}
use crate::arch::x86_64::interrupts::isr;

pub fn init() {
    isr::register_isr(32, timer_interrupt_handler);
}