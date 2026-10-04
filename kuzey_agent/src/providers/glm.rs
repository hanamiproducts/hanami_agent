use super::{ChatMessage, ProviderConfig, Reply};
use crate::tools::{ToolCall, ToolSpec};
use serde_json::{Value, json};

pub(super) fn all_available_models() -> Vec<&'static str> {
    vec!["GLM-5.3", "GLM-5.2"]
}

pub(super) async fn send(
    config: &ProviderConfig,
    history: &[ChatMessage],
    tools: &[ToolSpec],
) -> Result<Reply, Box<dyn std::error::Error>> {
    Ok(Reply::Text(
        "Currently GLM integration is under development.".to_string(),
    ))
}
