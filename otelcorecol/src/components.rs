use component::Factory;
use exporter::loggingexporter;
use otelcol::{self, Factories};
use receiver::otlpreceiver;

pub(super) fn components() -> anyhow::Result<Factories> {
    let mut factories = Factories::default();

    factories.receivers = [otlpreceiver::new_factory()]
        .into_iter()
        .map(|factory| {
            (
                factory.r#type(),
                Box::new(factory) as Box<dyn receiver::Factory>,
            )
        })
        .collect();

    factories.exporters = [loggingexporter::new_factory()]
        .into_iter()
        .map(|factory| {
            (
                factory.r#type(),
                Box::new(factory) as Box<dyn exporter::Factory>,
            )
        })
        .collect();

    Ok(factories)
}
