use core::arch::asm;
use core::mem::size_of;

use crate::println;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// Structures
#[derive(Clone, Copy)]
#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct GdtDescriptor {
    size: u16,
    offset: u64,
}

impl GdtEntry {
    const fn new(base: u32, limit: u32, access: u8, granularity: u8) -> Self {
        GdtEntry {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | (granularity & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

static mut GDT: [GdtEntry; 6] = [
    GdtEntry::new(0, 0, 0, 0),
    GdtEntry::new(0, 0xFFFFFFFF, 0x9A, 0xAF),
    GdtEntry::new(0, 0xFFFFFFFF, 0x92, 0xAF),
    GdtEntry::new(0, 0, 0, 0),
    GdtEntry::new(0, 0xFFFFFFFF, 0xFA, 0xAF),
    GdtEntry::new(0, 0xFFFFFFFF, 0xF2, 0xAF),
];

static mut GDT_DESCRIPTOR: GdtDescriptor = GdtDescriptor { size: 0, offset: 0 };

unsafe fn load_gdt() {
    GDT_DESCRIPTOR.size = (size_of::<[GdtEntry; 6]>() - 1) as u16;
    GDT_DESCRIPTOR.offset = GDT.as_ptr() as u64;

    asm!(
        "lgdt [{0}]",
        in(reg) &GDT_DESCRIPTOR,
        options(readonly, nostack, preserves_flags)
    );

    // Reload segment registers
    asm!(
        "push 0x08",
        "lea {tmp}, [1f + rip]",
        "push {tmp}",
        "retfq",
        "1:",
        "mov ax, 0x10",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        tmp = out(reg) _,
        options(nomem, nostack)
    );
}

pub fn init_gdt() {
    unsafe { load_gdt(); }
    // println!("called");
}
