mod registers;

use crate::header::{FileHeader, HEADER_SIZE};
use crate::section_header::SectionHeader;
use registers::Registers;

pub struct Executor {
    stack: Vec<u8>,
    registers: Registers,
}

impl Executor {
    pub fn execute(file: &[u8]) {
        let mut s = Self {
            // Initialize the stack with 128MiB of memory.
            stack: vec![0; 134217728],
            registers: Registers::new(),
        };

        s.initialize_executor(file);
    }

    fn initialize_executor(&mut self, file: &[u8]) -> Result<(), ()> {
        let header = FileHeader::load_header(file)?;
        let sections = Self::load_sections(file, header.off_shoff);

        Ok(())
    }

    fn load_sections(file: &[u8], off: u64) -> Result<Vec<SectionHeader>, ()> {
        Ok(Vec::new())
    }
}
