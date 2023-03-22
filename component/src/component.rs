use crate::{Config, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StabilityLevel(isize);

impl StabilityLevel {
    pub const UNDEFINED: Self = StabilityLevel(0);
    pub const DEVELOPMENT: Self = StabilityLevel(3);
    pub const BETA: Self = StabilityLevel(5);
    pub const STABLE: Self = StabilityLevel(6);
}

impl Default for StabilityLevel {
    fn default() -> Self {
        StabilityLevel::UNDEFINED
    }
}

/// Factory is implemented by all Component factories.
pub trait Factory {
    fn r#type(&self) -> Type;

    fn create_default_config(&self) -> Config;
}
