/// Factory is factory trait for receivers.
pub trait Factory: component::Factory {}

pub fn new_factory<F>(cfg_type: component::Type, create_default_config: F) -> impl Factory
where
    F: Fn() -> component::Config,
{
    let f = FactoryImpl {
        cfg_type,
        create_default_config,
    };

    f
}

struct FactoryImpl<F> {
    cfg_type: component::Type,
    create_default_config: F,
}

impl<F> component::Factory for FactoryImpl<F>
where
    F: Fn() -> component::Config,
{
    fn r#type(&self) -> component::Type {
        self.cfg_type
    }

    fn create_default_config(&self) -> component::Config {
        self.create_default_config()
    }
}

impl<F> Factory for FactoryImpl<F> where F: Fn() -> component::Config {}
