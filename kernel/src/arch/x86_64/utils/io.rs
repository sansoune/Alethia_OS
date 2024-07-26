pub unsafe fn io_wait() {
    outb(0x80, 0);
}

pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    core::arch::asm!("in al, dx", out("al") result, in("dx") port);
    result
}

pub unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value);
}

