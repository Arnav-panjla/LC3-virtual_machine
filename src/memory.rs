use std::fs::File;
use std::io::{Read, Result};

pub struct Memory {
    mem: [u16; 0x10000],

}

impl Memory {
    pub fn new() -> Self {
        Self { mem: [0; 0x10000] }
    }

    pub fn read(&self, addr: u16) -> u16 {
        if (addr as usize) >= self.mem.len() {
            return 0;
        }
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u16) {
        if (addr as usize) < self.mem.len() {
            self.mem[addr as usize] = val;
        }
    }

    pub fn load_program(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        
        // Check if first two bytes could be a valid origin address
        let origin = if contents.len() >= 2 {
            u16::from_be_bytes([contents[0], contents[1]])
        } else {
            // Default to standard PC_START if file is too short
            0x3000
        };
        
        // If origin looks invalid (not in reasonable range), assume file has no header
        let (start_offset, mem_offset) = if origin >= 0x3000 && origin <= 0x9000 {
            // Normal file with header
            (2, origin as usize)
        } else {
            // No header, assume PC_START
            (0, 0x3000)
        };
        
        // Load instructions
        for i in (start_offset..contents.len()).step_by(2) {
            if i + 1 < contents.len() {
                let instr = u16::from_be_bytes([contents[i], contents[i + 1]]);
                let offset = mem_offset + (i - start_offset) / 2;
                if offset < self.mem.len() {
                    self.mem[offset] = instr;
                }
            }
        }
        
        Ok(())
    }
}
