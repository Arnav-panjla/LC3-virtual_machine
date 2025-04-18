use crate::register::Registers;
use crate::utils::{sign_extend, update_flags};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum OpCode {
    BR = 0,
    ADD,
    LD,
    ST,
    JSR,
    AND,
    LDR,
    STR,
    RTI,
    NOT,
    LDI,
    STI,
    JMP,
    RES,
    LEA,
    TRAP,
}

impl OpCode {
    pub fn from_instr(instr: u16) -> Self {
        let op = (instr >> 12) & 0xF;
        unsafe { std::mem::transmute(op as u16) }
    }
}

pub fn handle_add(instr: u16, registers: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    if (instr >> 5) & 0x1 == 1 { // immediate mode 
        let imm5 = sign_extend(instr & 0x1F, 5);
        let res = registers.read(sr1).wrapping_add(imm5);
        registers.write(dr, res);
    } else { // register mode
        let sr2 = instr & 0x7;
        let res = registers.read(sr1).wrapping_add(registers.read(sr2));
        registers.write(dr, res);
    }
    update_flags(registers, dr);
}

pub fn handle_and(instr: u16, registers: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    if (instr >> 5) & 0x1 == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        let res = registers.read(sr1) & imm5;
        registers.write(dr, res);
    } else {
        let sr2 = instr & 0x7;
        let res = registers.read(sr1) & registers.read(sr2);
        registers.write(dr, res);
    }
    update_flags(registers, dr);
}

pub fn handle_not(instr: u16, registers: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let sr = (instr >> 6) & 0x7;
    registers.write(dr, !registers.read(sr));
    update_flags(registers, dr);
}
