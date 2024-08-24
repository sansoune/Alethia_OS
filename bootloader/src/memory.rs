use uefi::table::boot::MemoryMapIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MemoryType {
    Reserved,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    Conventional,
    Unusable,
    AcpiReclaim,
    AcpiNvs,
    MemoryMappedIo,
    MemoryMappedIoPortSpace,
    PalCode,
    PersistentMemory,
}

impl MemoryType {
    pub fn fromuefi(ty: u32) -> Self {
        match ty {
            0 => MemoryType::Reserved,
            1 => MemoryType::LoaderCode,
            2 => MemoryType::LoaderData,
            3 => MemoryType::BootServicesCode,
            4 => MemoryType::BootServicesData,
            5 => MemoryType::RuntimeServicesCode,
            6 => MemoryType::RuntimeServicesData,
            7 => MemoryType::Conventional,
            8 => MemoryType::Unusable,
            9 => MemoryType::AcpiReclaim,
            10 => MemoryType::AcpiNvs,
            11 => MemoryType::MemoryMappedIo,
            12 => MemoryType::MemoryMappedIoPortSpace,
            13 => MemoryType::PalCode,
            14 => MemoryType::PersistentMemory,
            _ => MemoryType::Reserved,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryMap {
    pub entries: &'static [MemoryDescriptor],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryDescriptor {
    pub ty: MemoryType,
    pub phys_start: u64,
    pub page_count: u64,
}

static mut MEMORY_MAP_ENTRIES: [MemoryDescriptor; 64] = [MemoryDescriptor {
    ty: MemoryType::Reserved,
    phys_start: 0,
    page_count: 0,
}; 64];

pub fn get_memory_info(memory_map: MemoryMapIter) -> MemoryMap {

    let mut count = 0; 

    unsafe {
    for dec in memory_map {
        if count >= 64 {
            break;
        }
        MEMORY_MAP_ENTRIES[count] = MemoryDescriptor {
            ty: MemoryType::fromuefi(dec.ty.0),
            phys_start: dec.phys_start,
            page_count: dec.page_count
        };
        count += 1;
    }

    MemoryMap{entries: &MEMORY_MAP_ENTRIES[..count]}
}
}