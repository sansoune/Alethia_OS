#![no_std]
#![no_main]

use bootloader::{frame_buffer::{get_frame_buffer, write_to_frame_buffer}, load_file::{load_file, open_file}, BootInfo};
use core::panic::PanicInfo;
use helpers::init;
use table::{boot::MemoryType, Boot, SystemTable};
use uefi::*;
use xmas_elf::ElfFile;
// use core::arch::asm;



#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // initilization
    init(&mut system_table).expect("Failed to initialize");

    //setuping the screen 
    system_table
        .stdout()
        .clear()
        .expect("Failed to reset stdout");
    println!("alethia os is booting...");

    let elf = {
        let mut file = open_file(&system_table, cstr16!("kernel.elf"));
        let buf = load_file(&system_table, &mut file);
        ElfFile::new(buf).expect("failed to parse ELF")
    };

    unsafe {
        ENTRY = elf.header.pt2.entry_point() as usize;
    }
    
    // //loading font
    // let mut font_file = load_file(&system_table, cstr16!("font.psf"));
    // let mut small_buffer = [0u8; 128];
    // let font_info =  font_file.get_info::<FileInfo>(&mut small_buffer).expect("Failed to get font file info");

    // //allocating memory
    // let font_size = font_info.file_size() as usize;
    // let font_memory = system_table.boot_services().allocate_pool(table::boot::MemoryType::LOADER_DATA, font_size).expect("Failed to allocate memory for font");

    // //reading font file into memory
    // let font_memory_slice = unsafe { core::slice::from_raw_parts_mut(font_memory, font_size) };
    // font_file.read(font_memory_slice).expect("Faled to read font file");

    
    let mut frame_buffer = get_frame_buffer(&system_table).expect("failed to get frame buffer");
    write_to_frame_buffer(&mut frame_buffer, 100, 1000, 0x00FF00);

    let (_, mmap) = system_table.exit_boot_services(MemoryType::LOADER_DATA);

    let bootinfo = BootInfo {
        framebuffer: frame_buffer,
    };

    let entry: extern "C" fn(&BootInfo) -> ! = unsafe { core::mem::transmute(ENTRY) };
    entry(&bootinfo);

    Status::SUCCESS
}

static mut ENTRY: usize = 0;