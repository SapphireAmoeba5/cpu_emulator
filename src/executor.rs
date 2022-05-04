mod registers;
use crate::format::{FileHeader, HEADER_SIZE};
use registers::Registers;

pub struct Executor {
    stack: Vec<u8>,
    registers: Registers,
}

impl Executor {
    pub fn execute(exe: &[u8]) {
        let mut s = Self {
            // Initialize the stack with 128MiB of memory.
            stack: vec![0; 134217728],
            registers: Registers::new(),
        };

        s.initialize_executor(exe);
    }

    fn initialize_executor(&mut self, exe: &[u8]) -> Result<(), ()> {
        let header = FileHeader::load_header(exe)?;

        let data_start: usize = HEADER_SIZE + header.data_start as usize;
        let data_end: usize = data_start + header.data_size as usize;

        if data_end >= exe.len() {
            eprintln!("Invalid file format: Data section is out of bounds");
            return Err(());
        }

        let text_start: usize = HEADER_SIZE + header.text_start as usize;
        let text_end: usize = text_start + header.text_size as usize;
        let entry_point: usize = HEADER_SIZE + header.entry_point as usize;

        if text_end > exe.len() {
            eprintln!("Invalid file format: Text section is out of bounds");
            return Err(());
        }
        if entry_point > exe.len() {
            eprintln!("Invalid file format: Entry point is out of bounds");
            return Err(());
        }

        let data = &exe[data_start..data_end];
        let text = &exe[text_start..text_end];

        self.stack
            .splice(0..header.data_size as usize, data.iter().cloned());

        self.stack.splice(
            header.data_size as usize..header.text_size as usize,
            text.iter().cloned(),
        );

        self.registers.sp = (self.stack.len() - 1) as u64;
        self.registers.ip = header.entry_point;

        println!(
            "Stack pointer: {}\nInstruction pointer: {}",
            self.registers.sp, self.registers.ip
        );
        println!("Stack size: {}", self.stack.len() - 1);

        Ok(())
    }
}
