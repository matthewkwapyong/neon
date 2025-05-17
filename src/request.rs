use std::io::{Read, Write};
use std::net::TcpStream;

use crate::header;
use crate::header::Headers;
use crate::http::Methods;
// use crate::response::Body;
// use std::collections::HashMap;
// const CRLF:&str = "\r\n";
#[derive(Debug, Clone)]
pub struct Body(pub Vec<u8>);

impl ToString for Body {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Request {
    pub method: Methods,
    pub path: String,
    pub version: String,
    pub headers: Headers,
    pub body: Option<Body>,
    pub body_length: Option<usize>,
}
impl Request {
    pub fn new(mut stream: TcpStream) -> Result<(Request, TcpStream), String> {
        let mut request_buffer = Vec::new();
        let mut temp_buf = [0; 1024];
        let mut headers_end = 0;
        loop {
            let bytes_read = match stream.read(&mut temp_buf) {
                Ok(0) => {
                    // println!("sassasaa");
                    stream
                        .write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n")
                        .unwrap();
                    return Err("empty request".to_string());
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    return Err("read error".to_string());
                }
            };
            request_buffer.extend_from_slice(&temp_buf[..bytes_read]);
            if let Some(pos) = request_buffer.windows(4).position(|w| w == b"\r\n\r\n") {
                headers_end = pos + 4;
                break;
            }
            if request_buffer.len() > 8192 {
                stream
                    .write_all(b"HTTP/1.1 413 Payload Too Large\r\n\r\n")
                    .unwrap();
                break;
            }
        }
        let header_str: String = String::from_utf8_lossy(&request_buffer).to_string();
        // println!("{}", header_str);
        let split_headers = header_str.split("\r\n").collect::<Vec<&str>>();
        let parsed_headers = header::Headers::new(&split_headers[1..]);
        let method_path_version: Vec<&str> = split_headers[0].split(" ").collect();
        let method = match method_path_version[0] {
            "GET" => Methods::Get,
            "POST" => Methods::Post,
            "PUT" => Methods::Put,
            "OPTION" => Methods::Option,
            "DELETE" => Methods::Delete,
            _ => Methods::None,
        };
        if let Methods::Get = method {
            return Ok((
                Request {
                    method: method,
                    path: method_path_version[1].to_string(),
                    version: method_path_version[2].to_string(),
                    headers: parsed_headers,
                    body_length: Some(0),
                    body: None,
                },
                stream,
            ));
        }

        let content_length = parsed_headers
            .headers
            .get("Content-Length")
            .unwrap_or(&"0".to_string())
            .parse::<usize>()
            .unwrap_or(0);

        // println!("{} {}",content_length,request_buffer.len());

        let mut body = request_buffer[headers_end..].to_vec();
        let mut remaining = content_length.saturating_sub(body.len());
        while remaining > 0 {
            let mut buf = vec![0u8; remaining.min(1024)];
            let bytes_read = match stream.read(&mut buf) {
                Ok(0) => break, // client closed connection early
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading body: {}", e);
                    return Err("body read error".into());
                }
            };
            body.extend_from_slice(&buf[..bytes_read]);
            remaining -= bytes_read;
        }
        Ok((
            Request {
                method: method,
                path: method_path_version[1].to_string(),
                version: method_path_version[2].to_string(),
                headers: parsed_headers,
                body_length: Some(0),
                body: Some(Body(body)),
            },
            stream,
        ))
    }
    pub fn get_headers(&self) -> &Headers {
        &self.headers
    }
    pub fn get_method(&self) -> &Methods {
        &self.method
    }
    pub fn get_path(&self) -> &String {
        &self.path
    }
    // pub fn get_body(&self) -> Option<Vec<u8>> {
    //     self.body
    // }
}
