use std::any::Any;

pub struct Config(pub Box<dyn Any>);

/// Type is the component type as it is used in the config.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Type(pub &'static str);
