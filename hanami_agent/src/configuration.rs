pub mod env_config;
pub mod manual_config;

use crate::providers::ProviderKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfigurationStyle {
    Env,
    Manual,
}

impl ConfigurationStyle {
    pub const ALL: [ConfigurationStyle; 2] = [Self::Env, Self::Manual];
}

impl std::fmt::Display for ConfigurationStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Env => "Env",
            Self::Manual => "Manual",
        };
        write!(f, "{name}")
    }
}

/// Returns (kind, api_key, model), or None if the user cancelled.
pub fn configure(
    style: ConfigurationStyle,
) -> Option<(ProviderKind, Option<String>, &'static str)> {
    match style {
        ConfigurationStyle::Env => Some(env_config::load()),
        ConfigurationStyle::Manual => manual_config::load(),
    }
}
