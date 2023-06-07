use std::sync::Arc;

use hetzner_bridge::{AppContext, SettingsReader, CreateEnvAction};
use service_sdk::ServiceContext;

#[tokio::main]
async fn main() {
    let settings_reader = Arc::new(SettingsReader::new(".my-cfd").await);
    let settings = Arc::new(settings_reader.get_settings().await);

    let mut service_context = ServiceContext::new(settings_reader);
    let app_ctx = Arc::new(AppContext::new(settings));
    service_context
        .setup_http(None, None)
        .register_http_routes(|server| {
            server.register_post(CreateEnvAction::new(app_ctx.clone()));
        
        });

    service_context.start_application().await;
}
