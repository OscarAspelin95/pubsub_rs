use crate::errors::PubSubError;
use google_cloud_pubsub::client::{Client, ClientConfig};
use tracing::instrument;

#[instrument]
pub async fn create_client() -> Result<Client, PubSubError> {
    let client = Client::new(ClientConfig::default()).await?;

    Ok(client)
}
