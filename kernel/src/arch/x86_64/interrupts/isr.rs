use crate::arch::x86_64::idt::set_idt_gate;
use core::arch::asm;


#[repr(C, packed)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

#[repr(C, packed)]
pub struct ExceptioStackFrame {
    pub error_code: u64,
    pub frame: InterruptStackFrame,
}

const EXCEPTION_MESSAGES: [&str; 32] = [
    "Division By Zero",
    "Debug",
    "Non Maskable Interrupt",
    "Breakpoint",
    "Into Detected Overflow",
    "Out of Bounds",
    "Invalid Opcode",
    "No Coprocessor",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Bad TSS",
    "Segment Not Present",
    "Stack Fault",
    "General Protection Fault",
    "Page Fault",
    "Unknown Interrupt",
    "Coprocessor Fault",
    "Alignment Check",
    "Machine Check",
    "SIMD Floating-Point Exception",
    "Virtualization Exception",
    "Control Protection Exception",
    "Reserved",
    "Hypervisor Injection Exception",
    "VMM Communication Exception",
    "Security Exception",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
];

macro_rules! isr_no_error_code {
    ($name:ident, $num:expr) => {
        #[naked]
        pub extern "x86-interrupt" fn $name() {
            unsafe {
                asm!(
                    "push 0",
                    "push {}",
                    "jmp isr_common",
                    const $num,
                    options(noreturn)
                );
            }
        }
    };
}

macro_rules! isr_error_code {
    ($name:ident, $num:expr) => {
        #[naked]
        pub "x86-interrupt" fn $name() {
            unsafe {
                asm!(
                    "push {}",
                    "jmp isr_common",
                    const $num,
                    options(noreturn)
                );
            }
        }
    };
}

// Define ISRs
isr_no_error_code!(isr0, 0);
isr_no_error_code!(isr1, 1);
isr_no_error_code!(isr2, 2);
isr_no_error_code!(isr3, 3);
isr_no_error_code!(isr4, 4);
isr_no_error_code!(isr5, 5);
isr_no_error_code!(isr6, 6);
isr_no_error_code!(isr7, 7);
isr_error_code!(isr8, 8);
isr_no_error_code!(isr9, 9);
isr_error_code!(isr10, 10);
isr_error_code!(isr11, 11);
isr_error_code!(isr12, 12);
isr_error_code!(isr13, 13);
isr_error_code!(isr14, 14);
isr_no_error_code!(isr15, 15);
isr_no_error_code!(isr16, 16);
isr_error_code!(isr17, 17);
isr_no_error_code!(isr18, 18);
isr_no_error_code!(isr19, 19);
isr_no_error_code!(isr20, 20);
isr_error_code!(isr21, 21);
isr_no_error_code!(isr22, 22);
isr_no_error_code!(isr23, 23);
isr_no_error_code!(isr24, 24);
isr_no_error_code!(isr25, 25);
isr_no_error_code!(isr26, 26);
isr_no_error_code!(isr27, 27);
isr_no_error_code!(isr28, 28);
isr_no_error_code!(isr29, 29);
isr_error_code!(isr30, 30);
isr_no_error_code!(isr31, 31);


#[no_mangle]
pub extern "C" fn common_isr_handler(stack_frame: &ExceptioStackFrame, int_num: u64) {
    if int_num < 32 {
        println!("Received exception: {} (error code: 0x{:x})", EXCEPTION_MESSAGES[int_num as usize], stack_frame.error_code);
        println!("At address: 0x{:x}", stack_frame.frame.instruction_pointer);
        println!("CPU Flags: 0x{:x}", stack_frame.frame.cpu_flags);
        println!("Stack Pointer: 0x{:x}", stack_frame.frame.stack_pointer);
        panic!();
    } else {
        println!("Received interrupt: {}", int_num);
    }
}


pub fn install_isr() {
    unsafe {
        for i in 0..32 {
            if i == 8 || (i >= 10 && i <= 14) || i = 17 || i = 21 {
                set_idt_gate(i, exception_handler_with_error_code as u64, 0, 0x8E);
            } else {
                set_idt_gate(i, exeption_handler as u64, 0, 0x8E);
            }
        }
    }
}

pub fn register_isr(index: usize, handler: extern "x86-interrupt" fn()) {
    unsafe {
        set_idt_gate(index, handler as u64, 0, );
    }
}