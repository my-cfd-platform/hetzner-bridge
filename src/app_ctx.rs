use std::sync::Arc;

use hetzner_client::ApiClient;

use crate::SettingsModel;

pub struct AppContext {
    pub settings: Arc<SettingsModel>,
    pub hetzner_client: ApiClient,
}

impl AppContext {
    pub fn new(settings: Arc<SettingsModel>) -> AppContext {
        let hetzner_client = ApiClient::new(settings.hetzner_api_key.clone());
        AppContext {
            settings,
            hetzner_client,
        }
    }
}
