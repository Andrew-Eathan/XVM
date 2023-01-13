extern crate sdl2;

pub mod processor;
use processor::Instruction;
use std::io::{self, Write};
use sdl2::Sdl;
static DEBUG: bool = false;

#[macro_export]
macro_rules! printdbg {
    () => {
        println!();
    };
    ( $($arg:tt)* ) => {
        println!($arg)
    };
}

pub fn main() {
    use std::time::Instant;
    let start = Instant::now();
    if DEBUG { println!("Hello, world!"); }

    let sdl_ctx: Sdl = match sdl2::init() {
        Ok(ctx) => ctx,
        Err(err) => {
            panic!("ERROR: Failed to initialise SDL screen! Check if you have the SDL2.dll file in the same place as your executable. ({:?})", err)
            
        },
    };


    let sdl_video_subsys = sdl_ctx.video();
    let window = sdl_video_subsys.expect("uh oh")
        .window("XPC Screen", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string());

    let mut cpu = processor::Processor::new();
    cpu.m_status = true;

    cpu.attach_int_handler(0, |cpu: &mut processor::Processor, _int: u8| {
        let chr: char = cpu.m_registers[15] as u8 as char;
        print!("{}", chr);
        io::stdout().flush().unwrap();
    });

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
    println!("\nexecution completed in {}ms", elapsed.as_millis());

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
