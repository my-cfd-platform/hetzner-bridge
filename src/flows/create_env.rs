use std::{collections::HashMap, sync::Arc, time::Duration};

use convert_case::Casing;
use hetzner_client::{
    CreateNetworkResponse, CreateServerResponse, HetznerClientFirewallRules,
    HetznerClientPublicNetType, HetznerClientSubnetModel, HetznerCreateFirewallResponse,
};
use tokio::time::sleep;

use crate::AppContext;

pub async fn create_network(
    app: Arc<AppContext>,
    env_id: &String,
    env_name: &String,
) -> CreateNetworkResponse {
    let labels = HashMap::from_iter(vec![
        ("env_id".to_string(), env_id.clone()),
        ("env_name".to_string(), env_name.clone()),
    ]);

    let name = format!("{}-network-{}", env_name, env_id).to_case(convert_case::Case::UpperCamel);

    app.hetzner_client
        .create_network(
            "10.100.0.0/24".to_string(),
            labels,
            name,
            vec![],
            vec![HetznerClientSubnetModel {
                ip_range: "10.100.0.0/28".to_string(),
                network_zone: "eu-central".to_string(),
            }],
        )
        .await
}

pub async fn create_firewall(
    app: Arc<AppContext>,
    env_id: &String,
    env_name: &String,
) -> HetznerCreateFirewallResponse {
    let labels = HashMap::from_iter(vec![
        ("env_id".to_string(), env_id.clone()),
        ("env_name".to_string(), env_name.clone()),
    ]);

    let name = format!("{}-firewall-{}", env_name, env_id).to_case(convert_case::Case::UpperCamel);

    app.hetzner_client
        .create_firewall(
            vec![],
            labels,
            name,
            vec![HetznerClientFirewallRules {
                destination_ips: None,
                direction: hetzner_client::CreateFirewallRequestRulesDirectionType::In,
                source_ips: Some(vec!["0.0.0.0/0".to_string()]),
                port: None,
                protocol: hetzner_client::CreateFirewallRequestRulesProtocolType::Icmp,
                description: None,
            }],
        )
        .await
}

pub async fn create_vm(
    app: Arc<AppContext>,
    env_id: &String,
    env_name: &String,
    firewall_id: i64,
    network_id: i64,
) -> CreateServerResponse {
    let labels = HashMap::from_iter(vec![
        ("env_id".to_string(), env_id.clone()),
        ("env_name".to_string(), env_name.clone()),
    ]);

    let name = format!("{}-server-{}", env_name, env_id).to_case(convert_case::Case::UpperCamel);

    app.hetzner_client
        .create_vm(
            vec![firewall_id],
            "ubuntu-22.04".to_string(),
            Some("nbg1".to_string()),
            None,
            vec![network_id, app.settings.main_network_id],
            labels,
            name,
            HetznerClientPublicNetType::IPv4(None),
            app.settings.default_server_type.clone(),
            vec![],
            vec![],
        )
        .await
}

pub async fn create_env(
    app: Arc<AppContext>,
    env_id: String,
    env_name: String,
) -> (
    CreateServerResponse,
    CreateNetworkResponse,
    HetznerCreateFirewallResponse,
    String,
) {
    let firewall_result = create_firewall(app.clone(), &env_id, &env_name).await;
    let network_result = create_network(app.clone(), &env_id, &env_name).await;
    let server_result = create_vm(
        app.clone(),
        &env_id,
        &env_name,
        firewall_result.firewall.id,
        network_result.network.id,
    )
    .await;

    let mut ip: Option<String> = None;

    loop {
        sleep(Duration::from_secs(2)).await;
        let server_info_response = app
            .hetzner_client
            .get_vm_info(server_result.server.id)
            .await;

        let search_result = server_info_response
            .server
            .private_net
            .into_iter()
            .find(|x| x.network == app.settings.main_network_id);

        match search_result {
            Some(src) => {
                ip = Some(src.ip);
                return (server_result, network_result, firewall_result, ip.unwrap());
            }
            None => continue,
        }
    }
}
