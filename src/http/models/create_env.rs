use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::Serialize;

#[derive(MyHttpInput)]
pub struct CreateEnvRequest {
    #[http_body(name = "envId", description = "Env id")]
    pub env_id: String,
    #[http_body(name = "envName", description = "Env name")]
    pub env_name: String,
}

#[derive(Serialize, Debug, MyHttpObjectStructure)]
pub struct CreateEnvResponse {
    pub servers: Vec<ApiServerModel>,
}

#[derive(Serialize, Debug, MyHttpObjectStructure)]
pub struct ApiServerModel {
    pub id: String,
    pub public_ip: Option<String>,
    pub private_ip: String,
}

#[derive(Serialize, Debug, MyHttpObjectStructure)]
pub struct ApiServerModelNetworks {
    pub network_id: i64,
    pub private_ip: String,
}
