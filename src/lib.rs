pub mod header;
pub mod request;
pub mod response;
use request::Request;
use response::Response;

// use std::io::prelude::*;SocketAddr
use std::{io::Write, net::TcpListener};
use std::{thread};

// use std::sync::mpsc;
// mod self::header;
// use std::time::Duration;
// use super::header::Headers;

// use serde::{Deserialize, Serialize};
// use serde_json::Result;

pub struct Http {
    routes: Vec<Routes>,
}

#[derive(Clone)]
pub struct Routes {
    method: Methods,
    path: String,
    handler: fn(&Request, &mut Response),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Methods {
    Get,
    Post,
    None,
}

pub fn conv(data: &str) -> Vec<u8> {
    data.as_bytes().to_vec()
}

impl Methods {
    // pub fn unwrap(&self) -> (String, String) {
    //     match self {
    //         Methods::Get(a) => (String::from("GET"), a.to_string()),
    //         Methods::Post(b) => (String::from("POST"), b.to_string()),
    //     }
    // }
}
// impl Routes {
//     pub fn new(meth: Methods, body: Box<fn(Request, &mut Response)>) -> Routes {
//         return Routes {
//             method: meth,
//             body: body,
//         };
//     }
// }

impl Http {
    pub fn new() -> Http {
        return Http { routes: Vec::new() };
    }
    pub fn get(&mut self, path: String, handler: fn(&Request, &mut Response)) {
        self.routes.push(Routes {
            method: Methods::Get,
            path: path,
            handler: handler,
        });
    }
    pub fn post(&mut self, path: String, handler: fn(&Request, &mut Response)) {
        self.routes.push(Routes {
            method: Methods::Post,
            path: path,
            handler: handler,
        });
    }

    pub fn run(self) {
        let Self { routes } = self;
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let request = request::Request::new(&stream);
            let mut response = Response::build();
            let mut seen = false;

            // println!("{:?}",request);
            for i in routes.iter() {
                // println!("{:?}{}",i.method,i.path);
                if i.method == request.method {
                    if i.path == request.path {
                        seen = true;
                        (i.handler)(&request, &mut response);
                        let response: Vec<u8> = response.parse();
                        stream.write(&response).unwrap();
                        stream.flush().unwrap();
                        break;
                    }
                }
            }
            if !seen {
                let h = ["HTTP/1.1 404 NOTFOUND ", "\r\n"];
                let mut response: Vec<u8> = h.join("\r\n").to_string().into_bytes();
                response.extend(format!("NotFound",).as_bytes());
                stream.write(&response).unwrap();
                stream.flush().unwrap();
            }

        }
    }
}
