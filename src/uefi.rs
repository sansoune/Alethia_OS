
pub type Handle  = *mut core::ffi::c_void;
pub type Event = *mut core::ffi::c_void;
pub type Status = usize;
pub type TPL = usize;
pub type MemoryType = usize;

pub type OutputString = extern "efiapi" fn(
    output_protocol:*const TextOutputProtocol,
    string :*const u16
    )-> Status;


#[repr(C)]
pub struct TextOutputProtocol{
    reset: u64,
    pub output_string: OutputString,
    test_output: u64,
    query_mode: u64,
    set_mode: u64,
    set_attribute: u64,
    pub clear_screen:  u64,
    set_cursor_position: u64,
    enable_cursor: u64,
    mode: u64,
}

#[repr(C)]
pub struct TableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
pub struct GUID {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

pub type PhysicalAddress = u64;
pub type VirtualAddress = u64;
pub type AllocateType = u32;
pub type CHAR16 = u16;
pub type BOOLEAN = bool;

#[repr(C)]
pub struct MemoryDescriptor {
    type_: MemoryType,
    physical_start: PhysicalAddress,
    virtual_start: VirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

#[repr(C)]
pub struct OpenProtocolInfoEntry {
    agent_handle: Handle,
    controller_handle: Handle,
    attributes: u32,
    open_count: u32,
}

#[repr(C)]
pub struct DevicePath {
    type_: u8,
    sub_type: u8,
    length: [u8; 2],
}

#[repr(C)]
pub struct ConfigurationTable {
    vendor_guid: GUID,
    vendor_table: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct TextInputProtocol {
    reset: extern "efiapi" fn(
        *const TextInputProtocol,
        extended_verification: BOOLEAN,
    ) -> Status,
    read_key_stroke: extern "efiapi" fn(
        *const TextInputProtocol,
        key: *mut InputKey,
    ) -> Status,
    wait_for_key: Event,
}

#[repr(C)]
pub struct RuntimeServices {
    hdr: TableHeader,

    get_time: extern "efiapi" fn(
        time: *mut Time,
        capabilities: *mut TimeCaâbilitties,
    ) -> Status,
    set_time: extern "efiapi" fn(
        time: *const Time,
    ) -> Status,
    get_wakeup_time: extern "efiapi" fn(
        enabled: *mut BOOLEAN,
        pending: *mut BOOLEAN,
        time: *mut Time,
    ) -> Status,
    set_wakeup_time: extern "efiapi" fn(
        enable: BOOLEAN,
        time: *const Time,
    ) -> Status,

    set_virtual_address_map: extern "efiapi" fn(
        memory_map_size: usize,
        descriptor_size: usize,
        descriptor_version: u32,
        virtual_map: *const MemoryDescriptor,
    ) -> Status,
    convert_pointer: extern "efiapi" fn(
        debug_disposition: usize,
        address: *mut *mut core::ffi::c_void,
    ) -> Status,

    get_variable: extern "efiapi" fn(
        variable_name: *const CHAR16,
        vendor_guid: *const GUID,
        attributes: *mut u32,
        data_size: *mut usize,
        data: *mut core::ffi::c_void,
    ) -> Status,
    get_next_variable_name: extern "efiapi" fn(
        variable_name_size: *mut usize,
        variable_name: *mut CHAR16,
        vendor_guid: *mut GUID,
    ) -> Status,
    set_variable: extern "efiapi" fn(
        variable_name: *const CHAR16,
        vendor_guid: *const GUID,
        attributes: u32,
        data_size: usize,
        data: *const core::ffi::c_void,
    ) -> Status,

    get_next_high_mono_count: extern "efiapi" fn(
        high_count: *mut u32,
    ) -> Status,
    reset_system: extern "efiapi" fn(
        reset_type: ResetType,
        reset_status: Status,
        data_size: usize,
        reset_data: *mut CHAR16,
    ) -> !,

