use thiserror::Error;

#[derive(Debug, Error)]
pub enum MPCError {
    #[error("Invalid number of participants: {0}")]
    InvalidParticipants(usize),

    #[error("Invalid threshold: {0}")]
    InvalidThreshold(usize),

    #[error("Insufficient shares for reconstruction")]
    InsufficientShares,

    #[error("Invalid signature share")]
    InvalidShare,

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Verification failed")]
    VerificationFailed,
}
