use crate::confignet;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct GrpcServerSettings {
    #[serde(flatten)]
    pub net_addr: confignet::NetAddr,
}
