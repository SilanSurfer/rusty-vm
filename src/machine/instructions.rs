use super::error::MachineError;

pub enum Opcodes {
    Br,   /* branch */
    Add,  /* add  */
    Ld,   /* load */
    St,   /* store */
    Jsr,  /* jump register */
    And,  /* bitwise and */
    Ldr,  /* load register */
    Str,  /* store register */
    Rti,  /* unused */
    Not,  /* bitwise not */
    Ldi,  /* load indirect */
    Sti,  /* store indirect */
    Jmp,  /* jump */
    Res,  /* reserved (unused) */
    Lea,  /* load effective address */
    Trap, /* execute trap */
}

impl TryFrom<u16> for Opcodes {
    type Error = MachineError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcodes::Br),
            1 => Ok(Opcodes::Add),
            2 => Ok(Opcodes::Ld),
            3 => Ok(Opcodes::St),
            4 => Ok(Opcodes::Jsr),
            5 => Ok(Opcodes::And),
            6 => Ok(Opcodes::Ldr),
            7 => Ok(Opcodes::Str),
            8 => Ok(Opcodes::Rti),
            9 => Ok(Opcodes::Not),
            10 => Ok(Opcodes::Ldi),
            11 => Ok(Opcodes::Sti),
            12 => Ok(Opcodes::Jmp),
            13 => Ok(Opcodes::Res),
            14 => Ok(Opcodes::Lea),
            15 => Ok(Opcodes::Trap),
            _ => Err(MachineError::InvalidOpcode(value)),
        }
    }
}
