use marine_rs_sdk::marine;

use tu_cdb_types::provider::ProviderDetails;
use tu_cdb_types::index::CdbIndex;
use tu_cdb_types::index::CdbResult;

pub fn main() {}

#[marine]
pub fn f_init(namespace: &String, n: &String) -> String {
    init(namespace, n)
}

#[marine]
pub fn f_provider_details(cid: &String) -> Vec<ProviderDetails> {
    provider_details(cid)
}

#[marine]
pub fn f_index_details(cid: &String) -> Vec<CdbIndex> {
    index_details(cid)
}

// deploy
#[marine]
pub fn f_deploy_index(provider_cid: String, schema: String, owner: String) -> Vec<String> {
    let ceramic_url = ceramic_url(&provider_cid);
    deploy_index(ceramic_url, provider_cid, schema, owner)
}

#[marine]
pub fn f_query(provider_cid: String, definition: String, query_s: String) -> CdbResult {
    let ceramic_url = ceramic_url(&provider_cid);
    query(ceramic_url, definition, query_s)
}

#[marine]
pub fn f_mutate(provider_cid: String, definition: String, query: String, session: String) -> CdbResult {
    let ceramic_url = ceramic_url(&provider_cid);
    mutate(ceramic_url, definition, query, session)
}


#[marine]
#[link(wasm_import_module = "config")]
extern "C" {
    pub fn init(namespace: &String, n: &String) -> String;
    pub fn provider_details(cid: &String) -> Vec<ProviderDetails>;
    pub fn index_details(cid: &String) -> Vec<CdbIndex>;
    pub fn ceramic_url(provider_cid: &String) -> String;
}

#[marine]
#[link(wasm_import_module = "index")]
extern "C" {
    pub fn deploy_index(ceramic_url: String, provider_cid: String, schema: String, owner: String) -> Vec<String>;
    pub fn query(ceramic_url: String, definition: String, query_s: String) -> CdbResult;
    pub fn mutate(ceramic_url: String, definition: String, query: String, session: String) -> CdbResult;

}