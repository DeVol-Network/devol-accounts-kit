use thiserror::Error;

#[derive(Error, Debug)]
pub enum DvlOffChainError {
    #[error("The option basket cannot contain more than four positions")]
    BasketTooLarge,
}