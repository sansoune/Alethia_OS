use crate::elf::*;
use crate::font::{Font, PSF1Header};
use core::ops::Add;
use uefi::println;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode, FileType, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::{AllocateType, MemoryType};
use uefi::table::{Boot, SystemTable};
use uefi::CStr16;

pub fn open_file(system_table: &SystemTable<Boot>, path: &CStr16) -> RegularFile {
    let boot_services = system_table.boot_services();

    let loaded_image = boot_services
        .open_protocol_exclusive::<LoadedImage>(boot_services.image_handle())
        .unwrap();
    let mut file_system = boot_services
        .open_protocol_exclusive::<SimpleFileSystem>(
            loaded_image.device().expect("Failed to get device"),
        )
        .expect("failed to get FileSystem");

    let mut directory = file_system.open_volume().expect("failed to open volume");
    let file = directory
        .open(path, FileMode::Read, FileAttribute::READ_ONLY)
        .expect("failed to open file");

    match file.into_type().expect("failed to into type") {
        FileType::Regular(regular) => regular,
        _ => panic!("invalid file type"),
    }
}

const R_X86_64_RELATIVE: u32 = 8;

pub fn load_kernel(st: &SystemTable<Boot>, kernel_file: &mut RegularFile) -> (u64, u64) {
    let bs = st.boot_services();

    let mut file_info_buffer = [0u8; 128];
    let file_info = kernel_file
        .get_info::<FileInfo>(&mut file_info_buffer)
        .unwrap();
    let file_size = file_info.file_size() as usize;

    let file_pages = (file_size + 0xFFF) / 0x1000;
    let file_buffer = bs
        .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, file_pages)
        .unwrap();
    let file_slice = unsafe { core::slice::from_raw_parts_mut(file_buffer as *mut u8, file_size) };

    kernel_file.read(file_slice).unwrap();

    println!("file read to buffer");

    let elf_header: &ElfHeader = unsafe { &*(file_buffer as *const ElfHeader) };

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
    let kernel_buffer = bs
        .allocate_pages(
            AllocateType::AnyPages,
            MemoryType::LOADER_DATA,
            kernel_pages,
        )
        .unwrap();

    for i in 0..ph_count {
        let ph_addr = file_buffer + ph_offset as u64 + (i * ph_size) as u64;
        let ph = unsafe { &*(ph_addr as *const ElfProgramHeader) };

        if ph.p_type == 1 {
            let src = unsafe { (file_buffer as *const u8).add(ph.p_offset as usize) };
            let dst = unsafe { (kernel_buffer as *mut u8).add(ph.p_vaddr as usize) };
            unsafe {
                core::ptr::copy_nonoverlapping(src, dst, ph.p_filesz as usize);
                core::ptr::write_bytes(
                    dst.add(ph.p_filesz as usize),
                    0,
                    (ph.p_memsz - ph.p_filesz) as usize,
                );
            }
        }
    }

    let sh_offset = elf_header.e_shoff as usize;
    let sh_size = elf_header.e_shentsize as usize;
    let sh_count = elf_header.e_shnum as usize;

    for i in 0..sh_count {
        let sh_addr = file_buffer + sh_offset as u64 + (i * sh_size) as u64;
        let sh: &ElfSectionHeader = unsafe { &*(sh_addr as *const ElfSectionHeader) };

        if sh.sh_type == 4 {
            let reloc_entries_count = sh.sh_size / sh.sh_entsize;
            for j in 0..reloc_entries_count {
                let relooc_addr = sh.sh_offset + j * sh.sh_entsize;
                let reloc =
                    unsafe { &*(file_buffer.add(relooc_addr) as *const ElfRelocationEntry) };

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

pub fn load_font(st: &SystemTable<Boot>, font_file: &mut RegularFile) -> Font {
    let bs = st.boot_services();

    let mut font_file_buffer = [0u8; 128];
    let font_info = font_file.get_info::<FileInfo>(&mut font_file_buffer).expect("failed getting file info");
    let font_file_size = font_info.file_size() as usize;
    let font_memory = bs.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, (font_file_size + 0xFFF) / 0x1000).expect("couldn't allocate font memory");

    let font_data = unsafe {
        core::slice::from_raw_parts_mut(font_memory as *mut u8, font_file_size)
    };
    font_file.read(font_data).expect("failed to read font file");

    let header = unsafe {
        &*(font_data.as_ptr() as *const PSF1Header)
    };

    if header.magic != 0x0436 && header.magic != 0x3642 {
        panic!("invalid psf1 magic number");
    }

    let glyphs = &font_data[4..];

    Font::new(*header, glyphs)
}
