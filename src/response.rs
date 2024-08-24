use crate::header::Headers;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Response {
    status:u16,
    pub headers: Headers,
    pub body: Vec<u8>,
}
impl Response {
    pub fn build() -> Response {
        Response {
            status:200,
            headers: Headers {
                headers: HashMap::new(),
            },
            body: Vec::new(),
        }
    }
    pub fn insert(&mut self, head: &str, value: &str) {
        // self.headers.push( format!("{}: {}",head.to_string(),value.to_string()))
        self.headers
            .headers
            .insert(head.to_string(), value.to_string());
    }
    pub fn body(&mut self, data: Vec<u8>) {
        self.body = data;
    }

    pub fn parse(&mut self) -> Vec<u8> {
        // res.head("\r\n");
        // self.headers.push("\r\n".to_string());
        let mut man: Vec<u8> = vec![];
        let code = format!("HTTP/1.1 {} OK\r\n",self.status).as_bytes().to_vec();
        let mut response = Headers::parse_to_u8(&self.headers);
        man.extend(code);
        man.extend(&response);
        response.extend(&self.body);
        response
    }
}




