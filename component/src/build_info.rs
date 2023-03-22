#[derive(Debug)]
pub struct BuildInfo {
    pub command: String,
    pub description: String,
    pub version: String,
}

impl Default for BuildInfo {
    fn default() -> Self {
        BuildInfo {
            command: "otelcol".into(),
            description: "OpenTelemetry Collector".into(),
            version: "latest".into(),
        }
    }
}
