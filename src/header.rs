use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct Headers{
    pub headers:Vec<Header>,
}
#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct Header{
    pub  header:String,
    pub value:String
}
// rustc header.rs && ./header

impl Headers{
    pub fn new(split_data:&[&str]) -> HashMap<String,String> {
        let mut map = HashMap::new();
        for i in split_data { 
            let data = parse(i.to_string());
            if data.0 != String::from("") && data.1 != String::from(""){
                map.insert(data.0, (data.1).replace("\r", "").replace(" ", "")); 
            }
      }
      map
    }
    pub fn parse(data:Headers) -> String{
        let h = data.headers;
        let mut raw:Vec<String> = Vec::new();
        for i in h.iter(){
            raw.push(format!("{}: {}",i.header,i.value));
        }
        let response: Vec<u8> = raw.join("\r\n").to_string().into_bytes();
        // response.extend(format!("NotFound",).as_bytes());
        String::from_utf8_lossy(&response).to_string()
    }
}

fn parse(data: String) -> (String, String) {
    let data_raw = data.as_bytes();
    let mut parsed_data: (String, String) = (String::from(""), String::from(""));
    for i in data_raw.iter().enumerate() {
        if i.1 == &b':' {
            let header_name = String::from_utf8_lossy(&data_raw[..i.0]).to_string();
            let header_value = String::from_utf8_lossy(&data_raw[i.0 + 1..]).to_string();
            parsed_data.0 = header_name;           
            parsed_data.1 = header_value;
            break;
        }
    }
    parsed_data
}


// fn main(){
//     let a = Headers::new(String::from("
//     GET /search?q=test HTTP/2
//     Host: www.bing.com
//     User-Agent: curl/7.54.0
//     Accept: */*
//     "));

// }