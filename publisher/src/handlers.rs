use crate::state::AppState;
use axum::{self, Json, extract::State, http::StatusCode, response::IntoResponse};
use common::TestContract;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio;
use tracing::{Level, event};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}
pub async fn publish_message(
    State(state): State<AppState>,
    Json(msg): Json<Message>,
) -> impl IntoResponse {
    let publisher = state.topic.new_publisher(None);
    let msg = TestContract {
        id: Uuid::now_v7(),
        message: msg.message,
    };

    let publish_process = tokio::spawn(async move {
        event!(Level::INFO, "Publishing message...");
        let pubsub_msg = PubsubMessage {
            data: serde_json::to_vec(&msg).expect("Failed to serialize message"),
            ..Default::default()
        };
        let awaiter = publisher.publish(pubsub_msg).await;

        awaiter.get().await
    });

    let publish_result = publish_process
        .await
        .expect("Failed to get publish result.");

    match publish_result {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"response": "message successfully published"})),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": err.to_string()})),
        )
            .into_response(),
    }
}

pub async fn health(State(_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"response": "healthy"}))).into_response()
}
