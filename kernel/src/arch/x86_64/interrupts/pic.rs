// use lazy_static::lazy_static;
// use pic8259::ChainedPics;
// use spin;

// pub const PIC_1_OFFSET: u8 = 32;
// pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;



// lazy_static! {
//     pub static ref PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe {
//         ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
//     });
// }

// pub fn init_pic() {
//     unsafe {
//         PICS.lock().initialize();
//     }
// }

// pub fn unmask_irq(irq: u8) {
//     unsafe {
//         let mut pics = PICS.lock();
//         let [mask1, mask2] = pics.read_masks();

//         if irq < 8 {
//             // IRQ belongs to PIC1
//             let irq_mask = !(1 << irq);
//             pics.write_masks(mask1 & irq_mask, mask2);
//         } else {
//             // IRQ belongs to PIC2
//             let irq_mask = !(1 << (irq - 8)); // Adjust shift for PIC2
//             pics.write_masks(mask1, mask2 & irq_mask);
//         }
//     }
// }

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

pub unsafe fn unmask_timer_interrupt() {
    let mut mask = inb(PIC1_DATA);
    mask &= !(1 << 0);  // Clear bit 0 to unmask IRQ0 (timer)
    outb(PIC1_DATA, mask);
}

pub fn send_eoi() {
    unsafe {
        outb(PIC1_COMMAND, 0x20);
    }
}