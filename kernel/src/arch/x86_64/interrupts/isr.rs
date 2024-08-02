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

#[naked]
pub extern "C" fn isr_common() {
    unsafe {
        asm!(
            "push rax",
            "push rbx",
            "push rcx",
            "push rdx",
            "push rsi",
            "push rdi",
            "push rbp",
            "push r8",
            "push r9",
            "push r10",
            "push r11",
            "push r12",
            "push r13",
            "push r14",
            "push r15",

            "mov rdi, rsp",
            "mov rsi, [rsp + 15*8]", // Get interrupt number
            "call common_isr_handler",

            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rbp",
            "pop rdi",
            "pop rsi",
            "pop rdx",
            "pop rcx",
            "pop rbx",
            "pop rax",

            "add rsp, 16",

            "iretq",
            options(noreturn)
        )
    }
}


pub fn install_isr() {
    unsafe {
        set_idt_gate(0, isr0 as u64, 0, 0x8E);
        set_idt_gate(1, isr1 as u64, 0, 0x8E);
        set_idt_gate(2, isr2 as u64, 0, 0x8E);
        set_idt_gate(3, isr3 as u64, 0, 0x8E);
        set_idt_gate(4, isr4 as u64, 0, 0x8E);
        set_idt_gate(5, isr5 as u64, 0, 0x8E);
        set_idt_gate(6, isr6 as u64, 0, 0x8E);
        set_idt_gate(7, isr7 as u64, 0, 0x8E);
        set_idt_gate(8, isr8 as u64, 0, 0x8E);
        set_idt_gate(9, isr9 as u64, 0, 0x8E);
        set_idt_gate(10, isr10 as u64, 0, 0x8E);
        set_idt_gate(11, isr11 as u64, 0, 0x8E);
        set_idt_gate(12, isr12 as u64, 0, 0x8E);
        set_idt_gate(13, isr13 as u64, 0, 0x8E);
        set_idt_gate(14, isr14 as u64, 0, 0x8E);
        set_idt_gate(15, isr15 as u64, 0, 0x8E);
        set_idt_gate(16, isr16 as u64, 0, 0x8E);
        set_idt_gate(17, isr17 as u64, 0, 0x8E);
        set_idt_gate(18, isr18 as u64, 0, 0x8E);
        set_idt_gate(19, isr19 as u64, 0, 0x8E);
        set_idt_gate(20, isr20 as u64, 0, 0x8E);
        set_idt_gate(21, isr21 as u64, 0, 0x8E);
        set_idt_gate(22, isr22 as u64, 0, 0x8E);
        set_idt_gate(23, isr23 as u64, 0, 0x8E);
        set_idt_gate(24, isr24 as u64, 0, 0x8E);
        set_idt_gate(25, isr25 as u64, 0, 0x8E);
        set_idt_gate(26, isr26 as u64, 0, 0x8E);
        set_idt_gate(27, isr27 as u64, 0, 0x8E);
        set_idt_gate(28, isr28 as u64, 0, 0x8E);
        set_idt_gate(29, isr29 as u64, 0, 0x8E);
        set_idt_gate(30, isr30 as u64, 0, 0x8E);
        set_idt_gate(31, isr31 as u64, 0, 0x8E);
    }
}

pub fn register_isr(index: usize, handler: extern "x86-interrupt" fn()) {
    unsafe {
        set_idt_gate(index, handler as u64, 0, 0x8E);
    }
}