use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct NetAddr {
    pub endpoint: String,
    pub transport: String,
}
