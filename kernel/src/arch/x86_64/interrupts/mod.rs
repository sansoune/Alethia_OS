pub mod pic;

pub fn init() {
    unsafe {
        pic::init_pic();
        pic::unmask_irq(0);
        // pic::unmask_irq(1);
    }
    x86_64::instructions::interrupts::enable();
}