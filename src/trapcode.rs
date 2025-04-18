use crate::{memory::Memory, register::Registers};

pub fn handle_trap(instr: u16, memory: &mut Memory, registers: &mut Registers) -> bool {
    let trap_vector = instr & 0xFF;
    match trap_vector {
        0x25 => {
            println!("HALT");
            return false;
        }
        _ => {
            println!("TRAP 0x{:02X} not implemented", trap_vector);
        }
    }
    true
}
