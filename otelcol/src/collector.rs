use crate::Factories;

#[derive(Default)]
pub struct CollectorSettings {
    pub factories: Factories,
    pub build_info: component::BuildInfo,
    pub disable_graceful_shutdown: bool,
    // pub config_provider: ConfigProvider,
    pub skip_setting_grpc_logger: bool,
}