    update_capsule: extern "efiapi" fn(
        capsule_header_array: *mut *mut CapsuleHeader,
        capsule_count: usize,
        scatter_gather_list: PhysicalAddress,
    ) -> Status,
    query_capsule_capabilities: extern "efiapi" fn(
        capsule_header_array: *mut *mut CapsuleHeader,
        capsule_count: usize,
        maximum_capsule_size: *mut u64,
        reset_type: *mut ResetType,
    ) -> Status,
    query_variable_info: extern "efiapi" fn(
        attributes: u32,
        maximum_variable_storage_size: *mut u64,
        remaining_variable_storage_size: *mut u64,
        maximum_variable_size: *mut u64,
    ) -> Status,
}

#[repr(C)]
pub struct Time {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    pad1: u8,
    nanosecond: u32,
    time_zone: i16,
    daylight: u8,
    pad2: u8,
}

#[repr(C)]
pub struct TimeCaâbilitties {
    resolution: u32,
    accuracy: u32,
    sets_to_zero: BOOLEAN,
}

#[repr(C)]
pub struct InputKey {
    scan_code: u16,
    unicode_char: CHAR16,
}

// Define other necessary enums and types
#[repr(C)]
pub enum TimerDelay {
    TimerCancel,
    TimerPeriodic,
    TimerRelative,
}

#[repr(C)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

#[repr(C)]
pub enum InterfaceType {
    NativeInterface,
}

#[repr(C)]
pub struct CapsuleHeader {
    capsule_guid: GUID,
    header_size: u32,
    flags: u32,
    capsule_image_size: u32,
}

#[repr(C)]
pub enum ResetType {
    EfiResetCold,
    EfiResetWarm,
    EfiResetShutdown,
    EfiResetPlatformSpecific,
}

#[repr(C)]
pub struct BootServices {
    hdr: TableHeader,

    // Task Priority Services
    raise_tpl: extern "efiapi" fn(new_tpl: TPL) -> TPL,
    restore_tpl: extern "efiapi" fn(old_tpl: TPL),

    // Memory Services
    allocate_pages: extern "efiapi" fn(
        type_: AllocateType,
        memory_type: MemoryType,
        pages: usize,
        memory: *mut PhysicalAddress,
    ) -> Status,
    free_pages: extern "efiapi" fn(
        memory: PhysicalAddress,
        pages: usize,
    ) -> Status,
    get_memory_map: extern "efiapi" fn(
        memory_map_size: *mut usize,
        memory_map: *mut MemoryDescriptor,
        map_key: *mut usize,
        descriptor_size: *mut usize,
        descriptor_version: *mut u32,
    ) -> Status,
    allocate_pool: extern "efiapi" fn(
        pool_type: MemoryType,
        size: usize,
        buffer: *mut *mut core::ffi::c_void,
    ) -> Status,
    free_pool: extern "efiapi" fn(buffer: *mut core::ffi::c_void) -> Status,

    // Event & Timer Services
    create_event: extern "efiapi" fn(
        type_: u32,
        notify_tpl: TPL,
        notify_function: extern "efiapi" fn(event: Event, context: *mut core::ffi::c_void),
        notify_context: *mut core::ffi::c_void,
        event: *mut Event,
    ) -> Status,
    set_timer: extern "efiapi" fn(
        event: Event,
        type_: TimerDelay,
        trigger_time: u64,
    ) -> Status,
    wait_for_event: extern "efiapi" fn(
        number_of_events: usize,
        event: *const Event,
        index: *mut usize,
    ) -> Status,
    signal_event: extern "efiapi" fn(event: Event) -> Status,
    close_event: extern "efiapi" fn(event: Event) -> Status,
    check_event: extern "efiapi" fn(event: Event) -> Status,

    // Protocol Handler Services
    install_protocol_interface: extern "efiapi" fn(
        handle: *mut Handle,
        protocol: *mut GUID,
        interface_type: InterfaceType,
        interface: *mut core::ffi::c_void,
    ) -> Status,
    reinstall_protocol_interface: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut GUID,
        old_interface: *mut core::ffi::c_void,
        new_interface: *mut core::ffi::c_void,
    ) -> Status,
    uninstall_protocol_interface: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut GUID,
        interface: *mut core::ffi::c_void,
    ) -> Status,
    handle_protocol: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> Status,
    reserved: *mut core::ffi::c_void,
    register_protocol_notify: extern "efiapi" fn(
        protocol: *mut GUID,
        event: Event,
        registration: *mut *mut core::ffi::c_void,
    ) -> Status,
    locate_handle: extern "efiapi" fn(
        search_type: LocateSearchType,
        protocol: *mut GUID,
        search_key: *mut core::ffi::c_void,
        buffer_size: *mut usize,
        buffer: *mut Handle,
    ) -> Status,
    locate_device_path: extern "efiapi" fn(
        protocol: *mut GUID,
        device_path: *mut *mut DevicePath,
        device: *mut Handle,
    ) -> Status,
    install_configuration_table: extern "efiapi" fn(
        guid: *mut GUID,
        table: *mut core::ffi::c_void,
    ) -> Status,

