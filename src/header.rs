pub const MAGIC_NUMBER: u64 = 0x00000000DE657865;
pub const HEADER_SIZE: u64 = std::mem::size_of::<FileHeader>() as u64;

#[derive(Debug)]
pub struct FileHeader {
    pub magic_number: u64, // Magic number to identify file type
    pub off_shoff: u64,    // Offset from beginning of file to the start of the section header table
    pub shnum: u64,        // Number of section table entries
    pub addr_entry: u64,   // Address of entry point
}

impl FileHeader {
    pub fn load_header(bytes: &[u8]) -> Result<Self, ()> {
        let mut s = Self {
            magic_number: 0,
            off_shoff: 0,
            shnum: 0,
            addr_entry: 0,
        };

        if (bytes.len() as u64) < HEADER_SIZE {
            eprintln!("Invalid file format: File is too small");
            return Err(());
        }

        s.magic_number = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        s.off_shoff = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        s.shnum = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
        s.addr_entry = u64::from_le_bytes(bytes[24..32].try_into().unwrap());

        Ok(s)
    }
}
