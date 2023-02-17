// use std::error::Error;
use crate::curl;
use crate:: helpers;

pub fn dag_put(stringified_object: String) -> String {

    let url = format!("http://127.0.0.1:5001/api/v0/dag/put?pin=true");

    let v: serde_json::Value = serde_json::from_str(&stringified_object).unwrap();

    let data_string = format!("file={}", v);
  
    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("-F"),
        data_string,
        url
    ];
  
    let response = curl::curl_request(curl_args);
  
    helpers::extract_cid(&String::from_utf8(response.stdout).unwrap())
}

pub fn dag_get(cid: &String) -> String {

    let url = format!("http://127.0.0.1:5001/api/v0/dag/get?arg={}", cid);
  
    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("--max-time"),
        String::from("15"),
        url
    ];
  
    let response = curl::curl_request(curl_args);
  
    if response.stdout.is_empty() {
        "timed_out".into()
    } else {       
        String::from_utf8(response.stdout).unwrap()
    }
}