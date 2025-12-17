use common::PubSubError;
use common::{create_client, create_topic};
use google_cloud_pubsub::subscription::{Subscription, SubscriptionConfig};
use std;
use tracing::{Level, event};
pub async fn create_subscriber() -> Result<Subscription, PubSubError> {
    event!(Level::INFO, "Creating client...");
    let client = create_client().await?;

    event!(Level::INFO, "Getting topic...");
    let topic = create_topic(&client).await?;

    let config = SubscriptionConfig {
        enable_message_ordering: true,
        ..Default::default()
    };

    let subscription_name =
        std::env::var("PUBSUB_SUBSCRIPTION").unwrap_or_else(|_| "test-subscription".into());
    let subscription = client.subscription(&subscription_name);

    if !subscription.exists(None).await? {
        event!(Level::INFO, "Subscription does not exist, creating...");
        subscription
            .create(topic.fully_qualified_name(), config, None)
            .await?;
    }

    Ok(subscription)
}
