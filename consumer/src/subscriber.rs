use crate::errors::ConsumerError;
use common::{create_client, create_topic};
use google_cloud_pubsub::subscription::{Subscription, SubscriptionConfig};
use tracing::{Level, event};

pub async fn create_subscriber() -> Result<Subscription, ConsumerError> {
    event!(Level::INFO, "Creating client...");
    let client = create_client()
        .await
        .map_err(|err| ConsumerError::UnknownError(err.to_string()))?;

    event!(Level::INFO, "Getting topic...");
    let topic = create_topic(&client)
        .await
        .map_err(|err| ConsumerError::UnknownError(err.to_string()))?;

    // Now, we need a subscriber.
    let config = SubscriptionConfig {
        enable_message_ordering: true,
        ..Default::default()
    };

    let subscription = client.subscription("test-subscription");

    if !subscription
        .exists(None)
        .await
        .map_err(|err| ConsumerError::UnknownError(err.to_string()))?
    {
        event!(Level::INFO, "Subscription does not exist, creating...");
        subscription
            .create(topic.fully_qualified_name(), config, None)
            .await
            .map_err(|err| ConsumerError::UnknownError(err.to_string()))?;
    }

    Ok(subscription)
}
