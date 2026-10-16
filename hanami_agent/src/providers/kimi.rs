use super::{ChatMessage, ProviderConfig, Reply};
use crate::tools::{ToolCall, ToolSpec};
use serde_json::{Value, json};
use std::time::Duration;

pub(super) fn all_available_models() -> Vec(&'static str) {
    vec!["kimi-k3"]
}

pub(super) async fn send(
    config: &ProviderConfig,
    history: &[ChatMessage],
    tools: &[ToolSpec],
) -> Result<Reply, Box<dyn std::error::Error>> {
    let api_key = config.api_key.as_deref().ok_or("kimi needs an api_key!")?;

    let mut input: Vec<Value> = Vec::new();

    for entry in history {
        match entry {
            ChatMessage::User(text) => input.push(json!({
                "role":"user",
                "content": text,
            })),
            ChatMessage::Assistant(text) => input.push(json!({
                "role":"assistant",
                "content": text,
            })),
            ChatMessage::ToolCalls { calls, .. } => {
                for c in calls {
                    input.push(json!({
                        "type": "function_call",
                        "call_id": c.id,
                        "name": c.name,
                        "arguments": c.args.to_string(),
                    }))
                }
            }
            ChatMessage::ToolResult { id, output, .. } => input.push(json!({})),
        }
    }

    Ok(Reply::Text(String::from("Currently under construction.")))
}
