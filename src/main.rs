pub mod processor;
pub mod screen;

use processor::Instruction;
use std::io::{stdout, Write};
static DEBUG: bool = false;

#[macro_export]
macro_rules! printdbg {
    () => {
        println!();
    };
    ( $($arg:expr),* ) => {
        if DEBUG {
            println!($($arg),*);
        }
    };
}

pub fn main() {
    use std::time::Instant;
    let start = Instant::now();

    let mut cpu = processor::Processor::new();
    cpu.m_status = true;

    cpu.attach_int_handler(0, |cpu: &mut processor::Processor, _int: u8| {
        let chr: char = cpu.m_registers[15] as u8 as char;
        print!("{}", chr);
        stdout().flush().unwrap();
    });

    cpu.push_byte((69420 & 0x0000FF) as u8);
    cpu.push_byte(((69420 & 0x00FF00) >> 8) as u8);
    cpu.push_byte(((69420 & 0xFF0000) >> 16) as u8);

    println!("{}", cpu.get_bytes(3));

    // load byte 4 in register 0
    cpu.push_byte(Instruction::LOAD1B as u8);
    cpu.push_byte(0);
    cpu.push_byte(4);

    // load byte 1 in register 1
    cpu.push_byte(Instruction::LOAD1B as u8);
    cpu.push_byte(1);
    cpu.push_byte(6);

    // add them into register 2
    cpu.push_byte(Instruction::ADD as u8);
    cpu.push_byte(0);
    cpu.push_byte(1);
    cpu.push_byte(2);
    
    // load 2 bytes
    cpu.push_byte(Instruction::LOAD2B as u8);
    cpu.push_byte(0);
    cpu.push_byte((500 & 0xff) as u8);
    cpu.push_byte(((500 & 0xff00) >> 8) as u8);
    
    // multiply together
    cpu.push_byte(Instruction::MUL as u8);
    cpu.push_byte(0);
    cpu.push_byte(2);
    cpu.push_byte(1);

    // load H byte
    cpu.push_byte(Instruction::LOAD1B as u8);
    cpu.push_byte(15);
    cpu.push_byte('H' as u8);
    
    // call print interrupt
    cpu.push_byte(Instruction::INT as u8);
    cpu.push_byte(0);

    // load i byte
    cpu.push_byte(Instruction::LOAD1B as u8);
    cpu.push_byte(15);
    cpu.push_byte('i' as u8);
    
    // print
    cpu.push_byte(Instruction::INT as u8);
    cpu.push_byte(0);

    cpu.push_byte(Instruction::LOAD1B as u8);
    cpu.push_byte(15);
    cpu.push_byte('!' as u8);

    // print
    cpu.push_byte(Instruction::INT as u8);
    cpu.push_byte(0);

    while cpu.m_status {
        cpu.process();
        cpu.dump_registers();
        cpu.dump_stack();
    }



    let elapsed = start.elapsed();
    println!("Execution completed in {}ms!\n", elapsed.as_millis());

    /*'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
    }*/
}