    // Image Services
    load_image: extern "efiapi" fn(
        boot_policy: bool,
        parent_image_handle: Handle,
        device_path: *mut DevicePath,
        source_buffer: *mut core::ffi::c_void,
        source_size: usize,
        image_handle: *mut Handle,
    ) -> Status,
    start_image: extern "efiapi" fn(
        image_handle: Handle,
        exit_data_size: *mut usize,
        exit_data: *mut *mut u16,
    ) -> Status,
    exit: extern "efiapi" fn(
        image_handle: Handle,
        exit_status: Status,
        exit_data_size: usize,
        exit_data: *mut u16,
    ) -> Status,
    unload_image: extern "efiapi" fn(image_handle: Handle) -> Status,
    exit_boot_services: extern "efiapi" fn(
        image_handle: Handle,
        map_key: usize,
    ) -> Status,

    // Miscellaneous Services
    get_next_monotonic_count: extern "efiapi" fn(count: *mut u64) -> Status,
    stall: extern "efiapi" fn(microseconds: usize) -> Status,
    set_watchdog_timer: extern "efiapi" fn(
        timeout: usize,
        watchdog_code: u64,
        data_size: usize,
        watchdog_data: *mut u16,
    ) -> Status,

    // Driver Support Services
    connect_controller: extern "efiapi" fn(
        controller_handle: Handle,
        driver_image_handle: *mut Handle,
        remaining_device_path: *mut DevicePath,
        recursive: bool,
    ) -> Status,
    disconnect_controller: extern "efiapi" fn(
        controller_handle: Handle,
        driver_image_handle: Handle,
        child_handle: Handle,
    ) -> Status,

    // Open and Close Protocol Services
    open_protocol: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut GUID,
        interface: *mut *mut core::ffi::c_void,
        agent_handle: Handle,
        controller_handle: Handle,
        attributes: u32,
    ) -> Status,
    close_protocol: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut GUID,
        agent_handle: Handle,
        controller_handle: Handle,
    ) -> Status,
    open_protocol_information: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut GUID,
        entry_buffer: *mut *mut OpenProtocolInfoEntry,
        entry_count: *mut usize,
    ) -> Status,

    // Library Services
    protocols_per_handle: extern "efiapi" fn(
        handle: Handle,
        protocol_buffer: *mut *mut *mut GUID,
        protocol_buffer_count: *mut usize,
    ) -> Status,
    locate_handle_buffer: extern "efiapi" fn(
        search_type: LocateSearchType,
        protocol: *mut GUID,
        search_key: *mut core::ffi::c_void,
        no_handles: *mut usize,
        buffer: *mut *mut Handle,
    ) -> Status,
    locate_protocol: extern "efiapi" fn(
        protocol: *mut GUID,
        registration: *mut core::ffi::c_void,
        interface: *mut *mut core::ffi::c_void,
    ) -> Status,
    install_multiple_protocol_interfaces: extern "efiapi" fn(
        handle: *mut Handle,
        // This function takes a variable number of arguments.
    ) -> Status,
    uninstall_multiple_protocol_interfaces: extern "efiapi" fn(
        handle: Handle,
        // This function takes a variable number of arguments.
    ) -> Status,

    // 32-bit CRC Services
    calculate_crc32: extern "efiapi" fn(
        data: *mut core::ffi::c_void,
        data_size: usize,
        crc32: *mut u32,
    ) -> Status,

    // Miscellaneous Services
    copy_mem: extern "efiapi" fn(destination: *mut core::ffi::c_void, source: *const core::ffi::c_void, length: usize),
    set_mem: extern "efiapi" fn(buffer: *mut core::ffi::c_void, size: usize, value: u8),
    create_event_ex: extern "efiapi" fn(
        type_: u32,
        notify_tpl: TPL,
        notify_function: extern "efiapi" fn(event: Event, context: *mut core::ffi::c_void),
        notify_context: *mut core::ffi::c_void,
        event_group: *const GUID,
        event: *mut Event,
    ) -> Status,
}

#[allow(dead_code)]
#[repr(C)]
pub struct  SystemTable{
    header: TableHeader,
    firmware_vendor: *const u16,
    firmware_revision:u32,
    input_handle: Handle,
    input: *const TextInputProtocol,
    output_handle: Handle,
    pub output: *const TextOutputProtocol,
    error_handle: Handle,
    error:  *const TextOutputProtocol,
    runtime: *const RuntimeServices,
    boot: *const BootServices,
    no_of_entries: usize,
    config_table: *const ConfigurationTable,
}