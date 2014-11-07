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
}
