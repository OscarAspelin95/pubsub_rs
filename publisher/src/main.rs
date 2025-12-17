use axum;
use common::PubSubError;
use dotenv::dotenv;
use tokio;
use tracing::{Level, event};
use tracing_subscriber;

mod state;
use state::get_state;

mod handlers;

mod router;
use router::create_router;

#[tokio::main]
async fn main() -> Result<(), PubSubError> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let state = get_state().await?;
    let router = create_router(state).await;

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    event!(Level::INFO, "Binding to address {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address.");

    event!(Level::INFO, "Starting server...");
    axum::serve(listener, router)
        .await
        .expect("Failed to start server");

    Ok(())
}
