pub struct Registers {
    regs: [u16; 8],
    pc: u16,
    cond: ConditionFlag,
}

#[derive(Clone, Copy)]
pub enum ConditionFlag {
    POS = 1,
    ZRO = 2,
    NEG = 4,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            regs: [0; 8],
            pc: 0,
            cond: ConditionFlag::ZRO,
        }
    }

    pub fn get(&self, r: usize) -> u16 {
        self.regs[r]
    }

    pub fn set(&mut self, r: usize, val: u16) {
        self.regs[r] = val;
        self.update_flags(r);
    }

    pub fn update_flags(&mut self, r: usize) {
        let val = self.regs[r];
        self.cond = if val == 0 {
            ConditionFlag::ZRO
        } else if (val >> 15) == 1 {
            ConditionFlag::NEG
        } else {
            ConditionFlag::POS
        };
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
}
