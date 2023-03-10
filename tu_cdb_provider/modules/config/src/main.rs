use marine_rs_sdk::marine;
use tu_cdb_types::provider::ProviderDetails;
use tu_cdb_types::provider::ComposeDbDirections;
use tu_cdb_types::provider::ComposeDBProviderConfig;
use tu_cdb_types::index::CdbIndex;

pub mod config;
pub mod helpers;

pub fn main() {}

#[marine]
pub fn m_init(namespace: &String, n: &String) -> String {
    config::init(namespace, n)
}

#[marine]
pub fn m_index_details(cid: &String) -> Vec<CdbIndex> {
    config::index_details(cid)
}

#[marine]
pub fn m_provider_details(cid: &String) -> Vec<ProviderDetails> {
    config::provider_details(cid)
}

#[marine]
pub fn m_ceramic_url(cid: &String) -> String {
    helpers::ceramic_url(cid)
} 