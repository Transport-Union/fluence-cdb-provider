use crate::marine;


pub fn create_dag(output: String, provider_cid: &String) -> String {

    let mut index: serde_json::Value = serde_json::from_str(&output).unwrap();
    // generic on ipfs .. ipld?
    let mut vec = index["provider"].as_array().unwrap().clone();
    vec.push(serde_json::Value::String(provider_cid.to_string()));
    index["provider"] = serde_json::Value::Array(vec.to_vec());

    dag_put(index.to_string())
} 

pub fn add_index(provider_cid: String, cid: &String) -> String {

    let s = dag_get(&provider_cid);

    let mut c: serde_json::Value = serde_json::from_str(&s).unwrap();
    let mut vec = c["composedb"]["indexes"].as_array().unwrap().clone();
    vec.push(serde_json::Value::String(cid.to_string()));
    c["composedb"]["indexes"] = serde_json::Value::Array(vec.to_vec());

    // ipld links ??? 
    // IpfsLink {x_
    //     Name,
    //     Tsize: stats["Size"].to_string().parse::<i32>().unwrap(),
    //     Hash: CidObject { Cid: cid }
    //   
    dag_put(c.to_string())

}

#[marine]
#[link(wasm_import_module = "ipfs_adapter")]
extern "C" {
    pub fn dag_get(cid: &String) -> String;
    pub fn dag_put(stringified_object: String) -> String;
}