use std::old_io::File;
use instruction::Instruction;

const MEMORY_SIZE: usize = 1 << 16;
const PROGRAM_LENGTH: usize = 256;

enum AluModes {
    AluNoOp = 0,
    Add = 1,
    BitAnd = 2,
    BitNot = 3
}

enum ShifterModes {
    ShifterNoOp = 0,
    ShiftLeft = 1,
    ShiftRight = 2
}

enum CondModes {
    IfNegative = 1,
    IfZero = 2,
    GoTo = 3
}

pub struct Cpu {
    registers: RegisterSet,
    memory: Memory,
    program: [u32; PROGRAM_LENGTH],
    program_counter: u8,
    negative_flag: bool,
    zero_flag: bool
}

impl Cpu {
    pub fn new(prog: [u32; PROGRAM_LENGTH]) -> Cpu {
	Cpu {
            registers: RegisterSet::new(),
	    memory: Memory::new(), 
	    program: prog,
	    program_counter: 0u8,
	    zero_flag: false,
	    negative_flag: false
        }
    }

    pub fn step(&mut self) {
	self.negative_flag = false;
	self.zero_flag = false;

	let next_instruction = self.program[self.program_counter as usize];
        let decoded_instr = Instruction::new(next_instruction);
        
        self.program_counter += 1;
    }
}

pub struct RegisterSet {
    zero: i16,
    one: i16,
    minus_one: i16,
    r0: i16,
    r1: i16,
    r2: i16,
    r3: i16,
    r4: i16,
    r5: i16,
    r6: i16,
    r7: i16,
    r8: i16,
    r9: i16,
    r10: i16,
    mar: i16,
    mbr: i16
}

impl RegisterSet {
    fn new() -> RegisterSet {
	RegisterSet {
	    zero: 0,
	    one: 1,
	    minus_one: -1,
	    r0: 0,
	    r1: 0,
	    r2: 0,
	    r3: 0,
	    r4: 0,
	    r5: 0,
	    r6: 0,
	    r7: 0,
	    r8: 0,
	    r9: 0,
	    r10: 0,
	    mar: 0,
	    mbr: 0
	}
    }
}

pub struct Memory {
    data: [i16; MEMORY_SIZE],
    ready: bool
}

impl Memory {
    fn new() -> Memory {
	Memory { 
	    data: [0i16; MEMORY_SIZE], 
	    ready: false 
	}
    }

    fn read(&mut self, idx: usize) -> Option<i16> {
	if !self.ready {
	    self.ready = true;
	    None
	}
	else {
	    self.ready = false;
	    let result = self.data[idx];
	    Some(result)
	}
    }

    fn write(&mut self, idx: usize, value: i16) -> bool {
	if !self.ready {
	    self.ready = true;
	    false
	}

	else {
	    self.ready = false;
	    self.data[idx] = value;
	    true
	}
    }
}
