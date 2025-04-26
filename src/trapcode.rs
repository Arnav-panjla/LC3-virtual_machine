use crate::{memory::Memory, register::Registers};
use std::io::{stdin, Read, Stdout, Write};
use termion::raw::*;

/// Handles the TRAP instruction execution (opcode 1111)
///
/// TRAP is used for system calls in the LC-3 architecture.
/// It provides services like character input/output and program termination.
///
/// # Arguments
///
/// * `instr` - The 16-bit instruction word containing the trap vector
/// * `memory` - Mutable reference to the VM's memory
/// * `registers` - Mutable reference to the VM's registers
/// * `stdout` - Mutable reference to the terminal output in raw mode
///
/// # Returns
///
/// * `true` if execution should continue
/// * `false` if the program should halt
pub fn handle_trap(
    instr: u16,
    memory: &mut Memory,
    registers: &mut Registers,
    stdout: &mut RawTerminal<Stdout>,
) -> bool {
    // Extract the trap vector (8-bit code) from the instruction
    let trap_vector = instr & 0xFF;
    
    // Save the return address in R7
    registers.set(7, registers.get_pc()); 
    
    match trap_vector {
        0x20 => {
            // GETC: Read a single character without echo
            // Places the character in R0
            let stdin = stdin();
            stdout.flush().unwrap();
            let c = stdin.lock().bytes().next().unwrap().unwrap();
            registers.set(0, c as u16);
        }
        0x21 => {
            // OUT: Output a single character
            // Displays the character in R0
            let char_code = registers.get(0) as u8;
            write!(stdout, "{}", char_code as char).unwrap();
            stdout.flush().unwrap();
        }
        0x22 => {
            // PUTS: Output a null-terminated string
            // String pointer is in R0, each memory location contains one character
            let mut addr = registers.get(0);
            loop {
                let ch = memory.read(addr);
                if ch == 0 {
                    break;
                }
                write!(stdout, "{}", (ch & 0xFF) as u8 as char).unwrap();
                addr = addr.wrapping_add(1);
            }
            stdout.flush().unwrap();
        }
        0x23 => {
            // IN: Input a character with prompt and echo
            // Prompts for input, reads a character, echoes it, and stores it in R0
            write!(stdout, "Enter a character: ").unwrap();
            stdout.flush().unwrap();
            let c = stdin().lock().bytes().next().unwrap().unwrap();
            registers.set(0, c as u16);
            write!(stdout, "{}", c as char).unwrap();
            stdout.flush().unwrap();
        }
        0x24 => {
            // PUTSP: Output a null-terminated string packed in 16-bit words
            // String pointer is in R0, each memory location contains two characters
            // (first character in low byte, second in high byte)
            let mut addr = registers.get(0);
            loop {
                let val = memory.read(addr);
                
                // First character is in the low byte
                let ch1 = (val & 0xFF) as u8;
                if ch1 == 0 {
                    break;
                }
                write!(stdout, "{}", ch1 as char).unwrap();

                // Second character is in the high byte
                let ch2 = (val >> 8) as u8;
                if ch2 == 0 {
                    break;
                }
                write!(stdout, "{}", ch2 as char).unwrap();

                addr = addr.wrapping_add(1);
            }
            stdout.flush().unwrap();
        }
        0x25 => {
            // HALT: Stops program execution
            // Returns false to signal the main loop to terminate
            write!(stdout, "\r\nHALT\r\n").unwrap();
            stdout.flush().unwrap();
            return false;
        }
        _ => {
            // Handle unimplemented trap codes
            write!(stdout, "TRAP 0x{:02X} not implemented\r\n", trap_vector).unwrap();
            stdout.flush().unwrap();
        }
    }

    // Continue execution
    true
}