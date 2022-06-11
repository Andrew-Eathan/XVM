pub mod processor;
use processor::Instruction;
use std::io::{self, Write};
static DEBUG: bool = false;

fn main() {
    use std::time::Instant;
    let start = Instant::now();
    if DEBUG { println!("Hello, world!"); }

    let mut cpu = processor::Processor::new();
    cpu.m_status = true;

    cpu.attach_int_handler(0, |cpu: &mut processor::Processor, _int: u8| {
        let chr: char = cpu.m_registers[15] as u8 as char;
        print!("{}", chr);
        io::stdout().flush().unwrap();
    });

    cpu.push_byte(Instruction::LOAD1B as u8);
    cpu.push_byte(0);
    cpu.push_byte(4);

    cpu.push_byte(Instruction::LOAD1B as u8);
    cpu.push_byte(1);
    cpu.push_byte(6);

    cpu.push_byte(Instruction::ADD as u8);
    cpu.push_byte(0);
    cpu.push_byte(1);
    cpu.push_byte(2);
    
    cpu.push_byte(Instruction::LOAD2B as u8);
    cpu.push_byte(0);
    cpu.push_byte((500 & 0xff) as u8);
    cpu.push_byte(((500 & 0xff00) >> 8) as u8);
    
    cpu.push_byte(Instruction::MUL as u8);
    cpu.push_byte(0);
    cpu.push_byte(2);
    cpu.push_byte(1);

    cpu.push_byte(Instruction::LOAD1B as u8);
    cpu.push_byte(15);
    cpu.push_byte('H' as u8);
    
    cpu.push_byte(Instruction::INT as u8);
    cpu.push_byte(0);

    cpu.push_byte(Instruction::LOAD1B as u8);
    cpu.push_byte(15);
    cpu.push_byte('i' as u8);
    
    cpu.push_byte(Instruction::INT as u8);
    cpu.push_byte(0);

    while cpu.m_status {
        cpu.process();
        cpu.dump_registers();
        cpu.dump_stack();
    }

    let elapsed = start.elapsed();
    println!("\nexecution completed in {}ms", elapsed.as_millis());
}
