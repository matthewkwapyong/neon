use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Headers {
    pub headers: HashMap<String, String>,
}
impl Headers {
    pub fn new(split_data: &[&str]) -> Headers {
        let mut map = HashMap::new();
        for i in split_data {
            let data = Self::split_header(i.to_string());
            if data.0 != String::from("") && data.1 != String::from("") {
                map.insert(data.0, (data.1).replace("\r", "").replace(" ", ""));
            }
        }
        Headers { headers: map }
    }
    pub fn parse_to_string(data: &Headers) -> String {
        let mut raw: Vec<String> = Vec::new();
        for (header, value) in &data.headers {
            raw.push(format!("{}: {}", header, value));
        }
        let response: Vec<u8> = raw.join("\r\n").to_string().into_bytes();
        // response.extend(format!("NotFound",).as_bytes());
        String::from_utf8_lossy(&response).to_string()
    }
    pub fn parse_to_u8(data: &Headers) -> Vec<u8> {
        let mut raw: Vec<String> = Vec::new();
        for (header, value) in &data.headers {
            raw.push(format!("{}: {}", header, value));
        }
        let response: Vec<u8> = raw.join("\r\n").to_string().into_bytes();
        response
    }
    fn split_header(data: String) -> (String, String) {
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
}
