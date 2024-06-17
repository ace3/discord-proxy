use axum::{routing::post, Json, Router};
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use std::net::SocketAddr;
mod models;
use models::WebhookPayload;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let app = Router::new().route("/webhook", post(webhook_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));
    println!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn webhook_handler(Json(payload): Json<WebhookPayload>) -> String {
    let client = Client::new();
    let webhook_url = format!(
        "https://discord.com/api/webhooks/{}{}",
        payload.webhook,
        if payload.thread.is_empty() {
            "".to_string()
        } else {
            format!("?thread_id={}", payload.thread)
        }
    );
    print!("Webhook URL: {}", webhook_url);

    let response = client
        .post(&webhook_url)
        .json(&payload.payload)
        .send()
        .await;

    match response {
        Ok(resp) => {
            println!("Response: {:?}", resp);
            if resp.status().is_success() {
                "Webhook sent successfully".to_string()
            } else {
                // return the error
                format!("Error sending webhook: {}", resp.text().await.unwrap()).to_string()
            }
        }
        Err(err) => {
            eprintln!("Error sending webhook: {}", err);
            "Failed to send webhook".to_string()
        }
    }
}
