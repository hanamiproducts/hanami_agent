pub mod env_config;

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

impl ConfigurationStyle {
    pub fn is_it_special(&self) -> bool {
        matches!(self, Self::Env)
    }
}
