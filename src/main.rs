//! LC-3 Virtual Machine Implementation
//!
//! This module provides the main execution loop and program initialization for
//! the LC-3 virtual machine. It handles loading programs, processing instructions,
//! and interfacing with the terminal in raw mode for proper I/O operations.

pub mod memory;
pub mod register;
pub mod trapcode;
pub mod utils;
pub mod opcode;

use memory::Memory;
use register::Registers;
use opcode::OpCode;

use std::io::*;
use std::io::stdout as stdout_main; 
use termion::raw::*;

use std::panic;
use std::thread;
use signal_hook::{iterator::Signals, consts::SIGINT};



fn main() {

    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = stdout().into_raw_mode().unwrap().suspend_raw_mode();
        original_hook(panic_info);
    }));
    
    
    let mut raw_stdout = stdout_main().into_raw_mode().unwrap();
    let mut signals = Signals::new(&[SIGINT]).expect("Failed to register signal handler");
    thread::spawn(move || {
        for _ in signals.forever() {
            // Restore terminal settings when Ctrl+C is pressed
            let _ = stdout_main().into_raw_mode().unwrap().suspend_raw_mode();
            eprintln!("\r\nProgram terminated by user.");
            std::process::exit(0);
        }
    });

    write!(raw_stdout, "LC3 Virtual Machine\r\n").unwrap();
    raw_stdout.flush().unwrap();

    let mut memory = Memory::new();
    let mut registers = Registers::new();
    let pc : u16 = 0x3000;

    let program_path = std::env::args().nth(1).expect("Usage: lc3vm <program.obj>");
    memory.load_program(&program_path, pc).expect("Failed to load program");

    registers.set_pc(pc);

    'exec: loop {
        let pc = registers.get_pc();
        let instr = memory.read(pc);
        registers.increment_pc();

        let opcode = OpCode::from_instr(instr);

        match opcode {
            OpCode::ADD => opcode::handle_add(instr, &mut registers),
            OpCode::AND => opcode::handle_and(instr, &mut registers),
            OpCode::NOT => opcode::handle_not(instr, &mut registers),
            OpCode::BR => opcode::handle_br(instr, &mut registers),
            OpCode::JSR => opcode::handle_jsr(instr, &mut registers),
            OpCode::LD => opcode::handle_ld(instr, &mut memory, &mut registers),
            OpCode::LDR => opcode::handle_ldr(instr, &mut memory, &mut registers),
            OpCode::ST => opcode::handle_st(instr, &mut memory, &mut registers),
            OpCode::JMP => opcode::handle_jmp(instr, &mut registers),
            OpCode::LEA => opcode::handle_lea(instr, &mut registers),
            OpCode::STI => opcode::handle_sti(instr, &mut memory, &mut registers),
            OpCode::LDI => opcode::handle_ldi(instr, &mut memory, &mut registers),
            OpCode::STR => opcode::handle_str(instr, &mut memory, &mut registers),
            OpCode::RTI => {
                println!("RTI not implemented");
                break 'exec;
            }
            OpCode::RES => {
                println!("RES not implemented");
                break 'exec;
            }
            OpCode::TRAP => {
                let cont = trapcode::handle_trap(instr, &mut memory, &mut registers, &mut raw_stdout);
                if !cont {
                    break 'exec;
                }
            }
        }
    }
}