pub const MAGIC_NUMBER: u64 = 0xDE65786500000000;
pub const HEADER_SIZE: usize = 48;

#[derive(Debug)]
#[repr(C)]
pub struct FileHeader {
    pub magic_number: u64, // Magic number to identify file type

    pub data_start: u64, // Start of the data section starting from the end of the header
    pub data_size: u64,  // Size of the data section in bytes

    pub text_start: u64, // Offset from end of the header to the start of the text section where the code is stored
    pub text_size: u64,  // Size of the text section in bytes
    pub entry_point: u64, // Offset from the end of the header to the entry point of the program
}

impl FileHeader {
    pub fn load_header(bytes: &[u8]) -> Result<Self, ()> {
        let mut s = Self {
            magic_number: 0,
            data_start: 0,
            data_size: 0,
            text_start: 0,
            text_size: 0,
            entry_point: 0,
        };

        if bytes.len() < HEADER_SIZE as usize {
            eprintln!("Invalid file format: File size too small");
            return Err(());
        }

        // it is from_be_bytes to preserve the order of the bytes
        s.magic_number = u64::from_be_bytes(bytes[0..8].try_into().unwrap());

        if s.magic_number != MAGIC_NUMBER {
            eprintln!("Invalid file format: Magic number mismatch");
            return Err(());
        }

        s.data_start = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        s.data_size = u64::from_le_bytes(bytes[16..24].try_into().unwrap());

        s.text_start = u64::from_le_bytes(bytes[24..32].try_into().unwrap());
        s.text_size = u64::from_le_bytes(bytes[32..40].try_into().unwrap());
        s.entry_point = u64::from_le_bytes(bytes[40..48].try_into().unwrap());

        Ok(s)
    }
}
