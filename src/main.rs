mod memory;
mod register;
mod trapcode;
mod utils;
mod opcode;

use termion::{clear, raw::IntoRawMode};
use std::io::{stdout, Write};
use memory::Memory;
use register::Registers;
use opcode::OpCode;

const PC_START: u16 = 0x3000;

fn main() {
    
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();
    
    write!(stdout, "LC3 Virtual Machine\r\n").unwrap();
    write!(stdout, "Press Ctrl+C to exit.\r\n").unwrap();
    stdout.flush().unwrap();

    let mut memory = Memory::new();
    let mut registers = Registers::new();

    let program_path = std::env::args().nth(1).expect("Usage: lc3vm <program.obj>");
    memory.load_program(&program_path).expect("Failed to load program");

    registers.set_pc(PC_START);

    'exec: loop {
        let pc = registers.get_pc();
        let instr = memory.read(pc);
        registers.increment_pc();

        let opcode = OpCode::from_instr(instr);

        match opcode {
            OpCode::ADD => opcode::handle_add(instr, &mut registers),
            OpCode::AND => opcode::handle_and(instr, &mut registers),
            OpCode::NOT => opcode::handle_not(instr, &mut registers),
            OpCode::BR => opcode::handle_br(instr, &mut registers), //Break
            OpCode::JSR => opcode::handle_jsr(instr, &mut registers), // Jump register
            OpCode::LD => opcode::handle_ld(instr, &mut memory, &mut registers), // Load
            OpCode::LDR => opcode::handle_ldr(instr, &mut memory, &mut registers), // Load register
            OpCode::ST => opcode::handle_st(instr, &mut memory, &mut registers), // Store
            OpCode::JMP => opcode::handle_jmp(instr, &mut registers), // Jump
            OpCode::LEA => opcode::handle_lea(instr, &mut memory, &mut registers), // Load effective address
            OpCode::STI => opcode::handle_sti(instr, &mut memory, &mut registers), // Store indirect
            OpCode::LDI => opcode::handle_ldi(instr, &mut memory, &mut registers), // Load indirect
            OpCode::STR => opcode::handle_str(instr, &mut memory, &mut registers), // Store register
            OpCode::RTI => {
                println!("RTI not implemented");
                break 'exec;
            }
            OpCode::RES => {
                println!("RES not implemented");
                break 'exec;
            }
            OpCode::TRAP => {
                let cont = trapcode::handle_trap(instr, &mut memory, &mut registers);
                if !cont {
                    break 'exec;
                }
            }
        }
    }
}

