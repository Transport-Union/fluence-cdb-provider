use crate::CdbResult;

pub fn format_result(output: String) -> CdbResult {

    let v : serde_json::Value = serde_json::from_str(&output).unwrap();

    let count = v["count"].as_u64().unwrap();
    let content = v["content"].to_string();
    let error = v["error"].to_string();
    let success = v["success"].as_bool().unwrap();

    CdbResult {
        count,
        content,
        error,
        success
    }
}