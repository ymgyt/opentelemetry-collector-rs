use std::collections::HashMap;

/// Factories structt holds in a single type all component factories that
/// can be handled by Config.
#[derive(Default)]
pub struct Factories {
    pub receivers: HashMap<component::Type, Box<dyn receiver::Factory>>,
}
