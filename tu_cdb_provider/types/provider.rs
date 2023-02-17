use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};

#[marine]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ComposeDbPublicInfo {
    pub eth_address: String,
    pub public_encryption_key: String
}

#[marine]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ComposeDbDirections {
    pub ceramic_port: String,
    pub n: String,
    pub namespace: String
}

#[marine]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ComposeDBProviderConfig {
    pub directions: ComposeDbDirections,
    pub indexes: Vec<String>,
    pub public_info: Vec<ComposeDbPublicInfo>,
}

#[marine]
#[derive(Debug, Deserialize, Serialize)]
pub struct ProviderDetails {
    pub composedb : ComposeDBProviderConfig
}