// use crate::types::ComposeDbIndex;
use crate::marine;

use crate::ComposeDbDirections;
use crate::ComposeDBProviderConfig;
use crate::ProviderDetails;
use crate::CdbIndex;


pub fn init(namespace: &String, n: &String) -> String {

    let ceramic_port : &str = "7007";

    let public_info = vec![]; 
    let indexes = vec![];

    let directions = ComposeDbDirections {
        namespace: namespace.to_owned(),
        n: n.to_owned(),
        ceramic_port: ceramic_port.to_owned()
    };

    let composedb = ComposeDBProviderConfig {
        directions,
        indexes,
        public_info
    };

    let obj = ProviderDetails {
        composedb: composedb
    };

    let v: serde_json::Value = serde_json::to_value(obj).unwrap();

    dag_put(serde_json::to_string(&v).unwrap())

}

pub fn index_details(cid: &String) -> Vec<CdbIndex> {

    let mut results: Vec<CdbIndex> = vec!();
    let s = dag_get(cid);
    if s != "timed_out" {
        results.push(serde_json::from_str(&s).unwrap());
    } 
    results
}

pub fn provider_details(cid: &String) -> Vec<ProviderDetails> {

    let mut results: Vec<ProviderDetails> = vec!();
    let s = dag_get(cid);
    if s != "timed_out" {
        results.push(serde_json::from_str(&s).unwrap());
    } 
    results
}

#[marine]
#[link(wasm_import_module = "ipfs_adapter")]
extern "C" {
    pub fn dag_get(cid: &String) -> String;
    pub fn dag_put(stringified_object: String) -> String;
}