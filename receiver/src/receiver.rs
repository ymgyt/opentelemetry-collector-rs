/// Factory is factory trait for receivers.
pub trait Factory: component::Factory {}

pub fn new_factory(
    cfg_type: component::Type,
    create_default_config: Box<dyn Fn() -> component::Config>,
    options: &[&dyn FactoryOption],
) -> impl Factory {
    let mut f = FactoryImpl {
        cfg_type,
        create_default_config,

        traces_stability_level: Default::default(),
        metrics_stability_level: Default::default(),
        logs_stability_level: Default::default(),
    };

    options.into_iter().for_each(|o| o.apply_option(&mut f));

    f
}

pub fn with_traces(sl: component::StabilityLevel) -> impl FactoryOption {
    move |o: &mut FactoryImpl| {
        o.traces_stability_level = sl;
    }
}

pub fn with_metrics(sl: component::StabilityLevel) -> impl FactoryOption {
    move |o: &mut FactoryImpl| {
        o.metrics_stability_level = sl;
    }
}

pub fn with_logs(sl: component::StabilityLevel) -> impl FactoryOption {
    move |o: &mut FactoryImpl| {
        o.logs_stability_level = sl;
    }
}

pub trait FactoryOption {
    fn apply_option(&self, o: &mut FactoryImpl);
}

impl<F> FactoryOption for F
where
    F: Fn(&mut FactoryImpl),
{
    fn apply_option(&self, o: &mut FactoryImpl) {
        self(o);
    }
}

pub struct FactoryImpl {
    cfg_type: component::Type,
    create_default_config: Box<dyn Fn() -> component::Config>,

    traces_stability_level: component::StabilityLevel,
    metrics_stability_level: component::StabilityLevel,
    logs_stability_level: component::StabilityLevel,
}

impl component::Factory for FactoryImpl {
    fn r#type(&self) -> component::Type {
        self.cfg_type
    }

    fn create_default_config(&self) -> component::Config {
        (self.create_default_config)()
    }
}

impl Factory for FactoryImpl {}
