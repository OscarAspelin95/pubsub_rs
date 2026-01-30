# pubsub_rs
Basic example of using GCP PubSub for containerized applications. We have a containerized axum webserver (publisher) that exposes a POST endpoint and that will publish a message for the containerized consumer to receive.

## Requirements:
- `Docker`

## Environment
A .env file with the following content
```
PORT=8080
PUBSUB_TOPIC=<pubsub_topic_name>
PUBSUB_SUBSCRIPTION=<pubsub_subscription_name>
```

## Usage
1. `make` - start services.
2. `docker compose logs -f rust-consumer` - 
3. Make a POST request to the publisher with your favorite API testing tool (e.g., PostMan or APIdog) to `http://localhost:8080/send_message` with a body of type `{"message": "my_message"}`.
4. Check that this message is received by the consumer.

## Notes
This repo needs some love.
