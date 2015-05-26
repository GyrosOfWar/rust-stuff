use std::fmt;
use std::ops::{Index, IndexMut};
use instruction::Instruction;
use std::mem;

const MEMORY_SIZE: usize = 1 << 16;
const PROGRAM_LENGTH: usize = 256;
const MAR_REGISTER_IDX: u8 = 3;
const MBR_REGISTER_IDX: u8 = 15;

#[repr(u8)]
#[derive(Debug)]
pub enum AluMode {
    NoOp = 0,
    Add = 1,
    BitAnd = 2,
    BitNot = 3
}

impl AluMode {
    pub fn from_u8(value: u8) -> AluMode {
        unsafe { mem::transmute(value) }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum ShifterMode {
    NoOp = 0,
    Left = 1,
    Right = 2
}

impl ShifterMode {
    pub fn from_u8(value: u8) -> ShifterMode {
        unsafe { mem::transmute(value) }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum CondMode {
    NoOp = 0,
    IfNegative = 1,
    IfZero = 2,
    GoTo = 3
}

impl CondMode {
    pub fn from_u8(value: u8) -> CondMode {
        unsafe { mem::transmute(value) }
    }
}

pub struct Cpu<'a> {
    registers: RegisterSet,
    memory: Memory,
    program: &'a [u32],
    program_counter: u8,
    negative_flag: bool,
    zero_flag: bool
}

impl<'a> Cpu<'a> {
    pub fn new(prog: &'a [u32]) -> Cpu<'a> {
        if prog.len() > PROGRAM_LENGTH {
            panic!("Program too long!");
        }
	Cpu {
            registers: RegisterSet::new(),
	    memory: Memory::new(), 
	    program: prog,
	    program_counter: 0,
	    zero_flag: false,
	    negative_flag: false
        }
    }

    pub fn done(&self) -> bool {
        self.program_counter >= self.program.len() as u8
    }
    
    pub fn step(&mut self) {
	self.negative_flag = false;
	self.zero_flag = false;

	let next_instruction = self.program[self.program_counter as usize];
        if next_instruction == 0 {
            return;
        }
        
        let instr = Instruction::new(next_instruction);
        
        let a_bus = if instr.a_mux() { MAR_REGISTER_IDX } else { instr.a_bus() };
        let b_bus = instr.b_bus();
        let s_bus = instr.s_bus();

        if instr.ens() && s_bus < 3 {
            panic!("Can't write to read-only registers!");
        }

        if instr.mar() {
            self.registers[MAR_REGISTER_IDX] = self.registers[b_bus];
        }
        
        let alu_result = self.alu_op(instr.alu(), a_bus, b_bus);
        
        self.negative_flag = alu_result < 0;
        self.zero_flag = alu_result == 0;
        
        let shifter_result = self.shifter_op(instr.sh(), alu_result);        
        self.cond_op(instr.cond(), instr.addr());

        self.registers[s_bus] = shifter_result;
        
        if instr.ms() {
            let mar = self.registers.mar;
            let mbr = self.registers.mbr;

            if instr.rd_wr() {
                match self.memory.read(mar as usize) {
                    Some(val) => self.registers[MBR_REGISTER_IDX] = val,
                    None => ()
                }
            } else {
                self.memory.write(mar as usize, mbr);
            }
        }
        
        self.program_counter += 1;
    }

    fn alu_op(&self, alu_mode: AluMode, a_bus: u8, b_bus: u8) -> i16 {
        match alu_mode {
            AluMode::NoOp => self.registers[a_bus],
            AluMode::Add => self.registers[a_bus] + self.registers[b_bus],
            AluMode::BitAnd => self.registers[a_bus] & self.registers[b_bus],
            AluMode::BitNot => !self.registers[a_bus]
        }
    }

    fn shifter_op(&self, shifter_mode: ShifterMode, alu_result: i16) -> i16 {
        match shifter_mode {
            ShifterMode::NoOp => alu_result,
            ShifterMode::Left => alu_result << 1,
            ShifterMode::Right => alu_result >> 1
        }
    }

    fn cond_op(&mut self, cond_mode: CondMode, addr: u8) {
        match cond_mode {
            CondMode::NoOp => (),
            CondMode::IfZero => if self.zero_flag {
                self.program_counter = addr;
            },
            CondMode::IfNegative => if self.negative_flag {
                self.program_counter = addr;
            },
            CondMode::GoTo => self.program_counter = addr
        }
    }
}

impl<'a> fmt::Debug for Cpu<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU {{\n");
        write!(f, "{:?},\n", self.registers);
        write!(f, "{:?},\n", self.memory);
        write!(f, "program_counter: {}\n", self.program_counter);
        write!(f, "}}")
    }
}

#[derive(Debug)]
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

impl Index<u8> for RegisterSet {
    type Output = i16;

    fn index(&self, index: u8) -> &i16 {
        match index {
            0 => panic!("Writing to read-only register: 0"),
            1 => panic!("Writing to read-only register: 1"),
            2 => panic!("Writing to read-only register: -1"),
            3 => &self.mar,
            4 => &self.r0,
            5 => &self.r1,
            6 => &self.r2,
            7 => &self.r3,
            8 => &self.r4,
            9 => &self.r5,
            10 => &self.r6,
            11 => &self.r7,
            12 => &self.r8,
            13 => &self.r9,
            14 => &self.r10,
            15 => &self.mbr,
            _ => panic!("Invalid index!")
        }
    }
}

impl IndexMut<u8> for RegisterSet {
    fn index_mut(&mut self, index: u8) -> &mut i16 {
        match index {
            0 => &mut self.zero,
            1 => &mut self.one,
            2 => &mut self.minus_one,
            3 => &mut self.mar,
            4 => &mut self.r0,
            5 => &mut self.r1,
            6 => &mut self.r2,
            7 => &mut self.r3,
            8 => &mut self.r4,
            9 => &mut self.r5,
            10 => &mut self.r6,
            11 => &mut self.r7,
            12 => &mut self.r8,
            13 => &mut self.r9,
            14 => &mut self.r10,
            15 => &mut self.mbr,
            _ => panic!("Invalid index!")
        }
    }
}

pub struct Memory {
    data: [i16; MEMORY_SIZE],
    ready: bool
}

impl Memory {
    pub fn new() -> Memory {
	Memory { 
	    data: [0i16; MEMORY_SIZE], 
	    ready: false 
	}
    }

    pub fn read(&mut self, idx: usize) -> Option<i16> {
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

    pub fn write(&mut self, idx: usize, value: i16) -> bool {
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

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory {{");
        for (i, &val) in self.data.iter().enumerate() {
            if val != 0 {
                write!(f, "\t{}: {}\n", i, val);
            }
        }
        write!(f, "}}");
        Ok(())
    }
}
