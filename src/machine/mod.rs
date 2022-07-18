pub const MEMORY_MAX: usize = 65536;
pub const R_COUNT: usize = 10;
pub const PC_START: u16 = 0x3000;

pub mod error;
pub mod instructions;
pub mod registers;

use byteorder::{BigEndian, ReadBytesExt};
use error::MachineError;
use registers::RegisterType;
use std::io::Read;

pub enum CondFlags {
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}

pub struct Machine {
    memory: Vec<u16>,
    registers: registers::Registers,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            memory: vec![0; MEMORY_MAX],
            registers: registers::Registers::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), MachineError> {
        self.registers
            .update_register(CondFlags::ZRO as u16, RegisterType::COND);

        let instr: u16 = self.memory[self.registers.register(RegisterType::PC) as usize];
        let op: u16 = instr >> 12;
        self.registers.inc_pc();

        loop {
            match op.try_into()? {
                instructions::Opcodes::Add => todo!(),
                instructions::Opcodes::Br => todo!(),
                instructions::Opcodes::Ld => todo!(),
                instructions::Opcodes::St => todo!(),
                instructions::Opcodes::Jsr => todo!(),
                instructions::Opcodes::And => todo!(),
                instructions::Opcodes::Ldr => todo!(),
                instructions::Opcodes::Str => todo!(),
                instructions::Opcodes::Rti => todo!(),
                instructions::Opcodes::Not => todo!(),
                instructions::Opcodes::Ldi => todo!(),
                instructions::Opcodes::Sti => todo!(),
                instructions::Opcodes::Jmp => todo!(),
                instructions::Opcodes::Res => todo!(),
                instructions::Opcodes::Lea => todo!(),
                instructions::Opcodes::Trap => todo!(),
            }
        }

        Ok(())
    }

    pub fn save_program_to_memory(&mut self, mut program: impl Read) -> Result<(), MachineError> {
        let mut start = program.read_u16::<BigEndian>()? as usize;
        while let Ok(instr) = program.read_u16::<BigEndian>() {
            self.memory[start] = instr;
            start += 1;
        }
        Ok(())
    }
}
