use providers::{ChatMessage, ProviderConfig, Reply};
use serde_json::{Value, json};
use std::time::Duration;
use tools::{ToolCall, ToolSpec};

pub struct AgentDef {
    pub name: &'name str,
    pub description: &'static str,
    pub system_prompt: &'static str,
    pub tools: &'static [&'static str],
    pub max_rounds: usize,
}

pub async fn run(
    config: &ProviderConfig,
    history: &mut Vec<ChatMessage>,
    tools: &[ToolSpec],
    label: &str,
    max_rounds: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    for _ in 0..max_rounds {
        match providers::send(config, history, tools).await? {
            Reply::Text(text) => {
                history.push(ChatMessage::Assisstant(text.clone()));
                return Ok(text);
            }
            Reply::ToolCalls { calls, reasoning } => {
                history.push(ChatMessage::ToolCalls(calls.clone(), reasoning.clone()));
                return Ok(calls);
            }
        }
    }
    Err("Ran out of tool rounds".into())
}
