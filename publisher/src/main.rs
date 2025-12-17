use axum::{self, Json, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use common::{PubSubError, TestContract, create_client, create_topic};
use dotenv::dotenv;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::{client::Client, topic::Topic};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio;
use tower_http::cors::CorsLayer;
use tracing::{Level, event};
use tracing_subscriber;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    message: String,
}

async fn publish_message(
    State(state): State<AppState>,
    Json(msg): Json<Message>,
) -> impl IntoResponse {
    let publisher = state.topic.new_publisher(None);
    let msg = TestContract {
        message: msg.message,
    };

    let publish_process = tokio::spawn(async move {
        event!(Level::INFO, "Publishing message...");
        let pubsub_msg = PubsubMessage {
            data: serde_json::to_vec(&msg).expect(""),
            ..Default::default()
        };
        let awaiter = publisher.publish(pubsub_msg).await;

        awaiter.get().await
    });

    publish_process.await.expect("").expect("");

    (
        StatusCode::OK,
        Json(json!({"response": "message successfully published"})),
    )
        .into_response()
}

#[derive(Debug, Clone)]
pub struct AppState {
    client: Client,
    topic: Topic,
}

#[tokio::main]
async fn main() -> Result<(), PubSubError> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    event!(Level::INFO, "Creating client...");
    let client = create_client().await?;

    event!(Level::INFO, "Getting topic...");
    let topic = create_topic(&client).await?;

    let state = AppState { client, topic };

    // Later, we'll put this in the axum handler.
    let router = axum::Router::new()
        .route("/send_message", post(publish_message))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address.");

    event!(Level::INFO, "Starting server...");
    axum::serve(listener, router)
        .await
        .expect("Failed to start server");

    Ok(())
}
