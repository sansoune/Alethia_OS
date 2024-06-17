use uefi::{proto::{loaded_image::LoadedImage, media::{file::{File, FileAttribute, FileHandle, FileMode}, fs::SimpleFileSystem}}, table::{Boot, SystemTable}, CStr16};




pub fn load_file(system_table: &SystemTable<Boot>, path: &CStr16) -> FileHandle{
    let boot_services = system_table.boot_services();

    let loaded_image = boot_services.open_protocol_exclusive::<LoadedImage>(boot_services.image_handle()).unwrap();
    let mut file_system = boot_services.open_protocol_exclusive::<SimpleFileSystem>(loaded_image.device().expect("Failed to get device")).unwrap();

    let mut directory = file_system.open_volume().unwrap();
    directory.open(path, FileMode::Read, FileAttribute::READ_ONLY).unwrap()
}