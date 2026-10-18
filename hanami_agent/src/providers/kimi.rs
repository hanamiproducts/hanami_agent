use super::{ChatMessage, ProviderConfig, Reply};
use serde_json::{Value,json};
use crate::tools::ToolSpec;

pub(super) fn all_available_models() -> Vec<&'static str> {
    vec!["kimi-k3"]
}

// TODO: Kimi uses Chat Completions (https://api.moonshot.ai/v1/chat/completions), same as
// deepseek.rs. Write the request here.
pub(super) async fn send(
    config: &ProviderConfig,
    history: &[ChatMessage],
    tools: &[ToolSpec],
) -> Result<Reply, Box<dyn std::error::Error>> {
    let _api_key = config.api_key.as_deref().ok_or("Kimi needs an API key!")?;

    let mut messages: Vec<Value> = vec![json!({
        "role": "system",
        "content": config.system_prompt,
    })];

    for entry in history {
        match entry {
            ChatMessage::User(text) => {
                messages.push(json!({
                    "role": "user",
                    "content": text,
                }))
            },
            ChatMessage::Assistant(text) => {
                messages.push(json!({
                    "role": "assistant",
                    "content": text,
                }))
            },
            ChatMessage::ToolCalls {calls, reasoning} => {
                let tool_calls: Vec<Value> = calls.iter().map(|c| {
                    json!({
                        "id": c.id,
                        "type": "function",
                        "function": {
                            "name": c.name,
                            "arguments": c.args.to_string(),
                        }
                    })
                }).collect();

                let mut msg = json!({
                    "role": "assistant",
                    "content": null,
                    "tool_calls": tool_calls,
                });

                if let Some(reasoning) = reasoning {
                    msg["reasoning_content"] = json!(reasoning)
                }
                messages.push(msg);
            },
            ChatMessage::ToolResult {id, output, ..} => messages.push(json!({
                "role":"tool",
                "tool_call_id": id,
                "content": output,
            })),
        }
    }

    let tool_list: Vec<Value> = tools.iter().map(|t| {
        json!({
            "type": "function",
            "function": {
                "name": t.name,
                "description": t.description,
                "parameters": t.parameters,
            }
        })
    }).collect()

    let body = json!({
        "model": config.model,
        "messages": messages,
        "tools": tool_list,
        "temperature":config.temp,
        "top_p": config.top_p,
    });

    let client = reqwest::Client::new();

    //let response: Value = client.post("")


    Err("Kimi is not implemented yet.".into())
}
