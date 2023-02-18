 // #![allow(improper_ctypes)]
use marine_rs_sdk::{
    marine,
    MountedBinaryResult
};

pub mod cmds;
pub mod helpers;
pub mod types;

use types::CdbResult;

fn main() {}
 
#[marine] 
pub fn a_query(ceramic_url: String, definition: String, query_s: String) -> CdbResult {
    cmds::query(ceramic_url, definition, query_s)
}

#[marine]
pub fn a_mutate(ceramic_url: String, definition: String, query: String, session: String) -> CdbResult {
    cmds::mutate(ceramic_url, definition, query, session)
}

#[marine]
pub fn a_create_from_schema(ceramic_url: String, schema: String, owner: String) -> String  {
    cmds::create_from_schema(ceramic_url, schema, owner)
}