use crate::{arch::x86_64::interrupts::pic::send_eoi, print, println};
use core::mem::size_of;
use core::arch::asm;

use x86_64::structures::idt::HandlerFunc;

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,
    segment_selector: u16,
    ist: u8,
    type_attributes: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u64,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry {
    offset_low: 0,
    segment_selector: 0,
    ist: 0,
    type_attributes: 0,
    offset_mid: 0,
    offset_high: 0,
    reserved: 0,
}; 256];

static mut IDT_PTR: IdtPtr = IdtPtr {
    limit: 0,
    base: 0,
};

unsafe fn set_idt_gate(index: usize, handler: extern "x86-interrupt" fn()) {
    let addr = handler as u64;
    IDT[index] = IdtEntry {
        offset_low: addr as u16,
        segment_selector: 0x08, // Code segment selector
        ist: 0,
        type_attributes: 0x8E, // Present, DPL=0, Interrupt Gate
        offset_mid: (addr >> 16) as u16,
        offset_high: (addr >> 32) as u32,
        reserved: 0,
    }
}

unsafe fn setup_idt() {

    set_idt_gate(32, timer_interrupt_handler);

    IDT_PTR.limit = (size_of::<[IdtEntry; 256]>() - 1) as u16;
    IDT_PTR.base = IDT.as_ptr() as u64;

    asm!(
        "lidt [{0}]",
        in(reg) &IDT_PTR,
        options(readonly, nostack, preserves_flags)
    );
}

pub fn init_idt() {
    unsafe { setup_idt(); }
}

extern "x86-interrupt" fn timer_interrupt_handler()
{
    print!(".");
    send_eoi(0);
}