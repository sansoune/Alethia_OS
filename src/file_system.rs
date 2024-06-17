// // use crate::uefi::*;
// use uefi::proto::media::fs::SimpleFileSystem
// use core::ffi::c_void;
// use core::ptr;

// const MAX_HANDLES: usize = 64;
// const MAX_FILE_INFO_SIZE: usize = 512;
// const MAX_MEMORY_MAP_SIZE: usize = 8192;
// const KERNEL_PATH: &str = "\\EFI\\BOOT\\kernel.elf";

// pub fn utf16_from_str(s: &str) -> [u16; 260] {
//     let mut buffer = [0u16; 260];
//     let mut i = 0;
//     for c in s.encode_utf16() {
//         if i < buffer.len() - 1 {
//             buffer[i] = c;
//             i += 1;
//         } else {
//             break;
//         }
//     }
//     buffer
// }

// pub fn load_kernel(image_handle: Handle, system_table: *const SystemTable) {
//     let boot_services = unsafe { &*(*system_table).boot };

//     unsafe {
//         let mut buffer_size = 0;
//         let status = (boot_services.locate_handle)(
//             LocateSearchType::ByProtocol,
//             &mut SIMPLE_FILE_SYSTEM_GUID as *mut GUID,
//             ptr::null_mut(),
//             &mut buffer_size,
//             ptr::null_mut(),
//         );

//         if status != Status::BufferTooSmall {
//             panic!("failed to locate handles: {:?}", status);
//         }

//         let handles: [Handle; MAX_HANDLES] = [ptr::null_mut(); MAX_HANDLES];
//         let mut handle_count = buffer_size / core::mem::size_of::<Handle>();
//         if handle_count > MAX_HANDLES {
//             handle_count = MAX_HANDLES;
//         }

//         let status = (boot_services.locate_handle)(
//             LocateSearchType::ByProtocol,
//             &mut SIMPLE_FILE_SYSTEM_GUID as *mut GUID,
//             ptr::null_mut(),
//             &mut buffer_size,
//             ptr::null_mut(),
//         );

//         if status != Status::Success {
//             panic!("Failed to locate handles: {:?}", status);
//         }

//         let mut simple_fs: *mut c_void = ptr::null_mut();
//         let status = (boot_services.handle_protocol)(
//             handles[0],
//             &mut SIMPLE_FILE_SYSTEM_GUID as *mut GUID,
//             &mut simple_fs as *mut *mut c_void,
//         );

//         if status != Status::Success {
//             panic!("Failed to open SimpleFileSystem protocol: {:?}", status);
//         }

//         let simple_fs = simple_fs as *mut FileSystemProtocol;
//         let mut root: *mut FileProtocol = core::ptr::null_mut();
//         let status = ((*simple_fs).open_volume)(simple_fs, &mut root);

//         if status != Status::Success {
//             panic!("Failed to open volume: {:?}", status);
//         }

//         let kernel_path = utf16_from_str(KERNEL_PATH);
//         let mut kernel_file: *mut FileProtocol = core::ptr::null_mut();
//         let status = ((*root).open)(
//             root,
//             &mut kernel_file,
//             kernel_path.as_ptr(),
//             EFI_FILE_MODE_READ,
//             0,
//         );

//         if status != Status::Success {
//             panic!("Failed to open kernel file: {:?}", status);
//         }

//         let mut file_info_size = 0;
//         let status = ((*kernel_file).get_info)(
//             kernel_file,
//             &EFI_FILE_INFO_ID as *const GUID,
//             &mut file_info_size,
//             core::ptr::null_mut(),
//         );

//         if status != Status::BufferTooSmall {
//             panic!("Failed to get file info size: {:?}", status);
//         }

//         let mut file_info: [u8; MAX_FILE_INFO_SIZE] = [0; MAX_FILE_INFO_SIZE];
//         if file_info_size > MAX_FILE_INFO_SIZE {
//             panic!("File info size too large");
//         }

//         let status = ((*kernel_file).get_info)(
//             kernel_file,
//             &EFI_FILE_INFO_ID as *const GUID,
//             &mut file_info_size,
//             file_info.as_mut_ptr() as *mut c_void,
//         );

//         if status != Status::Success {
//             panic!("Failed to get file info: {:?}", status);
//         }

//         let file_info = &*(file_info.as_ptr() as *const FileInfo);
//         let kernel_file_size = file_info.file_size as usize;

//         let pages = (kernel_file_size + 0xFFF) / 0x1000;
//         let mut kernel_buffer: PhysicalAddress = 0;
//         let status = (boot_services.allocate_pages)(
//             AllocateType::AllocateAnyPages,
//             LOADER_DATA,
//             pages,
//             &mut kernel_buffer,
//         );

//         if status != Status::Success {
//             panic!("Failed to allocate memory for kernel: {:?}", status);
//         }

//         let kernel_buffer_ptr = kernel_buffer as *mut c_void;
//         let mut remaining_size = kernel_file_size;
//         let status = ((*kernel_file).read)(kernel_file, &mut remaining_size, kernel_buffer_ptr);

//         if status != Status::Success {
//             panic!("Failed to read kernel file: {:?}", status);
//         }

//         let status = ((*kernel_file).close)(kernel_file);

//         if status != Status::Success {
//             panic!("Failed to close kernel file: {:?}", status);
//         }

//         let mut memory_map_size = MAX_MEMORY_MAP_SIZE;
//         let mut memory_map: [u8; MAX_MEMORY_MAP_SIZE] = [0; MAX_MEMORY_MAP_SIZE];
//         let mut map_key = 0;
//         let mut descriptor_size = 0;
//         let mut descriptor_version = 0;

//         let status = (boot_services.get_memory_map)(
//             &mut memory_map_size,
//             memory_map.as_mut_ptr() as *mut MemoryDescriptor,
//             &mut map_key,
//             &mut descriptor_size,
//             &mut descriptor_version,
//         );

//         if status != Status::Success {
//             panic!("Failed to get memory map: {:?}", status);
//         }

//         let status = (boot_services.exit_boot_services)(image_handle, map_key);

//         if status != Status::Success {
//             panic!("Failed to exit boot services: {:?}", status);
//         }

//         let kernel_entry: extern "C" fn() -> ! =
//             core::mem::transmute(kernel_buffer as *const ());
//         kernel_entry();
//     };
// }
