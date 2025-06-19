use crate::{register::Registers, utils, Memory};

#[derive(Debug)]
#[repr(u16)]
pub enum OpCode {
    BR = 0,    // Branch
    ADD = 1,   // Add
    LD = 2,    // Load
    ST = 3,    // Store
    JSR = 4,   // Jump to Subroutine
    AND = 5,   // Bitwise AND
    LDR = 6,   // Load Register
    STR = 7,   // Store Register
    RTI = 8,   // Return from Interrupt
    NOT = 9,   // Bitwise NOT
    LDI = 10,  // Load Indirect
    STI = 11,  // Store Indirect
    JMP = 12,  // Jump
    RES = 13,  // Reserved (unused)
    LEA = 14,  // Load Effective Address
    TRAP = 15, // Trap/System Call
}

impl OpCode {
    pub fn from_instr(instr: u16) -> Self {
        match instr >> 12 {
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

/// Handles the ADD instruction (opcode 0001)
/// 
/// Format: 0001 DR SR1 0 00 SR2 | 0001 DR SR1 1 imm5
///
/// Adds the contents of SR1 and either SR2 or an immediate value,
/// storing the result in DR.
pub fn handle_add(instr: u16, reg: &mut Registers) {
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

/// Handles the AND instruction (opcode 0101)
/// 
/// Format: 0101 DR SR1 0 00 SR2 | 0101 DR SR1 1 imm5
///
/// Performs bitwise AND on the contents of SR1 and either SR2 or an immediate value,
/// storing the result in DR.
pub fn handle_and(instr: u16, reg: &mut Registers) {
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

/// Handles the NOT instruction (opcode 1001)
/// 
/// Format: 1001 DR SR 1 11111
///
/// Performs bitwise NOT on the contents of SR and stores the result in DR.
pub fn handle_not(instr: u16, reg: &mut Registers) {
    let dr = ((instr >> 9) & 0x7) as usize; 
    let sr1 = ((instr >> 6) & 0x7) as usize; 
    reg.set(dr, !reg.get(sr1));
}

/// Handles the BR (branch) instruction (opcode 0000)
/// 
/// Format: 0000 n z p PCoffset9
///
/// Conditionally branches to PC + offset if any enabled condition flag matches
/// the current processor condition.
pub fn handle_br(instr: u16, reg: &mut Registers) {
    let cond_flag = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);

    let cond = reg.get_cond_flag();
    if cond & cond_flag != 0 {
        let pc = reg.get_pc().wrapping_add(pc_offset);
        reg.set_pc(pc);
    }
}

/// Handles the JMP/RET instruction (opcode 1100)
/// 
/// Format: 1100 000 BaseR 000000
///
/// Jumps to the address specified in BaseR.
/// When BaseR is R7, this is a RET instruction.
pub fn handle_jmp(instr: u16, reg: &mut Registers) {
    let base_r = (instr >> 6) & 0x7;
    let address = reg.get(base_r as usize);
    reg.set_pc(address);
}

/// Handles the LD (load) instruction (opcode 0010)
/// 
/// Format: 0010 DR PCoffset9
///
/// Loads a value from memory at PC + offset into DR.
pub fn handle_ld(instr: u16, mem: &mut Memory, reg: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);
    let address = reg.get_pc().wrapping_add(pc_offset);
    let value = mem.read(address);
    reg.set(dr as usize, value);
}

/// Handles the ST (store) instruction (opcode 0011)
/// 
/// Format: 0011 SR PCoffset9
///
/// Stores the value in SR to memory at PC + offset.
pub fn handle_st(instr: u16, mem: &mut Memory, reg: &mut Registers) {
    let sr = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);
    let address = reg.get_pc().wrapping_add(pc_offset);
    let value = reg.get(sr as usize);
    mem.write(address, value);
}

/// Handles the JSR/JSRR instruction (opcode 0100)
/// 
/// Format: 0100 1 PCoffset11 | 0100 0 00 BaseR 000000
///
/// Saves the return address in R7 and jumps to either PC + offset (JSR)
/// or the address in BaseR (JSRR).
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

/// Handles the LDR (load register) instruction (opcode 0110)
/// 
/// Format: 0110 DR BaseR PCoffset6
///
/// Loads a value from memory at BaseR + offset into DR.
pub fn handle_ldr(instr: u16, mem: &mut Memory, reg: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let base_r = (instr >> 6) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x3F, 6);
    let address = reg.get(base_r as usize).wrapping_add(pc_offset);
    let value = mem.read(address);
    reg.set(dr as usize, value);
}

/// Handles the STR (store register) instruction (opcode 0111)
/// 
/// Format: 0111 SR BaseR PCoffset6
///
/// Stores the value in SR to memory at BaseR + offset.
pub fn handle_str(instr: u16, mem: &mut Memory, reg: &mut Registers) {
    let sr = (instr >> 9) & 0x7;
    let base_r = (instr >> 6) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x3F, 6);
    let address = reg.get(base_r as usize).wrapping_add(pc_offset);
    let value = reg.get(sr as usize);
    mem.write(address, value);
}

/// Handles the STI (store indirect) instruction (opcode 1011)
/// 
/// Format: 1011 SR PCoffset9
///
/// Stores the value in SR to memory at the address contained in 
/// the memory location at PC + offset.
pub fn handle_sti(instr: u16, mem: &mut Memory, reg: &mut Registers) {
    let sr = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);
    let address = reg.get_pc().wrapping_add(pc_offset);
    let value = reg.get(sr as usize);
    let target_addr = mem.read(address);
    mem.write(target_addr, value); 
}

/// Handles the LDI (load indirect) instruction (opcode 1010)
/// 
/// Format: 1010 DR PCoffset9
///
/// Loads a value into DR from the memory location whose address is stored
/// at the memory location PC + offset.
pub fn handle_ldi(instr: u16, mem: &mut Memory, reg: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);
    let address = reg.get_pc().wrapping_add(pc_offset);
    let indirect_addr = mem.read(address);
    let value = mem.read(indirect_addr);
    reg.set(dr as usize, value);
}

/// Handles the LEA (load effective address) instruction (opcode 1110)
/// 
/// Format: 1110 DR PCoffset9
///
/// Loads the address PC + offset into DR without accessing memory.
pub fn handle_lea(instr: u16, reg: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = crate::utils::sign_extend(instr & 0x1FF, 9);
    let address = reg.get_pc().wrapping_add(pc_offset);
    reg.set(dr as usize, address);
}