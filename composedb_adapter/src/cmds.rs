/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

 // #![allow(improper_ctypes)]

 use marine_rs_sdk::marine;
 // use marine_rs_sdk::WasmLoggerBuilder;

 fn main() {}
 
#[marine] 
 fn query() -> String {
    // let cmd = vec!(
    //     "query".to_owned()
    // );
    
    // let res = tu_cdb(cmd);
    // String::from_utf8(res.stdout).unwrap()
    String::from("kip")
 }

//  'add or update records',
#[marine]
fn mutate() -> String {
    // let cmd = vec!(
    //     "mutate".to_owned()
    // );
    
    // let res = tu_cdb(cmd);
    // String::from_utf8(res.stdout).unwrap()

    String::from("kip")
}

// creates a runtime defintion from a graphql schema'
// ceramic_url: String, schema: String, owner: String
#[marine]
fn create_from_schema() -> String  {
    
    // let cmd = vec!(
    //     "createFromSchema".to_owned(),
    //     "-c".to_owned(),
    //     ceramic_url,
    //     "-s".to_owned(), 
    //     schema,
    //     "-o".to_owned(), 
    //     owner
    // );
    // let res = tu_cdb(cmd);

    // String::from_utf8(res.stdout).unwrap()

    String::from("kip")
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn tu_cdb(cmd: Vec<String>) -> MountedBinaryResult;
}