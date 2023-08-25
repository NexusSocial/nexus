use common::{Did, Homeserver};
use anyhow::Result;
use reqwest::Client;

pub async fn add_user_to_server(did: Did, homeserver: Homeserver) -> Result<()> {
    Client::new()
        .get("http://".to_string() + &homeserver.0 + "/add-user")
        .body(serde_json::to_vec(&common::non_standard::AddUser {
            did,
        }).unwrap())
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

pub async fn add_user_homeserver_mapping(did: Did, homeserver: Homeserver, to_map: Homeserver) -> Result<()> {
    Client::new()
        .get("http://".to_string() + &homeserver.0 + "/add-user-homeserver-mapping")
        .body(serde_json::to_vec(&common::non_standard::AddHomeServerMapping {
            did,
            homeserver: to_map,
        }).unwrap())
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}