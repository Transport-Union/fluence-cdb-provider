use marine_rs_sdk::marine;
use serde::{Deserialize };

#[marine]
#[derive(Debug, Deserialize)]
pub struct CdbResult {
    pub count: u64,
    pub content: String,
    pub error: String,
    pub success: bool
}

#[marine]
#[derive(Debug, Deserialize)]
pub struct CdbIndex {
    pub composite_definition: String,
    pub description: String,
    pub id: String,
    pub models: String,
    pub name: String,
    pub owner: String,
    pub provider: Vec<String>,
    pub runtime_definition: String
}