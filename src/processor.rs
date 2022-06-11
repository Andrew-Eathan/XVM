type IntHandler = fn(&mut Processor, u8);
use std::collections::HashMap;
use crate::DEBUG;

#[allow(dead_code)]
pub struct Processor {
    pub m_memory: Vec<u8>,
    pub m_program_counter: u64,
    pub m_status: bool,
    pub m_registers: [i64; 16],
    pub m_stack: Vec<u64>,
    pub m_intlisteners: HashMap<u8, IntHandler>
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum Instruction {
    NOP = 0,

    LOAD1B, LOAD2B, LOAD3B, LOAD4B,
    LOAD5B, LOAD6B, LOAD7B, LOAD8B,

    ADD, SUB,
    MUL, DIV, 
    
    NOT, AND, OR, XOR,

    CALL, RET, END, CPUID, INT
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Signal {
    DEADEND,
    FAULT,
    ERROR,
    BADOP,
    OVERFLOWED
}

impl Processor {
    pub fn process(&mut self) {
        if self.m_program_counter as usize >= self.m_memory.len() {
            self.emit_signal(Signal::DEADEND);
            self.m_status = false;
            return;
        }

        let byte = self.get_byte();
        let inst: Instruction = unsafe { ::std::mem::transmute(byte) };

        if DEBUG { println!("Instruction: {:?}", inst); }

        match inst {
            Instruction::NOP => {}
            Instruction::LOAD1B => {
                let reg = self.get_byte();
                let byte1 = self.get_byte();
                self.m_registers[reg as usize] = byte1 as i64;
            }
            Instruction::LOAD2B => {
                let reg = self.get_byte();
                let byte1 = self.get_byte();
                let byte2 = self.get_byte();
                self.m_registers[reg as usize] = (
                    byte1 as u16 | 
                    ((byte2 as u16) << 8)
                ) as i64;
            }
            Instruction::LOAD3B => {
                let reg = self.get_byte();
                let byte1 = self.get_byte();
                let byte2 = self.get_byte();
                let byte3 = self.get_byte();
                self.m_registers[reg as usize] = (
                    byte1 as u64 | 
                    ((byte2 as u64) << 8) |
                    ((byte3 as u64) << 16)
                ) as i64;
            }
            Instruction::LOAD4B => {
                let reg = self.get_byte();
                let byte1 = self.get_byte();
                let byte2 = self.get_byte();
                let byte3 = self.get_byte();
                let byte4 = self.get_byte();
                self.m_registers[reg as usize] = (
                    byte1 as u64 | 
                    ((byte2 as u64) << 8) |
                    ((byte3 as u64) << 16) |
                    ((byte4 as u64) << 24)
                ) as i64;
            }
            Instruction::LOAD5B => {
                let reg = self.get_byte();
                let byte1 = self.get_byte();
                let byte2 = self.get_byte();
                let byte3 = self.get_byte();
                let byte4 = self.get_byte();
                let byte5 = self.get_byte();
                self.m_registers[reg as usize] = (
                    byte1 as u64 | 
                    ((byte2 as u64) << 8) |
                    ((byte3 as u64) << 16) |
                    ((byte4 as u64) << 24) |
                    ((byte5 as u64) << 32)
                ) as i64;
            }
            Instruction::LOAD6B => {
                let reg = self.get_byte();
                let byte1 = self.get_byte();
                let byte2 = self.get_byte();
                let byte3 = self.get_byte();
                let byte4 = self.get_byte();
                let byte5 = self.get_byte();
                let byte6 = self.get_byte();
                self.m_registers[reg as usize] = (
                    byte1 as u64 | 
                    ((byte2 as u64) << 8) |
                    ((byte3 as u64) << 16) |
                    ((byte4 as u64) << 24) |
                    ((byte5 as u64) << 32) |
                    ((byte6 as u64) << 40)
                ) as i64;
            }
            Instruction::LOAD7B => {
                let reg = self.get_byte();
                let byte1 = self.get_byte();
                let byte2 = self.get_byte();
                let byte3 = self.get_byte();
                let byte4 = self.get_byte();
                let byte5 = self.get_byte();
                let byte6 = self.get_byte();
                let byte7 = self.get_byte();
                self.m_registers[reg as usize] = (
                    byte1 as u64 | 
                    ((byte2 as u64) << 8) |
                    ((byte3 as u64) << 16) |
                    ((byte4 as u64) << 24) |
                    ((byte5 as u64) << 32) |
                    ((byte6 as u64) << 40) |
                    ((byte7 as u64) << 48) 
                ) as i64;
            }
            Instruction::LOAD8B => {
                let reg = self.get_byte();
                let byte1 = self.get_byte();
                let byte2 = self.get_byte();
                let byte3 = self.get_byte();
                let byte4 = self.get_byte();
                let byte5 = self.get_byte();
                let byte6 = self.get_byte();
                let byte7 = self.get_byte();
                let byte8 = self.get_byte();
                self.m_registers[reg as usize] = (
                    byte1 as u64 | 
                    ((byte2 as u64) << 8) |
                    ((byte3 as u64) << 16) |
                    ((byte4 as u64) << 24) |
                    ((byte5 as u64) << 32) |
                    ((byte6 as u64) << 40) |
                    ((byte7 as u64) << 48) |
                    ((byte8 as u64) << 56)
                ) as i64;
            }
            Instruction::ADD => {
                let reg1 = self.get_byte();
                let reg2 = self.get_byte();
                let reg3 = self.get_byte();
                self.m_registers[reg3 as usize] = self.m_registers[reg1 as usize] + self.m_registers[reg2 as usize];
            }
            Instruction::SUB => {
                let reg1 = self.get_byte();
                let reg2 = self.get_byte();
                let reg3 = self.get_byte();
                self.m_registers[reg3 as usize] = self.m_registers[reg1 as usize] - self.m_registers[reg2 as usize];
            }
            Instruction::MUL => {
                let reg1 = self.get_byte();
                let reg2 = self.get_byte();
                let reg3 = self.get_byte();
                self.m_registers[reg3 as usize] = self.m_registers[reg1 as usize] * self.m_registers[reg2 as usize];
            }
            Instruction::DIV => {
                let reg1 = self.get_byte();
                let reg2 = self.get_byte();
                let reg3 = self.get_byte();
                self.m_registers[reg3 as usize] = self.m_registers[reg1 as usize] / self.m_registers[reg2 as usize];
            }
            Instruction::NOT => {
                let reg1 = self.get_byte();
                self.m_registers[reg1 as usize] = !self.m_registers[reg1 as usize];
            }
            Instruction::AND => {
                let reg1 = self.get_byte();
                let reg2 = self.get_byte();
                self.m_registers[reg1 as usize] &= self.m_registers[reg2 as usize];
            }
            Instruction::OR => {
                let reg1 = self.get_byte();
                let reg2 = self.get_byte();
                self.m_registers[reg1 as usize] |= self.m_registers[reg2 as usize];
            }
            Instruction::XOR => {
                let reg1 = self.get_byte();
                let reg2 = self.get_byte();
                self.m_registers[reg1 as usize] ^= self.m_registers[reg2 as usize];
            }
            Instruction::CALL => {
                let byte1 = self.get_byte();
                let byte2 = self.get_byte();
                let byte3 = self.get_byte();
                let byte4 = self.get_byte();
                let byte5 = self.get_byte();
                let byte6 = self.get_byte();
                let byte7 = self.get_byte();
                let byte8 = self.get_byte();
                let address = (
                    byte1 as u64 | 
                    ((byte2 as u64) << 8) |
                    ((byte3 as u64) << 16) |
                    ((byte4 as u64) << 24) |
                    ((byte5 as u64) << 32) |
                    ((byte6 as u64) << 40) |
                    ((byte7 as u64) << 48) |
                    ((byte8 as u64) << 56)
                ) as i64;

                self.m_stack.push(self.m_program_counter);
                self.m_program_counter = (address - 1) as u64;
            }
            Instruction::RET => {
                self.m_program_counter = *self.m_stack.last().expect("Tried to return with no call address on the stack") as u64 - 1;
                self.m_stack.pop();
            }
            Instruction::END => {
                self.m_status = false;
                self.emit_signal(Signal::DEADEND);
            }
            Instruction::INT => {
                let byte = self.get_byte();
                self.m_intlisteners[&byte](self, byte);
            }
            _ => {
                self.emit_signal(Signal::BADOP);
            }
        }
    }
    pub fn emit_signal(&self, s: Signal) {
        if DEBUG { println!("{:#01x} ({}) signaled {:?}", self.m_program_counter, self.m_program_counter, s); }
    }
    pub fn get_byte(&mut self) -> u8 {
        let mem = self.m_memory[self.m_program_counter as usize];
        self.m_program_counter += 1;
        mem
    }
    pub fn new() -> Processor {
        Processor {
            m_memory: Vec::new(),
            m_program_counter: 0,
            m_status: false,
            m_registers: [0; 16],
            m_stack: Vec::new(),
            m_intlisteners: HashMap::new()
        }
    }
    pub fn push_byte(&mut self, byte: u8) {
        self.m_memory.push(byte);
    }
    pub fn dump_registers(&self) {
        if DEBUG { println!("Register dump:"); }
        for n in 0 .. 16 {
            if self.m_registers[n] == 0 { continue }
            if DEBUG { println!("{}: {}", n, self.m_registers[n]); }
        }
    }
    pub fn dump_stack(&self) {
        if self.m_stack.len() == 0 { return }
        if DEBUG { println!("Stack dump:"); }

        let mut i = 0;
        for n in self.m_stack.iter() {
            let indicator = if i == self.m_stack.len() { "< we're here" } else { "" };
            if DEBUG { println!("{}: {}{}", i, n, indicator); }
            i += 1;
        }
    }
    pub fn getpc(&self) -> u64 {
        self.m_program_counter
    }
    pub fn attach_int_handler(&mut self, int: u8, func: IntHandler) {
        self.m_intlisteners.insert(int, func);
        if DEBUG { println!("Attached listener for interrupt {}", int); }
    }
}