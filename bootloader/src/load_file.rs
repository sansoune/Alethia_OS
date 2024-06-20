use uefi::{proto::{loaded_image::LoadedImage, media::{file::{File, FileAttribute, FileInfo, FileMode, FileType, RegularFile}, fs::SimpleFileSystem}}, table::{boot::{AllocateType, MemoryType}, Boot, SystemTable}, CStr16};




pub fn open_file(system_table: &SystemTable<Boot>, path: &CStr16) -> RegularFile {
    let boot_services = system_table.boot_services();

    let loaded_image = boot_services.open_protocol_exclusive::<LoadedImage>(boot_services.image_handle()).unwrap();
    let mut file_system = boot_services.open_protocol_exclusive::<SimpleFileSystem>(loaded_image.device().expect("Failed to get device")).expect("failed to get FileSystem");
    

    let mut directory = file_system.open_volume().expect("failed to open volume");
    let file = directory.open(path, FileMode::Read, FileAttribute::READ_ONLY).expect("failed to open file");
    
    match file.into_type().expect("failed to into type") {
        FileType::Regular(regular) => regular,
        _ => panic!("invalid file type")
    }
}

pub fn load_file(system_table: &SystemTable<Boot>, file: &mut RegularFile) -> &'static mut [u8] {
    let mut info_buf = [0u8; 0x100];
    let info = file.get_info::<FileInfo>(&mut info_buf).expect("failed to get file info");
    let pages = info.file_size() as usize / 0x1000 + 1;
    let mem_start = system_table.boot_services().allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages).expect("failed to allocate pages");
    let buf = unsafe { core::slice::from_raw_parts_mut(mem_start as *mut u8, pages * 0x1000) };
    let len = file.read(buf).expect("failed to read file");
    &mut buf[..len]
}
