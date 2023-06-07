use my_settings_reader::SettingsModel;
use serde::{Deserialize, Serialize};
use service_sdk::ServiceInfo;

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub bastion_vm_id: i64,
    pub default_vm_image: String,
    pub default_server_type: String,
    pub hetzner_api_key: String,
    pub telemetry: String,
    pub main_network_id: i64,
}

#[async_trait::async_trait]
impl my_telemetry_writer::MyTelemetrySettings for SettingsReader {
    async fn get_telemetry_url(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.telemetry.clone()
    }
}

#[async_trait::async_trait]
impl ServiceInfo for SettingsReader {
    fn get_service_name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }
    fn get_service_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}
