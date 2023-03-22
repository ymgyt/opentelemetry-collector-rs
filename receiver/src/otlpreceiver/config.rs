use config::configgrpc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Protocols {
    pub grpc: Option<configgrpc::GrpcServerSettings>,
}
/// Config defines configuration for OTLP receiver.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub protocols: Protocols,
}
