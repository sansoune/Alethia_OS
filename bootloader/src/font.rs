#[repr(C)]
#[derive(Copy, Clone)]
pub struct PSF1Header {
    pub magic: u16,
    pub mode: u8,
    pub char_size: u8,
}

pub struct Font {
    pub header: PSF1Header,
    pub glyphs: &'static [u8],
}

impl Font {
    pub fn new(header: PSF1Header, glyphs: &'static [u8]) -> Self {
        Font { header: header , glyphs }
    }
}