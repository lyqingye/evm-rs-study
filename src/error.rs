use thiserror::Error;

#[derive(Error, Debug)]
pub enum EVMError {
    #[error("insufficient balance")]
    InsufficientBalance,
    #[error("invalid jump destination")]
    InvalidJumpDestination,
    #[error("revert")]
    Revert,
    #[error("invalid opcode: {0}")]
    InvalidOpcode(u8),
    #[error("stop")]
    Stop,
}
