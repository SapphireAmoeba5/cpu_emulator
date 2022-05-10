pub struct Registers {
    x: [u64; 32],

    pub ip: u64,
    pub sp: u64,
    pub bp: u64,
    // Cpu flags
    pub zero: u8,
    pub carry: u8,
    pub sign: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            x: [0; 32],

            ip: 0,
            sp: 0,
            bp: 0,

            zero: 0,
            carry: 0,
            sign: 0,
        }
    }
}
