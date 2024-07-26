pub mod pic;

pub fn init() {
    unsafe {
        pic::init_pic();
        pic::unmask_timer_interrupt();
    }
    x86_64::instructions::interrupts::enable();
}