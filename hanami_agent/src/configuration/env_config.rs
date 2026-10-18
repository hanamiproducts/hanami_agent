use crate::providers::ProviderKind;

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

/// Same shape as the Manual branch in main: (kind, api_key, model).
pub fn load() -> (ProviderKind, Option<String>, &'static str) {
    (provider_kind(), api_key(), model())
}
