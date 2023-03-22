use crate::exporter;

const TYPE: component::Type = component::Type("logging");

pub fn new_factory() -> impl exporter::Factory {
    exporter::new_factory(
        TYPE,
        Box::new(create_default_config),
        &[
            &exporter::with_traces(component::StabilityLevel::DEVELOPMENT),
            &exporter::with_metrics(component::StabilityLevel::DEVELOPMENT),
            &exporter::with_logs(component::StabilityLevel::DEVELOPMENT),
        ],
    )
}

fn create_default_config() -> component::Config {
    todo!()
}
