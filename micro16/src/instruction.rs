use bitset32::BitSet32;
use cpu::{AluMode, ShifterMode, CondMode};

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

    pub fn rd_wr(&self) -> bool {
        self.bits.get(22)
    }

    pub fn mar(&self) -> bool {
        self.bits.get(23)
    }

    pub fn mbr(&self) -> bool {
        self.bits.get(24)
    }

    pub fn sh(&self) -> ShifterMode {
        ShifterMode::from_u8(self.bits.get_many(25, 27) as u8)
    }

    pub fn alu(&self) -> AluMode {
        AluMode::from_u8(self.bits.get_many(27, 29) as u8)
    }

    pub fn cond(&self) -> CondMode {
        CondMode::from_u8(self.bits.get_many(29, 31) as u8)
    }

    pub fn a_mux(&self) -> bool {
        self.bits.get(31)
    }
}
