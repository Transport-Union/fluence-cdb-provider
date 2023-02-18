use marine_rs_sdk::marine;

use tu_cdb_types::provider::ProviderDetails;
use tu_cdb_types::index::CdbIndex;
use tu_cdb_types::index::CdbResult;

pub fn main() {}

#[marine]
pub fn init(namespace: &String, n: &String) -> String {
    m_init(namespace, n)
}

#[marine]
pub fn provider_details(cid: &String) -> Vec<ProviderDetails> {
    m_provider_details(cid)
}

#[marine]
pub fn index_details(cid: &String) -> Vec<CdbIndex> {
    m_index_details(cid)
}

// deploy
#[marine]
pub fn deploy_index(provider_cid: String, schema: String, owner: String) -> Vec<String> {
    let ceramic_url = m_ceramic_url(&provider_cid);
    m_deploy_index(ceramic_url, provider_cid, schema, owner)
}

#[marine]
pub fn query(provider_cid: String, definition: String, query_s: String) -> CdbResult {
    let ceramic_url = m_ceramic_url(&provider_cid);
    m_query(ceramic_url, definition, query_s)
}

#[marine]
pub fn mutate(provider_cid: String, definition: String, query: String, session: String) -> CdbResult {
    let ceramic_url = m_ceramic_url(&provider_cid);
    m_mutate(ceramic_url, definition, query, session)
}


#[marine]
#[link(wasm_import_module = "config")]
extern "C" {
    pub fn m_init(namespace: &String, n: &String) -> String;
    pub fn m_provider_details(cid: &String) -> Vec<ProviderDetails>;
    pub fn m_index_details(cid: &String) -> Vec<CdbIndex>;
    pub fn m_ceramic_url(provider_cid: &String) -> String;
}

#[marine]
#[link(wasm_import_module = "index")]
extern "C" {
    pub fn m_deploy_index(ceramic_url: String, provider_cid: String, schema: String, owner: String) -> Vec<String>;
    pub fn m_query(ceramic_url: String, definition: String, query_s: String) -> CdbResult;
    pub fn m_mutate(ceramic_url: String, definition: String, query: String, session: String) -> CdbResult;

}