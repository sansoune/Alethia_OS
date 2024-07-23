pub mod pic;

pub fn init() {
    unsafe { pic::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}