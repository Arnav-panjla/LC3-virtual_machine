use lc3_vm::memory::Memory;
use lc3_vm::register::Registers;
use lc3_vm::opcode;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_instruction() {
        let mut registers = Registers::new();
        
        // Set initial values
        registers.set(1, 5); // R1 = 5
        
        let instructions = vec![
            0b0001010001100111, // ADD R2, R1, #7  ; R2 = R1 + 7 = 5 + 7 = 12
            0b0001011001000010, // ADD R3, R1, R2  ; R3 = R1 + R2 = 5 + 12 = 17
        ];
        
        for instr in instructions {
            opcode::handle_add(instr, &mut registers);
        }
        
        assert_eq!(registers.get(2), 12);
        assert_eq!(registers.get(3), 17);
    }
    
    #[test]
    fn test_br_instruction() {
        let mut registers = Registers::new();
        
        // Set PC
        registers.set_pc(0x3000);
        
        // Set condition codes (negative flag)
        // Instead of passing 0xFFFF directly, use a negative value
        // Reset PC and set Z flag
        registers.set_pc(0x3000);
        registers.set(0, 0); // This will set the ZRO flag
        
        let instructions = vec![
            0b0000010000000101, // BRz #5 ; Branch if zero (PC = 0x3000 + 5 = 0x3005)
        ];
        
        opcode::handle_br(instructions[0], &mut registers);
        assert_eq!(registers.get_pc(), 0x3005);
        
        // Reset PC and set Z flag
        registers.set_pc(0x3000);
        registers.update_flags(0); // This should set the Z flag since value is zero
        
        let instructions = vec![
            0b0000010000000101, // BRz #5 ; Branch if zero (PC = 0x3000 + 5 = 0x3005)
        ];
        
        opcode::handle_br(instructions[0], &mut registers);
        assert_eq!(registers.get_pc(), 0x3005);
    }
    
    #[test]
    fn test_jmp_instruction() {
        let mut registers = Registers::new();
        
        // Set base register value
        registers.set(3, 0x4000);
        
        let instructions = vec![
            0b1100000011000000, // JMP R3 ; Jump to address in R3 (PC = 0x4000)
        ];
        
        opcode::handle_jmp(instructions[0], &mut registers);
        assert_eq!(registers.get_pc(), 0x4000);
    }
    
    #[test]
    fn test_jsr_instruction() {
        let mut registers = Registers::new();
        
        registers.set_pc(0x3000);
        
        let instructions = vec![
            0b0100100000010100, // JSR #20 ; Jump to subroutine at offset 20, save PC in R7
        ];
        
        opcode::handle_jsr(instructions[0], &mut registers);
        assert_eq!(registers.get(7), 0x3000); // R7 should contain the original PC
        assert_eq!(registers.get_pc(), 0x3014); // PC should be 0x3000 + 20 = 0x3014
        
        // Test JSRR
        registers.set_pc(0x3000);
        registers.set(2, 0x4000);
        
        let instructions = vec![
            0b0100000010000000, // JSRR R2 ; Jump to subroutine at address in R2, save PC in R7
        ];
        
        opcode::handle_jsr(instructions[0], &mut registers);
        assert_eq!(registers.get(7), 0x3000); // R7 should contain the original PC
        assert_eq!(registers.get_pc(), 0x4000); // PC should be value from R2
    }
    
    #[test]
    fn test_ld_and_st_instructions() {
        let mut memory = Memory::new();
        let mut registers = Registers::new();
        
        registers.set_pc(0x3000);
        memory.write(0x3005, 42); // Write value 42 at address 0x3005
        
        let instructions = vec![
            0b0010001000000101, // LD R1, #5 ; Load from 0x3000 + 5 into R1
            0b0011010000001010, // ST R2, #10 ; Store R2 to address 0x3000 + 10 = 0x300A
        ];
        
        opcode::handle_ld(instructions[0], &mut memory, &mut registers);
        assert_eq!(registers.get(1), 42); // R1 should now equal 42
        
        registers.set(2, 100);
        opcode::handle_st(instructions[1], &mut memory, &mut registers);
        assert_eq!(memory.read(0x300A), 100); // Memory at 0x300A should now be 100
    }
    
    #[test]
    fn test_ldr_and_str_instructions() {
        let mut memory = Memory::new();
        let mut registers = Registers::new();
        
        registers.set(3, 0x4000); // Base register
        memory.write(0x4005, 42); // Write value 42 at address 0x4005
        
        let instructions = vec![
            0b0110001011000101, // LDR R1, R3, #5 ; Load from 0x4000 + 5 into R1
            0b0111010011001010, // STR R2, R3, #10 ; Store R2 to address 0x4000 + 10 = 0x400A
        ];
        
        opcode::handle_ldr(instructions[0], &mut memory, &mut registers);
        assert_eq!(registers.get(1), 42); // R1 should now equal 42
        
        registers.set(2, 100);
        opcode::handle_str(instructions[1], &mut memory, &mut registers);
        assert_eq!(memory.read(0x400A), 100); // Memory at 0x400A should now be 100
    }
    
    #[test]
    fn test_ldi_and_sti_instructions() {
        let mut memory = Memory::new();
        let mut registers = Registers::new();
        
        registers.set_pc(0x3000);
        memory.write(0x3005, 0x4000); // Address to indirect memory location
        memory.write(0x4000, 42);     // Value at indirect location
        
        let instructions = vec![
            0b1010001000000101, // LDI R1, #5 ; Load from memory[memory[0x3000 + 5]] into R1
            0b1011010000001010, // STI R2, #10 ; Store R2 to memory[memory[0x3000 + 10]]
        ];
        
        opcode::handle_ldi(instructions[0], &mut memory, &mut registers);
        assert_eq!(registers.get(1), 42); // R1 should now equal 42
        
        registers.set(2, 100);
        memory.write(0x300A, 0x5000); // Add pointer for STI
        opcode::handle_sti(instructions[1], &mut memory, &mut registers);
        assert_eq!(memory.read(0x5000), 100); // Memory at indirect location should now be 100
    }
    
    #[test]
    fn test_not_instruction() {
        let mut registers = Registers::new();
        
        registers.set(1, 0b1010);
        
        let instructions = vec![
            0b1001010001111111, // NOT R2, R1 ; R2 = ~R1 = ~0b1010 = 0b...11110101
        ];
        
        opcode::handle_not(instructions[0], &mut registers);
        assert_eq!(registers.get(2), !0b1010);
    }
    
    #[test]
    fn test_lea_instruction() {
        let mut registers = Registers::new();
        
        registers.set_pc(0x3000);
        
        let instructions = vec![
            0b1110001000000101, // LEA R1, #5 ; R1 = PC + 5 = 0x3000 + 5 = 0x3005
        ];
        
        opcode::handle_lea(instructions[0], &mut registers);
        assert_eq!(registers.get(1), 0x3005);
    }
}