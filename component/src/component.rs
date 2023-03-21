use crate::{Config, Type};

/// Factory is implemented by all Component factories.
pub trait Factory {
    fn r#type(&self) -> Type;

    fn create_default_config(&self) -> Config;
}
