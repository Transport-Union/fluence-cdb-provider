use crate::ProviderDetails;

pub fn ceramic_url(cid: &String) -> String {

    let url: String;

    if cid == "localhost" {
        
        url = String::from("http://0.0.0.0:7007");
    
    } else {
        
        let details : Vec<ProviderDetails> = crate::m_provider_details(cid);
        let d = &details[0].composedb.directions;
        url = format!("http://{}{}_ceramic:{}", d.namespace, d.n, d.ceramic_port);
    }

    url
} 