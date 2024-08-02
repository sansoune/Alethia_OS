use crate::{inb, outb, io_wait};

const PIC1: u16 = 0x20;
const PIC2: u16 = 0xA0;
const PIC1_COMMAND: u16 = PIC1;
const PIC1_DATA: u16 = PIC1 + 1;
const PIC2_COMMAND: u16 = PIC2;
const PIC2_DATA: u16 = PIC2 + 1;


pub unsafe fn init_pic() {
    // Save masks
    let mask1 = inb(PIC1_DATA);
    let mask2 = inb(PIC2_DATA);

    // Start initialization sequence
    outb(PIC1_COMMAND, 0x11);
    io_wait();
    outb(PIC2_COMMAND, 0x11);
    io_wait();

    // Set vector offsets
    outb(PIC1_DATA, 32); // Start of hardware interrupts (32)
    io_wait();
    outb(PIC2_DATA, 40); // Start of hardware interrupts 2 (40)
    io_wait();

    // Set up cascading
    outb(PIC1_DATA, 4);
    io_wait();
    outb(PIC2_DATA, 2);
    io_wait();

    // Set 8086 mode
    outb(PIC1_DATA, 0x01);
    io_wait();
    outb(PIC2_DATA, 0x01);
    io_wait();

    // Restore saved masks
    outb(PIC1_DATA, mask1);
    outb(PIC2_DATA, mask2);
}

pub unsafe fn unmask_irq(irq: u8) {
    if irq > 15 {
        // Invalid IRQ number
        return;
    }

    let (port, irq) = if irq < 8 {
        (PIC1_DATA, irq)
    } else {
        (PIC2_DATA, irq - 8)
    };

    let mut mask = inb(port);
    mask &= !(1 << irq);
    outb(port, mask);
}

pub fn send_eoi() {
    unsafe {
        outb(PIC1_COMMAND, 0x20);
    }
}