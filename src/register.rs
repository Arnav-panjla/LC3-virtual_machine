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
        } else if (val >> 15) != 0 {
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
    pub fn get_cond_flag(&self) -> u16 {
        match self.cond {
            ConditionFlag::POS => 1,
            ConditionFlag::ZRO => 2,
            ConditionFlag::NEG => 4,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_get_set() {
        let mut regs = Registers::new();
        regs.set(3, 0xABCD);
        assert_eq!(regs.get(3), 0xABCD);
    }

    #[test]
    fn test_update_flags_positive() {
        let mut regs = Registers::new();
        regs.set(4, 0x1234);  // Normal positive value
        assert_eq!(regs.get_cond_flag(), 1);
    }

    #[test]
    fn test_program_counter_set_get() {
        let mut regs = Registers::new();
        regs.set_pc(0x3000);
        assert_eq!(regs.get_pc(), 0x3000);
    }

    #[test]
    fn test_program_counter_increment() {
        let mut regs = Registers::new();
        regs.set_pc(0xFFFF);
        regs.increment_pc();
        assert_eq!(regs.get_pc(), 0x0000);  // wrapping around
    }
}

