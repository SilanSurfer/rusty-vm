use thiserror::Error;

#[derive(Error, Debug)]
pub enum MachineError {
    #[error("Not supported opcode ({0})")]
    InvalidOpcode(u16),
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}
