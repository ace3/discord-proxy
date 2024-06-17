use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct WebhookPayload {
    pub webhook: String,
    pub thread: String,
    pub payload: serde_json::Value,
}
