use cpu::Cpu;

pub mod bitset32;
pub mod cpu;
pub mod instruction;

fn main() {
    let mut program = 
        &[0x081e1100,
         0x001d0e00,
         0x021e0e00,
         0x0080e000,
         0x01200d00,
         0x00200000];
    let mut cpu = Cpu::new(program);
    println!("{:?}", cpu);
    
    while !cpu.done() {
        println!("{:?}", cpu);
        cpu.step();
    }
}
