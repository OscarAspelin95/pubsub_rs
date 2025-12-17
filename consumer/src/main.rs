use dotenv::dotenv;
use futures_util::StreamExt;
use google_cloud_pubsub::subscription::SubscriptionConfig;
use tokio;
use tracing::{Level, event};
use tracing_subscriber;
mod errors;
use common::{TestContract, create_client, create_topic};
use errors::ConsumerError;

#[tokio::main]
async fn main() -> Result<(), ConsumerError> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

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

    let mut stream = subscription
        .subscribe(None)
        .await
        .expect("Failed to subscribe");

    event!(Level::INFO, "Awaiting messages...");
    while let Some(message) = stream.next().await {
        event!(Level::INFO, "Received message: {:?}", message);

        let m = &message.message;
        let contract: TestContract = serde_json::from_slice(&m.data).expect("");

        event!(Level::INFO, "Parsed into {:?}", contract);

        let _ = message.ack().await;
    }

    Ok(())
}
