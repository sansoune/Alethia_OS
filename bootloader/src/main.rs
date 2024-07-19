#![no_std]
#![no_main]

use core::ops::Add;
use core::panic::PanicInfo;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::file::{File, FileAttribute, FileHandle, FileInfo, FileMode, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::{AllocateType, MemoryType};
use uefi::{prelude::*, println, CStr16};
use bootloader::frame_buffer::{get_frame_buffer, write_to_frame_buffer, FrameBuffer};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[repr(C)]
struct ElfHeader {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
struct ElfProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

#[repr(C)]
struct ElfSectionHeader {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

#[repr(C)]
struct ElfRelocationEntry {
    r_offset: u64,
    r_info: u64,
    r_add: u64,
}

const R_X86_64_RELATIVE: u32 = 8;


fn load_file(bs: &BootServices, kernel_file: &mut RegularFile) -> (u64, u64) {
    
    let mut file_info_buffer = [0u8; 128];
    let file_info = kernel_file.get_info::<FileInfo>(&mut file_info_buffer).unwrap();
    let file_size = file_info.file_size() as usize;

    let file_pages = (file_size + 0xFFF) / 0x1000;
    let file_buffer = bs.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, file_pages).unwrap();
    let file_slice = unsafe {
        core::slice::from_raw_parts_mut(file_buffer as *mut u8, file_size)
    };

    kernel_file.read(file_slice).unwrap();

    println!("file read to buffer");  

    let elf_header: &ElfHeader = unsafe {
        &*(file_buffer as *const ElfHeader)
    };


    
    let ph_offset = elf_header.e_phoff; //the program is stuch at this line
    let ph_size = elf_header.e_phentsize as usize;
    let ph_count = elf_header.e_phnum as usize;
    let e_entry = elf_header.e_entry;
    

    let mut max_addr = 0u64;
    for i in 0..ph_count {
        let ph_addr = file_buffer + ph_offset as u64 + (i * ph_size) as u64;
        let ph: &ElfProgramHeader = unsafe { &*(ph_addr as *const ElfProgramHeader) };

        if ph.p_type == 1 && ph.p_vaddr != 0 {
            max_addr = max_addr.max(ph.p_vaddr + ph.p_memsz);
        }
    }

    let kernel_pages = (max_addr as usize + 0xFFF) / 0x1000;
    let kernel_buffer = bs.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, kernel_pages).unwrap();

    for i in 0..ph_count {
        let ph_addr = file_buffer + ph_offset as u64 + (i * ph_size) as u64;
        let ph = unsafe {
            &*(ph_addr as *const ElfProgramHeader)
        };

        if ph.p_type == 1 {
            let src = unsafe { (file_buffer as *const u8).add(ph.p_offset as usize) };
            let dst = unsafe { (kernel_buffer as *mut u8).add(ph.p_vaddr as usize) };
            unsafe {
                core::ptr::copy_nonoverlapping(src, dst, ph.p_filesz as usize);
                core::ptr::write_bytes(dst.add(ph.p_filesz as usize), 0, (ph.p_memsz - ph.p_filesz) as usize);
            }

        }
    }

    let sh_offset = elf_header.e_shoff as usize;
    let sh_size = elf_header.e_shentsize as usize;
    let sh_count = elf_header.e_shnum as usize;

    for i in 0..sh_count {
        let sh_addr = file_buffer + sh_offset as u64 + (i * sh_size) as u64;
        let sh: &ElfSectionHeader = unsafe {
            &*(sh_addr as *const ElfSectionHeader)
        };

        if sh.sh_type == 4 {
            let reloc_entries_count = sh.sh_size /sh.sh_entsize;
            for j in 0..reloc_entries_count {
                let relooc_addr = sh.sh_offset + j * sh.sh_entsize;
                let reloc = unsafe {
                    &*(file_buffer.add(relooc_addr) as *const ElfRelocationEntry)
                };

                let reloc_type = (reloc.r_info & 0xFFFFFFFF) as u32;

                if reloc_type == R_X86_64_RELATIVE {
                    let addr = kernel_buffer + reloc.r_offset;
                    unsafe {
                        let ptr = addr as *mut u64;
                        *ptr = *ptr + kernel_buffer + reloc.r_add;
                    }
                }
            }
        }
    }

    unsafe { bs.free_pages(file_buffer, file_pages).unwrap() };

    println!("entry: {}", e_entry);

    (kernel_buffer, e_entry)
}

#[entry]
fn main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut st).unwrap();
    st
        .stdout()
        .clear()
        .expect("Failed to reset stdout");
    println!("alethia os is booting...");
    

    let kernel_add;
    let entry_point_offset;

    {
        
        let bs = &st.boot_services();
        
        let loaded_image = bs.open_protocol_exclusive::<LoadedImage>(image).unwrap();
        let mut fs = bs
            .open_protocol_exclusive::<SimpleFileSystem>(loaded_image.device().unwrap())
            .unwrap();
        let mut root = fs.open_volume().unwrap();

        

        let mut kernel_file = root
            .open(
                cstr16!("kernel.elf"),
                FileMode::Read,
                FileAttribute::empty(),
            )
            .unwrap()
            .into_regular_file()
            .unwrap();
        
        (kernel_add, entry_point_offset) = load_file(bs, &mut kernel_file);
        
    }

    let fb = get_frame_buffer(&st).expect("couldn't load the frame buffer");
    write_to_frame_buffer(&fb, 100, 100, 0xFF0000);
    println!("entry point offset: {}", entry_point_offset);

    // let (_runtime, _) = st.exit_boot_services(MemoryType::LOADER_DATA);
    
    let entry_point = kernel_add + entry_point_offset;

    let kernel_main: extern "sysv64" fn(&FrameBuffer) -> ! =
        unsafe { core::mem::transmute(entry_point) };
    kernel_main(&fb);

    Status::SUCCESS
}
