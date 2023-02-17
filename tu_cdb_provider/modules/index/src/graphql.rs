use crate::marine;
use crate::CdbResult;
use crate::helpers;

pub fn graphql_query(ceramic_url: String, definition: String, query_s: String) -> CdbResult {

    let cmd = vec![
        "query".to_owned(),
        "-c".to_owned(),
        ceramic_url.to_owned(),
        "-d".to_owned(),
        definition.to_owned(),
        "-q".to_owned(),
        query_s.to_owned()
    ];

    helpers::format_result(query(cmd))
}


pub fn graphql_mutate(ceramic_url: String, definition: String, query: String, session: String) -> CdbResult {

    let cmd = vec![
        "mutate".to_owned(),
        "-c".to_owned(),
        ceramic_url.to_owned(),
        "-d".to_owned(),
        definition.to_owned(),
        "-q".to_owned(),
        query.to_owned(),
        "-s".to_owned(),
        session.to_owned(),
    ];

    helpers::format_result(mutate(cmd))
}

#[marine]
#[link(wasm_import_module = "composedb_adapter")]
extern "C" {
    pub fn query(cmd: Vec<String>) -> String;
    pub fn mutate(cmd: Vec<String>) -> String;
}