use crate::{memory::Memory, register::Registers};
use std::io::{self, Read, Write};

pub fn handle_trap(instr: u16, memory: &mut Memory, registers: &mut Registers) -> bool {
    let trap_vector = instr & 0xFF;
    match trap_vector {
        0x20 => {
            // GETC: Read a single character from the keyboard (no echo)
            let mut buffer = [0];
            io::stdin().read_exact(&mut buffer).unwrap();
            registers.set(0, buffer[0] as u16);
        }
        0x21 => {
            // OUT: Output the character in R0[7:0] to the console
            let ch = (registers.get(0) & 0xFF) as u8 as char;
            print!("{}", ch);
            io::stdout().flush().unwrap();
        }
        0x22 => {
            // PUTS: Print a null-terminated string stored in memory starting at address in R0
            let mut addr = registers.get(0);
            loop {
                let ch = memory.read(addr);
                if ch == 0 {
                    break;
                }
                print!("{}", (ch & 0xFF) as u8 as char);
                addr = addr.wrapping_add(1);
            }
            io::stdout().flush().unwrap();
        }
        0x23 => {
            // IN: Prompt for a character and store it in R0 (with echo)
            print!("Enter a character: ");
            io::stdout().flush().unwrap();
            let mut buffer = [0];
            io::stdin().read_exact(&mut buffer).unwrap();
            let input_char = buffer[0];
            registers.set(0, input_char as u16);
            print!("{}", input_char as char);
            io::stdout().flush().unwrap();
        }
        0x24 => {
            // PUTSP: Print a string stored in memory using two characters per word (packed)
            let mut addr = registers.get(0);
            loop {
                let val = memory.read(addr);
                let ch1 = (val & 0xFF) as u8;
                if ch1 == 0 {
                    break;
                }
                print!("{}", ch1 as char);

                let ch2 = (val >> 8) as u8;
                if ch2 == 0 {
                    break;
                }
                print!("{}", ch2 as char);

                addr = addr.wrapping_add(1);
            }
            io::stdout().flush().unwrap();
        }
        0x25 => {
            // HALT: Stop the program
            println!("\nHALT");
            return false;
        }
        _ => {
            println!("TRAP 0x{:02X} not implemented", trap_vector);
        }
    }

    true
}
