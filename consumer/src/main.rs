use common::{PubSubError, TestContract};
use dotenv::dotenv;
use futures_util::StreamExt;
use tokio;
use tracing::{Level, event};
use tracing_subscriber;

mod subscriber;
use subscriber::create_subscriber;

#[tokio::main]
async fn main() -> Result<(), PubSubError> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let subscription = create_subscriber().await?;

    let mut stream = subscription
        .subscribe(None)
        .await
        .expect("Failed to subscribe");

    event!(Level::INFO, "Awaiting messages...");
    while let Some(message) = stream.next().await {
        event!(Level::INFO, "Received message: {:?}", message);

        let m = &message.message;
        let contract: TestContract = serde_json::from_slice(&m.data)
            .expect("Failed to serialize data from incoming message.");

        event!(Level::INFO, "Parsed into {:?}", contract);

        let _ = message.ack().await;
    }

    Ok(())
}
