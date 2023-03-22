use crate::otlpreceiver::{Config, Protocols};
use crate::receiver;
use ::config::{configgrpc::GrpcServerSettings, confignet};

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
    component::Config(Box::new(Config {
        protocols: Protocols {
            grpc: Some(GrpcServerSettings {
                net_addr: confignet::NetAddr {
                    endpoint: "xxx".into(),
                    transport: "tcp".into(),
                },
            }),
        },
    }))
}
