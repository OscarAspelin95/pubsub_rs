use common::{PubSubError, create_client, create_topic};
use google_cloud_pubsub::{client::Client, topic::Topic};
use tracing::{Level, event};

#[derive(Debug, Clone)]
pub struct AppState {
    pub client: Client,
    pub topic: Topic,
}

pub async fn get_state() -> Result<AppState, PubSubError> {
    event!(Level::INFO, "Creating client...");
    let client = create_client().await?;
    event!(Level::INFO, "Getting topic...");
    let topic = create_topic(&client).await?;

    let state = AppState { client, topic };
    Ok(state)
}
