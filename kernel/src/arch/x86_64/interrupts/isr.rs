use crate::arch::x86_64::idt::set_idt_gate;

extern "x86-interrupt" fn commun_handler() {

}

pub fn install_isr() {
    unsafe {
        for i in 0..50 {
            set_idt_gate(i, commun_handler);
        }
    }
}

pub fn register_isr(index: usize, handler: extern "x86-interrupt" fn()) {
    unsafe {
        set_idt_gate(index, handler);
    }
}