use marine_rs_sdk::marine;
use tu_cdb_types::index::CdbResult;

pub fn main() {}

pub mod index;


#[marine]
pub fn m_query(ceramic_url: String, definition: String, query_s: String) -> CdbResult {
   a_query(ceramic_url, definition, query_s)
}

#[marine]
pub fn m_mutate(ceramic_url: String, definition: String, query: String, session: String) -> CdbResult {
    a_mutate(ceramic_url, definition, query, session)
}

#[marine]
pub fn deploy( ceramic_url: String, provider_cid: String, schema: String, owner: String) -> Vec<String> {
   
    let output = a_create_from_schema(ceramic_url, schema, owner);
    let index_cid = index::create_dag(output, &provider_cid);
    let new_provider_cid = index::add_index(provider_cid, &index_cid);
    vec!(new_provider_cid, index_cid)
}

#[marine]
#[link(wasm_import_module = "composedb_adapter")]
extern "C" {
    pub fn a_create_from_schema(ceramic_url: String, schema: String, owner: String) -> String;
    pub fn a_query(ceramic_url: String, definition: String, query_s: String) -> CdbResult;
    pub fn a_mutate(ceramic_url: String, definition: String, query: String, session: String) -> CdbResult;
}
