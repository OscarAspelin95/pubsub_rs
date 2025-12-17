use thiserror::Error;

#[derive(Debug, Error)]
pub enum PubSubError {
    #[error("Failed to create client")]
    ClientError(String),

    #[error("Environment variables not set")]
    EnvError(String),

    #[error("Failed to create topic")]
    TopicCreationError(String),
}

impl From<google_cloud_pubsub::client::Error> for PubSubError {
    fn from(err: google_cloud_pubsub::client::Error) -> Self {
        PubSubError::ClientError(err.to_string())
    }
}

impl From<std::env::VarError> for PubSubError {
    fn from(err: std::env::VarError) -> Self {
        PubSubError::EnvError(err.to_string())
    }
}

impl From<tonic::Status> for PubSubError {
    fn from(err: tonic::Status) -> Self {
        PubSubError::TopicCreationError(err.to_string())
    }
}
