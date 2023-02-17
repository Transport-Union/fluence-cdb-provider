use marine_rs_sdk::marine;


pub mod kubo;
pub mod curl;
pub mod helpers;

fn main() {

}

#[marine]
pub fn dag_put(stringified_object: String) -> String {
    kubo::dag_put(stringified_object) 
}

#[marine]
pub fn dag_get(cid: &String) -> String {
    kubo::dag_get(cid)
}