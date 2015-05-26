use cpu::Cpu;

pub mod bitset32;
pub mod cpu;
pub mod instruction;

fn main() {
    let mut program = [0u32; 256];
    program[0] = 0x0a151100;
    program[1] = 0x01200500;
    program[2] = 0x00200000;
    
    let mut cpu = Cpu::new(program, 3);

    while !cpu.done() {
        println!("{:?}", cpu);
        cpu.step();
    }
}
