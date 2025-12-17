use google_cloud_pubsub;
use std;
use thiserror::Error;
use tonic::Status;

#[derive(Debug, Error)]
pub enum PubSubError {
    #[error("Failed to create client")]
    ClientError(#[from] google_cloud_pubsub::client::Error),

    #[error("Environment variables not set")]
    EnvError(#[from] std::env::VarError),

    #[error("Failed to create topic")]
    TopicCreationError(String),

    #[error("Unknown error")]
    UnknownError(#[from] Status),
}
