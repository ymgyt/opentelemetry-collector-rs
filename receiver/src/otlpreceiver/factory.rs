use crate::receiver;

const TYPE_STR: component::Type = component::Type("otlp");

pub fn new_factory() -> impl receiver::Factory {
    receiver::new_factory(
        TYPE_STR,
        Box::new(create_default_config),
        &[
            &receiver::with_traces(component::StabilityLevel::STABLE),
            &receiver::with_metrics(component::StabilityLevel::STABLE),
            &receiver::with_logs(component::StabilityLevel::BETA),
        ],
    )
}

fn create_default_config() -> component::Config {
    todo!()
}
