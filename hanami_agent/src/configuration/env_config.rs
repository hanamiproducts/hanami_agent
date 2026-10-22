use crate::providers::ProviderKind;
use inquire::{Select, Text};

// Mock values until reading from the env file is written.

pub fn provider_kind() -> ProviderKind {
    ProviderKind::Ollama
}

/// Ollama runs locally, so no key is needed for the mock.
pub fn api_key() -> Option<String> {
    None
}

pub fn model() -> &'static str {
    "qwen3.8:latest"
}

fn is_env_at_current_path(name: &str) -> bool {
    matches!(name, "yes")
}

/// Same shape as the Manual branch in main: (kind, api_key, model).
pub fn load() -> (ProviderKind, Option<String>, &'static str) {
    let Ok(is_it_at_the_current_path) =
        Select::new("Is env file at the current path!", ["yes", "no"].to_vec()).prompt()
    else {
        todo!()
    };

    if (is_env_at_current_path(is_it_at_the_current_path)) {
    } else {
    }

    (provider_kind(), api_key(), model())
}
