use crate::header::Headers;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::ops::Deref;

pub struct ResponseStream {
    pub stream: TcpStream,
    pub response: Response,
}
impl Deref for ResponseStream {
    type Target = Response;
    fn deref(&self) -> &Self::Target {
        &self.response
    }
}
impl ResponseStream {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            response: Response::build(),
        }
    }
    pub fn body(mut self, body: Vec<u8>) {
        self.response.body = body;
    }
}
impl Drop for ResponseStream {
    fn drop(&mut self) {
        self.stream.write(&&self.response.raw()).unwrap();
    }
}
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: Headers,
    pub body: Vec<u8>,
}
impl Response {
    pub fn build() -> Response {
        Response {
            status: 200,
            headers: Headers {
                headers: HashMap::new(),
            },
            body: Vec::new(),
        }
    }
    pub fn status(&mut self, status: u16) {
        self.status = status;
    }
    pub fn set_header(&mut self, head: &str, value: &str) {
        // self.headers.push( format!("{}: {}",head.to_string(),value.to_string()))
        self.headers
            .headers
            .insert(head.to_string(), value.to_string());
    }
    pub fn body(&mut self, data: Vec<u8>) {
        self.body = data;
    }
    pub fn body_with_type<T: Body>(mut self, data: T) {
        let data = data.raw();
        self.headers
            .headers
            .insert("Content-type".to_string(), data.0);
        self.body = data.1;
    }
    pub fn raw(&mut self) -> Vec<u8> {
        // self.headers.push("\r\n".to_string());
        let mut response: Vec<u8> = vec![];
        let code = format!("HTTP/1.1 {} OK\r\n", self.status)
            .as_bytes()
            .to_vec();
        let header_raw = Headers::parse_to_u8(&self.headers);
        response.extend(code);
        response.extend(&header_raw);
        response.extend(b"\r\n\r\n");
        response.extend(&self.body);
        response
    }
}
pub trait Body {
    fn raw(self) -> (String, Vec<u8>);
}
