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

    pub fn load_program(&mut self, path: &str, mut pc: u16) -> Result<()> {
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        
        let origin = if contents.len() >= 2 {
            u16::from_be_bytes([contents[0], contents[1]])
        } else {
            0x3000
        };
        
        let (start_offset, mem_offset) = if origin >= 0x3000 && origin <= 0x9000 {
            // with header
            (2, origin as usize)
        } else {
            // no header
            (0, 0x3000 as usize)
        };

        pc = mem_offset
            .try_into()
            .unwrap_or(0x3000);
        
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_program_with_header() {
        let mut mem = Memory::new();
        let pc = 0x3000;

        let binary: Vec<u8> = vec![
            0x40, 0x00, // origin = 0x4000
            0x12, 0x34, // instruction 1
            0xAB, 0xCD  // instruction 2
        ];

        std::fs::write("test_prog_with_header.obj", &binary).unwrap();
        mem.load_program("test_prog_with_header.obj", pc).unwrap();

        assert_eq!(mem.read(0x4000), 0x1234);
        assert_eq!(mem.read(0x4001), 0xABCD);
    }

    #[test]
    fn test_load_program_without_header() {
        let mut mem = Memory::new();
        let pc = 0x3000;

        let binary: Vec<u8> = vec![
            0x12, 0x34,
            0x56, 0x78
        ];

        std::fs::write("test_prog_no_header.obj", &binary).unwrap();
        mem.load_program("test_prog_no_header.obj", pc).unwrap();

        assert_eq!(mem.read(0x3000), 0x1234);
        assert_eq!(mem.read(0x3001), 0x5678);
    }
}
