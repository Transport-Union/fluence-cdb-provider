

pub fn extract_cid (response: &String) -> String {
    let v : serde_json::Value = serde_json::from_str(response).unwrap();
    v["Cid"]["/"].as_str().unwrap().to_string()
}
