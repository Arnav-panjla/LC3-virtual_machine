use crate::{register::Registers, utils, Memory};

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

pub fn handle_add(instr: u16, reg: &mut Registers) {// format: 0001 DR SR1 SR2/imm5
    let dr = (instr >> 9) & 0x7; 
    let sr1 = (instr >> 6) & 0x7; 
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {// Immediate mode
        let imm5 = utils::sign_extend(instr & 0x1F, 5);
        reg.set(dr as usize, reg.get(sr1 as usize).wrapping_add(imm5));
    } 
    else { // Register mode
        let sr2 = instr & 0x7;
        reg.set(dr as usize, reg.get(sr1 as usize).wrapping_add(reg.get(sr2 as usize)));
    }
}

pub fn handle_and(instr: u16, reg: &mut Registers) {// format: 0001 DR SR1 SR2/imm5
    let dr = ((instr >> 9) & 0x7 ) as usize;
    let sr1 = ((instr >> 6) & 0x7) as usize; 
    let imm_flag = ((instr >> 5) & 0x1) as usize; 
    if imm_flag == 1 {  // Immediate mode
        let imm5 = utils::sign_extend(instr & 0x1F, 5);
        reg.set(dr as usize, reg.get(sr1 as usize) & imm5);
    }
    else {  // Register mode
        let sr2 = instr & 0x7;
        reg.set(dr as usize, reg.get(sr1 as usize) & reg.get(sr2 as usize));
    }

}

pub fn handle_not(instr: u16, reg: &mut Registers) {// format: 1001 DR SR1
    let dr = ((instr >> 9) & 0x7) as usize; 
    let sr1 = ((instr >> 6) & 0x7) as usize; 

    reg.set(dr, !reg.get(sr1));

}

pub fn handle_br(instr: u16, reg: &mut Registers) {// format: 0000 n z p PCoffset9
    let cond_flag = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);

    let cond = reg.get_cond_flag();
    if cond & cond_flag != 0 {
        let pc = reg.get_pc().wrapping_add(pc_offset);
        reg.set_pc(pc);
    }
}

pub fn handle_jmp(instr: u16, reg: &mut Registers) {
    let base_r = (instr >> 6) & 0x7;
    let address = reg.get(base_r as usize);
    reg.set_pc(address);
}

pub fn handle_ld(instr: u16, mem: &mut Memory, reg: &mut Registers) {// format: 0010 DR PCoffset9
    let dr = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);
    let address = reg.get_pc().wrapping_add(pc_offset);
    let value = mem.read(address);
    reg.set(dr as usize, value);
}

pub fn handle_st(instr: u16, mem: &mut Memory, reg: &mut Registers) {// format: 0011 SR PCoffset9
    let sr = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);
    let address = reg.get_pc().wrapping_add(pc_offset);
    let value = reg.get(sr as usize);
    mem.write(address, value);
}

pub fn handle_jsr(instr: u16, reg: &mut Registers) {
    let long_flag = (instr >> 11) & 1;

    let ret_addr = reg.get_pc();
    reg.set(7, ret_addr); 

    if long_flag == 1 {
        // JSR with PCoffset11
        let offset = crate::utils::sign_extend(instr & 0x7FF, 11);
        reg.set_pc(ret_addr.wrapping_add(offset));
    } else {
        // JSRR with baseR
        let base_r = (instr >> 6) & 0x7;
        reg.set_pc(reg.get(base_r as usize));
    }
}


pub fn handle_ldr(instr: u16, mem: &mut Memory, reg: &mut Registers) {// format: 0110 DR BaseR PCoffset6
    let dr = (instr >> 9) & 0x7;
    let base_r = (instr >> 6) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x3F, 6);
    let address = reg.get(base_r as usize).wrapping_add(pc_offset);
    let value = mem.read(address);
    reg.set(dr as usize, value);
}

pub fn handle_str(instr: u16, mem: &mut Memory, reg: &mut Registers) {// format: 0111 SR BaseR PCoffset6
    let sr = (instr >> 9) & 0x7;
    let base_r = (instr >> 6) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x3F, 6);
    let address = reg.get(base_r as usize).wrapping_add(pc_offset);
    let value = reg.get(sr as usize);
    mem.write(address, value);
}

pub fn handle_sti(instr: u16, mem: &mut Memory, reg: &mut Registers) {// format: 1011 SR PCoffset9
    let sr = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);
    let address = reg.get_pc().wrapping_add(pc_offset);
    let value = reg.get(sr as usize);
    let target_addr = mem.read(address);
    mem.write(target_addr, value); 
}

pub fn handle_ldi(instr: u16, mem: &mut Memory, reg: &mut Registers) {// format: 1010 DR PCoffset9
    let dr = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);
    let address = reg.get_pc().wrapping_add(pc_offset);
    let indirect_addr = mem.read(address);
    let value = mem.read(indirect_addr);
    reg.set(dr as usize, value);

}

pub fn handle_lea(instr: u16, mem: &mut Memory, reg: &mut Registers) {// format: 1110 DR PCoffset9
    let dr = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);
    let address = reg.get_pc().wrapping_add(pc_offset);
    reg.set(dr as usize, address);
}   
