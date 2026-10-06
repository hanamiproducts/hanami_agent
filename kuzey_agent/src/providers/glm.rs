use super::{ChatMessage, ProviderConfig, Reply};
use crate::tools::{ToolCall, ToolSpec};
use serde_json::{Value, json};

// Model ids are lowercase in the API.
pub(super) fn all_available_models() -> Vec<&'static str> {
    vec!["glm-5.3", "glm-5.2"]
}

pub(super) async fn send(
    config: &ProviderConfig,
    history: &[ChatMessage],
    tools: &[ToolSpec],
) -> Result<Reply, Box<dyn std::error::Error>> {
    let api_key = config.api_key.as_deref().ok_or("GLM needs an API key!")?;

    // GLM uses the Chat Completions format (like Deepseek), so the system prompt is the first message.
    let mut messages: Vec<Value> = vec![json!({
        "role": "system",
        "content": config.system_prompt,
    })];

    for entry in history {
        match entry {
            ChatMessage::User(text) => messages.push(json!({
                "role": "user",
                "content": text,
            })),
            ChatMessage::Assistant(text) => messages.push(json!({
                "role": "assistant",
                "content": text,
            })),
            // All calls of one turn go into a single assistant message.
            ChatMessage::ToolCalls { calls, reasoning } => {
                let tool_calls: Vec<Value> = calls
                    .iter()
                    .map(|c| {
                        json!({
                            "id": c.id,
                            "type": "function",
                            "function": {
                                "name": c.name,
                                "arguments": c.args.to_string(),
                            }
                        })
                    })
                    .collect();

                let mut msg = json!({
                    "role": "assistant",
                    "content": null,
                    "tool_calls": tool_calls,
                });
                // Thinking is on by default (forced on glm-5.3); the docs say the reasoning
                // must be sent back unmodified.
                if let Some(reasoning) = reasoning {
                    msg["reasoning_content"] = json!(reasoning);
                }
                messages.push(msg);
            }
            ChatMessage::ToolResult { id, output, .. } => messages.push(json!({
                "role": "tool",
                "tool_call_id": id,
                "content": output,
            })),
        }
    }

    let tool_list: Vec<Value> = tools
        .iter()
        .map(|t| {
            json!({
                "type": "function",
                "function": {
                    "name": t.name,
                    "description": t.description,
                    "parameters": t.parameters,
                }
            })
        })
        .collect();

    let body = json!({
        "model": config.model,
        "messages": messages,
        "tools": tool_list,
        "temperature": config.temp,
        "top_p": config.top_p,
    });

    let client = reqwest::Client::new();

    // Coding plan keys use https://api.z.ai/api/coding/paas/v4/chat/completions instead.
    let response: Value = client
        .post("https://api.z.ai/api/paas/v4/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    if let Some(msg) = response["error"]["message"].as_str() {
        return Err(format!("GLM error: {msg}").into());
    }

    let message = &response["choices"][0]["message"];
    if message.is_null() {
        return Err(format!("GLM returned no choices: {response}").into());
    }

    let answer = message["content"].as_str().unwrap_or("").to_string();
    let reasoning = message["reasoning_content"].as_str().map(|s| s.to_string());

    let mut calls: Vec<ToolCall> = Vec::new();
    if let Some(items) = message["tool_calls"].as_array() {
        for item in items {
            calls.push(ToolCall {
                id: item["id"].as_str().map(|s| s.to_string()),
                name: item["function"]["name"].as_str().unwrap_or("").to_string(),
                // arguments comes as a JSON string, not an object.
                args: serde_json::from_str(item["function"]["arguments"].as_str().unwrap_or("{}"))
                    .unwrap_or(json!({})),
                signature: None,
            });
        }
    }

    if calls.is_empty() {
        Ok(Reply::Text(answer))
    } else {
        Ok(Reply::ToolCalls { calls, reasoning })
    }
}
