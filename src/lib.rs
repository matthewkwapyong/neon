pub mod header;
pub mod request;
pub mod response;
use request::Request;
use response::Response;

use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
// use std::io::prelude::*;SocketAddr
use std::thread;
use std::{io::Write, net::TcpListener};
use std::process;
// use std::sync::mpsc;
// mod self::header;
// use std::time::Duration;
// use super::header::Headers;

// use serde::{Deserialize, Serialize};
// use serde_json::Result;

pub struct Http {
    routes: Vec<Routes>,
    port:u16
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
        return Http { routes: Vec::new() ,port:3000};
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
    pub fn port(mut self,port:u16) -> Self{
        self.port = port;
        self
    }
    pub fn listen(mut self,port:u16) {
        println!("{}",self.routes.len());
        self.port = port;
        let addr = SocketAddr::from(([127, 0, 0, 1],self.port));
        let listener = match TcpListener::bind(addr){
            Ok(a) => {
                println!("listening on port {}",self.port);
                a
            },
            Err(b) =>{
                if let Some(code) = b.raw_os_error(){
                    if code == 10048{
                        println!("Err: Port {} is already being used", self.port);
                        process::exit(1);
                    }
                }
                process::exit(1);
            }
        };
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!(" addr {}",&stream.peer_addr().unwrap());  
            let routes = self.routes.clone();
            let _ = thread::spawn(move || {
                let mut seen = false;
                let mut buf_reader = BufReader::new(&mut stream);
                let http_request: Vec<u8> = buf_reader.fill_buf().unwrap().to_vec();
                
                let request = request::Request::new(http_request);
                for i in routes.clone().into_iter() {
                    if i.method == request.method && i.path == request.path {
                        seen = true;
                        let mut response = Response::build();
                        (i.handler)(&request, &mut response);
                        let response: Vec<u8> = response.parse();
                        stream.write(&response).unwrap();
                        stream.flush().unwrap();
                        // (i.handler)(&request, &mut response);
                        break;
                    }
                }
                if !seen {
                    let h = ["HTTP/1.1 404 NOTFOUND ", "\r\n"];
                    let mut response: Vec<u8> = h.join("\r\n").to_string().into_bytes();
                    response.extend(format!("NotFound",).as_bytes());
                    stream.write(&response).unwrap();
                    stream.flush().unwrap();
                }
            }).join();
        }
    }
}
