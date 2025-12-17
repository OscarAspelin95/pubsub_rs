use crate::errors::PubSubError;
use google_cloud_pubsub::{client::Client, topic::Topic};
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
            topic
                .create(None, None)
                .await
                .map_err(|err| PubSubError::TopicCreationError(err.to_string()))?;

            Ok(topic)
        }
        Err(err) => Err(PubSubError::TopicCreationError(err.to_string())),
    }
}
