pub mod pic;
pub mod isr;
pub mod irq;

pub fn init() {
    unsafe {
        pic::init_pic();
        pic::unmask_irq(0);
        // pic::unmask_irq(1);
        isr::install_isr();
    }
    x86_64::instructions::interrupts::enable();
}