use crate::header;
use crate::header::Headers;
use crate::Methods;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Request {
    method: Methods,
    path: String,
    version: String,
    pub headers: Headers,
    body: Option<Vec<u8>>,
    body_length: Option<usize>,
}

// #[derive(Debug, Clone)]
// pub enum Typo{
//     File(u8),
//     String(String),
//     None
// }

impl Request {
    pub fn new(http_request: Vec<u8>) -> Request{
        let http_request_str: String = String::from_utf8_lossy(&http_request).to_string();
        let mut body_startidx = 0;
        if let Some(header_end) = http_request_str.find("\r\n\r\n") {
            // Return everything after the headers as the body
            body_startidx = header_end + 4;
        }
        println!("{}",body_startidx);
        let var_name = &http_request_str[..body_startidx - 4];
        let headers = var_name;
        // println!("body idx {} \n\n\n\n\n\n\n {:?}",body_startidx,&headers);
        let split_headers = headers.split("\r\n").collect::<Vec<&str>>();
        // MPV = method path version
        let method_path_version: Vec<&str> = split_headers[0].split(" ").collect();
        let parsed_headers = header::Headers::new(&split_headers[1..]);
        match method_path_version[0] {
            "GET" => Request {
                method: Methods::Get,
                path: method_path_version[1].to_string(),
                version: method_path_version[2].to_string(),
                headers: parsed_headers,
                body: None,
                body_length:None
            },
            "POST" => {
                let body = http_request[body_startidx..].to_vec();
                Request {
                    method: Methods::Post,
                    path: method_path_version[1].to_string(),
                    version: method_path_version[2].to_string(),
                    headers: parsed_headers,
                    body_length:Some(*&body.len()),
                    body: Some(body),
                }
            },
            _ => {
                panic!("method error")
            }
        }
    }
    pub fn get_headers(&self) -> &Headers{
        &self.headers
    }
    pub fn get_method(&self) -> &Methods{
        &self.method
    }
    pub fn get_path(&self) -> &String{
        &self.path
    }





    // pub fn parse(data: &Vec<S>) -> Request {
    //     // println!("{}",data[0]);
    //     let headers = header::Headers::new(data);

    //     let path: Vec<&str> = data[0].split(" ").collect();
    //     let mut body_idx = 0usize;
    //     if let Some(body_index) = data.iter().position(|line| line.is_empty()) {
    //         body_idx = body_index;
    //     }

    //     let mut method: Methods;
    //     match path[0] {
    //         "GET" => Request {
    //             method: Methods::Get,
    //             path: path[1].to_string(),
    //             version: path[2].to_string(),
    //             headers: headers,
    //             body: None,
    //         },
    //         "POST" => Request {
    //             method: Methods::Get,
    //             path: path[1].to_string(),
    //             version: path[2].to_string(),
    //             headers: headers,
    //             body: None,
    //         },
    //         _ => {
    //             panic!("method error")
    //         }
    //     }

    //     // println!("{:?}",path);
    //     // let req_str = String::from_utf8_lossy(&data[..]).to_string();
    //     // println!("{:?}",headers);
    //     // req
    // }
    // pub fn parse_wbody(data:&Vec<u8>) -> Request{
    //     let req_str = String::from_utf8_lossy(&data[..]).to_string();
    //     let headers = header::Headers::new(req_str.clone());
    //     let path: Vec<&str> = req_str.lines().next().unwrap().split(" ").collect();
    //     let cursor = io::Cursor::new(data);
    //     let mut split_iter = cursor.split(b'\n').map(|l| l.unwrap());
    //     let mut body:Vec<u8> = Vec::new();
    //     let mut values:Vec<Vec<u8>> = Vec::new();
    //     for i in split_iter.into_iter() {
    //         values.push(i);
    //     }
    //     let body = &values[values.len() - 1];
    //     let req = Request {method:path[0].to_string(),path:path[1].to_string(),headers:headers.headers,body:body.to_vec()};
    //     req
    // }
}
