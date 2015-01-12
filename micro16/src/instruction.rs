use bitset32::BitSet32;

pub struct Instruction {
    bits: BitSet32
}

impl Instruction {
    pub fn new(raw: u32) -> Instruction {
        Instruction {
            bits: BitSet32::new(raw)
        }
    }

    pub fn addr(&self) -> u8 {
        self.bits.get_many(0, 8) as u8
    }

    pub fn a_bus(&self) -> u8 {
        self.bits.get_many(8, 12) as u8
    }

    pub fn b_bus(&self) -> u8 {
        self.bits.get_many(12, 16) as u8
    }

    pub fn s_bus(&self) -> u8 {
        self.bits.get_many(16, 20) as u8
    }

    pub fn ens(&self) -> bool {
        self.bits.get(20)
    }

    pub fn ms(&self) -> bool {
        self.bits.get(21)
    }
}
