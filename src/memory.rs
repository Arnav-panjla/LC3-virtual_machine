use std::fs::File;
use std::io::{Read, Result};

pub struct Memory {
    mem: [u16; u16::MAX as usize],
}

impl Memory {
    pub fn new() -> Self {
        Self { mem: [0; u16::MAX as usize] }
    }

    pub fn read(&self, addr: u16) -> u16 {
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u16) {
        self.mem[addr as usize] = val;
    }

    pub fn load_program(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;
        let mut origin_bytes = [0; 2];
        file.read_exact(&mut origin_bytes)?;
        let origin = u16::from_be_bytes(origin_bytes);

        let mut offset = origin as usize;
        let mut buffer = [0; 2];
        while let Ok(_) = file.read_exact(&mut buffer) {
            self.mem[offset] = u16::from_be_bytes(buffer);
            offset += 1;
        }

        Ok(())
    }
}
