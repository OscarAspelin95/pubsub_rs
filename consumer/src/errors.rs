use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConsumerError {
    #[error("")]
    UnknownError(String),
}
