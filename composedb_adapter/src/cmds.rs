use crate::{
    marine,
    MountedBinaryResult,
    CdbResult,
    helpers
};


pub fn query(ceramic_url: String, definition: String, query_s: String) -> CdbResult {
    
    let cmd = vec![
        "query".to_owned(),
        "-c".to_owned(),
        ceramic_url.to_owned(),
        "-d".to_owned(),
        definition.to_owned(),
        "-q".to_owned(),
        query_s.to_owned()
    ];

    let res = tu_cdb(cmd);
    helpers::format_result(String::from_utf8(res.stdout).unwrap())
}

pub fn mutate(ceramic_url: String, definition: String, query: String, session: String) -> CdbResult {
    
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

    let res = tu_cdb(cmd);
    helpers::format_result(String::from_utf8(res.stdout).unwrap())
}

pub fn create_from_schema(ceramic_url: String, schema: String, owner: String) -> String  {
    
    let cmd = vec!(
        "createFromSchema".to_owned(),
        "-c".to_owned(),
        ceramic_url,
        "-s".to_owned(), 
        schema,
        "-o".to_owned(), 
        owner
    );
    let res = tu_cdb(cmd);

    String::from_utf8(res.stdout).unwrap()
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn tu_cdb(cmd: Vec<String>) -> MountedBinaryResult;
}