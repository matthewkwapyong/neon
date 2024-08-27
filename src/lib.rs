pub mod header;
pub mod request;
pub mod response;
use request::Request;
use response::Response;

use std::io::{self, BufRead, BufReader};
use std::net::{Incoming, SocketAddr, TcpStream, ToSocketAddrs};
// use std::io::prelude::*;SocketAddr
use std::process;
use std::thread;
use std::{io::Write, net::TcpListener};
// use std::sync::mpsc;
// mod self::header;
// use std::time::Duration;
// use super::header::Headers;

// use serde::{Deserialize, Serialize};
// use serde_json::Result;

pub struct Http {
    port: u16,
    listener: TcpListener,
}
pub struct IncomingConnections<'a> {
    listener: &'a TcpListener,
}
#[derive(Clone)]
pub(crate) struct Route {
    method: Methods,
    path: String,
    handler: fn(&Request, &mut Response),
}
pub struct Router {
    routes: Vec<Route>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Methods {
    Get,
    Post,
    None,
}

impl<'a> Iterator for IncomingConnections<'a> {
    type Item = io::Result<Request>;

    fn next(&mut self) -> Option<Self::Item> {
        // `accept` will block until a new connection is available
        Some(self.listener.accept().map(|(mut stream, _addr)| {
            let mut buf_reader = BufReader::new(&mut stream);
            let http_request: Vec<u8> = buf_reader.fill_buf().unwrap().to_vec();
            let request = request::Request::new(http_request);
            request
        }))
    }
}

impl Router {
    pub fn new() -> Self{
        Router{
            routes:Vec::new()
        }
    }
    pub fn get(&mut self, path: &str, handler: fn(&Request, &mut Response)) {
        self.routes.push(Route {
            method: Methods::Get,
            path: path.to_string(),
            handler: handler,
        });
    }
    pub fn post(&mut self, path: &str, handler: fn(&Request, &mut Response)) {
        self.routes.push(Route {
            method: Methods::Post,
            path: path.to_string(),
            handler: handler,
        });
    }
    pub fn run(self, http: Http) {}
}

// impl Routes {
//     pub fn new(meth: Methods, body: Box<fn(Request, &mut Response)>) -> Routes {
//         return Routes {
//             method: meth,
//             body: body,
//         };
//     }
// }
// impl Iterator for Http{
//     type Item = Request;
//     fn next(&mut self) -> Incoming<'_>{
//         self.stream.incoming()
//     }
// }
impl Http {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Http, std::io::Error> {
        let listener = TcpListener::bind(addr)?;
        Ok(Http {
            port: 3000,
            listener: listener,
        })
    }
    pub fn incoming(&self) ->IncomingConnections{
        IncomingConnections{
            listener: &self.listener
        }
    }


    pub fn listen(self, route:Router) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let route = route.routes.clone();
            let _ = thread::spawn(move || {
                let mut seen = false;
                let mut buf_reader = BufReader::new(&mut stream);
                let http_request: Vec<u8> = buf_reader.fill_buf().unwrap().to_vec();
                let request = request::Request::new(http_request);

                for i in route.into_iter(){
                    if &i.method == request.get_method() && &i.path == request.get_path(){
                        seen = true;
                        let mut response = Response::build();
                        (i.handler)(&request, &mut response);
                        let response: Vec<u8> = response.parse();
                        stream.write(&response).unwrap();
                        stream.flush().unwrap();
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
            })
            .join();
        }
    }
}
