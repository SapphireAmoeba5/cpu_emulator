pub const SECTION_HEADER_SIZE: u64 = std::mem::size_of::<SectionHeader>() as u64;

#[derive(Debug)]
pub struct SectionHeader {
    pub off_offset: u64, // Offset from beginning of file to the start of the section
    pub addr: u64,       // Address that this section is loaded to
    pub size: u64,       // Size of the section
}

impl SectionHeader {
    pub fn load_header(header: &[u8]) -> Result<Self, ()> {
        let mut s = Self {
            off_offset: 0,
            addr: 0,
            size: 0,
        };

        // These if statements should never be true, but these are here just in case
        if (header.len() as u64) < SECTION_HEADER_SIZE {
            eprintln!("Invalid file format: Section header is too small");
            return Err(());
        } else if (header.len() as u64) > SECTION_HEADER_SIZE {
            eprintln!("Invalid file format: Section header is too large");
            return Err(());
        }

        s.off_offset = u64::from_le_bytes(header[0..8].try_into().unwrap());
        s.addr = u64::from_le_bytes(header[8..16].try_into().unwrap());
        s.size = u64::from_le_bytes(header[16..24].try_into().unwrap());

        Ok(s)
    }
}
