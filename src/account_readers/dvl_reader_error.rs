use std::error::Error;

pub enum AccountReadError {
    Client(solana_client::client_error::ClientError),
    DecodeError(String),
    Other(String),
}

impl From<solana_client::client_error::ClientError> for AccountReadError {
    fn from(err: solana_client::client_error::ClientError) -> Self {
        AccountReadError::Client(err)
    }
}

impl From<Box<dyn Error>> for AccountReadError {
    fn from(err: Box<dyn Error>) -> Self {
        AccountReadError::Other(err.to_string())
    }
}