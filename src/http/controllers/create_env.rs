use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::{create_env, ApiServerModel, AppContext, CreateEnvRequest, CreateEnvResponse};

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/bridge/v1/createEnv",
    summary: "Create env",
    description: "Create env",
    controller: "EnvController",
    input_data: "CreateEnvRequest",
    result:[
        {status_code: 200, description: "Ok response", model: "CreateEnvResponse"},
    ]
)]
pub struct CreateEnvAction {
    app: Arc<AppContext>,
}

impl CreateEnvAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &CreateEnvAction,
    input_data: CreateEnvRequest,
    _: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let (server, _, _, ip) =
        create_env(action.app.clone(), input_data.env_id, input_data.env_name).await;

    let response = CreateEnvResponse{
        servers: vec![ApiServerModel {
            id: server.server.id.to_string(),
            public_ip: None,
            private_ip: ip,
        }],
    };

    return HttpOutput::as_json(response).into_ok_result(true).into();
}
