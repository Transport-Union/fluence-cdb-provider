use marine_rs_sdk::marine;
use tu_cdb_types::index::CdbResult;

pub fn main() {}

pub mod index;
pub mod helpers;
pub mod graphql;

#[marine]
pub fn query(ceramic_url: String, definition: String, query_s: String) -> CdbResult {
   graphql::graphql_query(ceramic_url, definition, query_s)
}

#[marine]
pub fn mutate(ceramic_url: String, definition: String, query: String, session: String) -> CdbResult {
    graphql::graphql_mutate(ceramic_url, definition, query, session)
}

#[marine]
pub fn deploy( ceramic_url: String, provider_cid: String, schema: String, owner: String) -> Vec<String> {
    index::deploy(ceramic_url, provider_cid, schema, owner)
}