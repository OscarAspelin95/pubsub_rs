use crate::errors::PubSubError;
use google_cloud_pubsub::{
    client::{Client, ClientConfig},
    topic::Topic,
};
use tracing::{Level, event, instrument};

#[instrument]
pub async fn create_topic(client: &Client) -> Result<Topic, PubSubError> {
    let topic_as_str = std::env::var("PUBSUB_TOPIC")?;

    let topic = client.topic(&topic_as_str);

    match topic.exists(None).await {
        Ok(true) => {
            event!(Level::INFO, "Topic exists");

            Ok(topic)
        }
        Ok(false) => {
            event!(Level::INFO, "Creating topic");
            topic.create(None, None).await?;

            Ok(topic)
        }
        _ => Err(PubSubError::ClientError(
            "Failed to check is topic exists".to_string(),
        )),
    }
}
