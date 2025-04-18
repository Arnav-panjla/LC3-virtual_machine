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
    
    write!(stdout, "LC3 Virtual Machine\n").unwrap();
    write!(stdout, "Press Ctrl+C to exit.\n").unwrap();
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
            OpCode::TRAP => {
                let cont = trapcode::handle_trap(instr, &mut memory, &mut registers);
                if !cont {
                    break 'exec;
                }
            }
            _ => {
                println!("Unimplemented opcode: {:?}", opcode);
                break 'exec;
            }
        }
    }
}

