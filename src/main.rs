//! LC-3 Virtual Machine Implementation
//!
//! This module provides the main execution loop and program initialization for
//! the LC-3 virtual machine. It handles loading programs, processing instructions,
//! and interfacing with the terminal in raw mode for proper I/O operations.

mod memory;
mod register;
mod trapcode;
mod utils;
mod opcode;

use memory::Memory;
use register::Registers;
use opcode::OpCode;

use std::io::*;
use std::io::stdout as stdout_main; 
use termion::raw::*;

use std::panic;
use std::thread;
use signal_hook::{iterator::Signals, consts::SIGINT};

/// The starting memory address for LC-3 programs
const PC_START: u16 = 0x3000;

/// A guard struct that ensures terminal settings are properly restored
/// when the program exits, even in case of panics or crashes
struct TerminalGuard <T : termion::raw::IntoRawMode> {
    /// The raw terminal instance being protected
    terminal: T,
}

impl<T: termion::raw::IntoRawMode> Drop for TerminalGuard<T> {
    /// Restores terminal settings when the guard goes out of scope
    fn drop(&mut self) {
        // Simply dropping the terminal will restore the previous terminal mode
        // as RawTerminal implements Drop
    }
}

/// The main entry point for the LC-3 virtual machine
///
/// This function:
/// 1. Sets up terminal handling in raw mode
/// 2. Registers signal handlers and panic hooks for clean termination
/// 3. Loads a program from a file specified as a command-line argument
/// 4. Runs the main instruction execution loop until program termination
fn main() {
    // Set panic hook to ensure terminal is restored on panic
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal settings before displaying panic information
        let _ = stdout().into_raw_mode().unwrap().suspend_raw_mode();
        original_hook(panic_info);
    }));
    
    // Create guarded terminal to ensure settings are restored on exit
    let stdout = stdout_main().into_raw_mode().unwrap();
    let mut raw_stdout = TerminalGuard { terminal: stdout };
    
    // Set up signal handler for Ctrl+C (SIGINT)
    let mut signals = Signals::new(&[SIGINT]).expect("Failed to register signal handler");
    thread::spawn(move || {
        for _ in signals.forever() {
            // Restore terminal settings when Ctrl+C is pressed
            let _ = stdout_main().into_raw_mode().unwrap().suspend_raw_mode();
            eprintln!("\r\nProgram terminated by user.");
            std::process::exit(0);
        }
    });

    // Display welcome message
    write!(raw_stdout.terminal, "LC3 Virtual Machine\r\n").unwrap();
    write!(raw_stdout.terminal, "Press Ctrl+C to exit.\r\n").unwrap();
    raw_stdout.terminal.flush().unwrap();

    // Initialize VM memory and registers
    let mut memory = Memory::new();
    let mut registers = Registers::new();

    // Load program from command line argument
    let program_path = std::env::args().nth(1).expect("Usage: lc3vm <program.obj>");
    memory.load_program(&program_path).expect("Failed to load program");

    // Set program counter to standard LC-3 program start address
    registers.set_pc(PC_START);

    // Main execution loop - fetch, decode, execute cycle
    'exec: loop {
        // Fetch: Get the current instruction from memory
        let pc = registers.get_pc();
        let instr = memory.read(pc);
        registers.increment_pc();

        // Decode: Extract the opcode from the instruction
        let opcode = OpCode::from_instr(instr);

        // Execute: Process the instruction based on its opcode
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
                // Return from interrupt - not implemented in this VM
                println!("RTI not implemented");
                break 'exec;
            }
            OpCode::RES => {
                // Reserved instruction - not implemented
                println!("RES not implemented");
                break 'exec;
            }
            OpCode::TRAP => {
                // System call - handle and check if execution should continue
                let cont = trapcode::handle_trap(instr, &mut memory, &mut registers, &mut raw_stdout.terminal);
                if !cont {
                    break 'exec;
                }
            }
        }
    }
}