#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Invalid(&'static str),
    #[error("Resource not found")]
    NotFound,
    #[error("Operation conflicts with current state")]
    Conflict,
    // #[error("Insufficient stock")]
    // InsufficientStock,
    #[error("Request rate exceeded")]
    RateLimited,
    // #[error("Payment provider is not configured")]
    // ProviderUnavailable,
    #[error("Storage temporarily unavailable; retry with the same idempotency key")]
    Storage,
}
pub type Result<T> = std::result::Result<T, Error>;
