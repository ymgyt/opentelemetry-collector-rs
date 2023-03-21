use otelcol::{self, Factories};
use receiver::otlpreceiver;

pub(super) fn components() -> anyhow::Result<Factories> {
    let mut factories = Factories::default();

    factories.receivers = [otlpreceiver::new_factory()].into_iter().map(|factory| (factory.type(), factory))
    .collect();

    todo!()
}
