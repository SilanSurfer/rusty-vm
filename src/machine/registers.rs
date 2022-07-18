pub enum RegisterType {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
}

pub struct Registers {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,
    pc: u16, /* program counter */
    cond: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: crate::machine::PC_START,
            cond: 0,
        }
    }

    pub fn register(&self, reg: RegisterType) -> u16 {
        match reg {
            RegisterType::R0 => self.r0,
            RegisterType::R1 => self.r1,
            RegisterType::R2 => self.r2,
            RegisterType::R3 => self.r3,
            RegisterType::R4 => self.r4,
            RegisterType::R5 => self.r5,
            RegisterType::R6 => self.r6,
            RegisterType::R7 => self.r7,
            RegisterType::PC => self.pc,
            RegisterType::COND => self.cond,
        }
    }

    pub fn update_register(&mut self, val: u16, reg: RegisterType) {
        match reg {
            RegisterType::R0 => self.r0 = val,
            RegisterType::R1 => self.r1 = val,
            RegisterType::R2 => self.r2 = val,
            RegisterType::R3 => self.r3 = val,
            RegisterType::R4 => self.r4 = val,
            RegisterType::R5 => self.r5 = val,
            RegisterType::R6 => self.r6 = val,
            RegisterType::R7 => self.r7 = val,
            RegisterType::PC => self.pc = val,
            RegisterType::COND => self.cond = val,
        }
    }

    pub fn inc_pc(&mut self) {
        self.pc += 1
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_update_proper_register() {
        // Arrange
        let mut reg = Registers::new();
        assert_eq!(reg.register(RegisterType::R0), 0);

        // Act
        reg.update_register(4, RegisterType::R0);

        // Assert
        assert_eq!(reg.register(RegisterType::R0), 4);
    }

    #[test]
    fn should_increment_reqister_pc() {
        // Arrange
        let mut reg = Registers::new();
        assert_eq!(reg.register(RegisterType::PC), 0x3000);

        // Act
        reg.inc_pc();

        // Assert
        assert_eq!(reg.register(RegisterType::PC), 0x3001);
    }
}
