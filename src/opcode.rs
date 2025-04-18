use crate::{register::Registers, utils};

#[derive(Debug)]
#[repr(u16)]
pub enum OpCode {
    BR = 0,
    ADD = 1,
    LD = 2,
    ST = 3,
    JSR = 4,
    AND = 5,
    LDR = 6,
    STR = 7,
    RTI = 8,
    NOT = 9,
    LDI = 10,
    STI = 11,
    JMP = 12,
    RES = 13,
    LEA = 14,
    TRAP = 15,
}

impl OpCode {
    pub fn from_instr(instr: u16) -> Self {
        match (instr >> 12) & 0xF {
            0 => OpCode::BR,
            1 => OpCode::ADD,
            2 => OpCode::LD,
            3 => OpCode::ST,
            4 => OpCode::JSR,
            5 => OpCode::AND,
            6 => OpCode::LDR,
            7 => OpCode::STR,
            8 => OpCode::RTI,
            9 => OpCode::NOT,
            10 => OpCode::LDI,
            11 => OpCode::STI,
            12 => OpCode::JMP,
            13 => OpCode::RES,
            14 => OpCode::LEA,
            15 => OpCode::TRAP,
            _ => OpCode::RES,
        }
    }
}

// Implement opcode logic stubs here:
pub fn handle_add(instr: u16, reg: &mut Registers) {

}

pub fn handle_and(instr: u16, reg: &mut Registers) {

}

pub fn handle_not(instr: u16, reg: &mut Registers) {
    
}
