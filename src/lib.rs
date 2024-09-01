pub mod header;
pub mod request;
pub mod response;
use request::Request;
use response::{Response, ResponseStream};
use std::io::{self, BufRead, BufReader};
use std::net::ToSocketAddrs;
// use std::io::prelude::*;SocketAddr
use std::process;
use std::sync::Arc;
use std::thread;
use std::{io::Write, net::TcpListener};

// use proc_macro::TokenStream;

// use syn;
// use std::sync::mpsc;
// mod self::header;
// use std::time::Duration;
// use super::header::Headers;

// use serde::{Deserialize, Serialize};
// use serde_json::Result;
pub struct Http {
    listener: TcpListener,
}
#[derive(Clone)]
pub struct Route {
    method: Methods,
    path: String,
    handler: fn(&Request, &mut Response),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Methods {
    Get,
    Post,
    Option,
    Put,
    Delete,
    None,
}

pub struct IncomingConnections<'a> {
    listener: &'a TcpListener,
}
impl<'a> Iterator for IncomingConnections<'a> {
    type Item = io::Result<(Request, ResponseStream)>;

    fn next(&mut self) -> Option<Self::Item> {
        // `accept` will block until a new connection is available
        let res = match self.listener.accept() {
            Ok((mut stream, _addr)) => {
                let mut buf_reader = BufReader::new(&mut stream);
                match buf_reader.fill_buf() {
                    Ok(buf) => {
                        let request = Request::new(buf.to_vec());
                        let r = ResponseStream::new(stream);
                        Ok((request, r))
                    }
                    Err(e) => {
                        eprintln!("Failed to read buffer: {}", e);
                        Err(e)
                        // process::exit(1);
                    }
                }
            },
            Err(e) =>{
                Err(e)
            }
        };
        Some(res)

        // Some(self.listener.accept().map(|(mut stream, _addr)| {
        //     let mut buf_reader = BufReader::new(&mut stream);
        //     match buf_reader.fill_buf() {
        //         Ok(buf) => {
        //             let request = Request::new(buf.to_vec());
        //             let r = ResponseStream::new(stream);
        //             (request, r)
        //         }
        //         Err(e) => {
        //             eprintln!("Failed to read buffer: {}", e);
        //             process::exit(1);
        //         }
        //     }
        // }))
    }
}
pub struct Router {
    routes: Vec<Route>,
}
impl Router {
    pub fn new() -> Self {
        Router { routes: Vec::new() }
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
}
impl Http {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Http, std::io::Error> {
        let socket_addr = addr.to_socket_addrs().unwrap().next().unwrap();
        let mut addr = socket_addr;
        let listener = loop {
            match TcpListener::bind(addr) {
                Ok(a) => {
                    println!("listening on port {}", addr.port());
                    break a;
                }
                Err(b) => {
                    if let Some(code) = b.raw_os_error() {
                        if code == 10048 {
                            let mut port = addr.port();
                            println!("Err: Port {} is already being used", port);
                            port += 1;
                            let new_port = port;
                            addr.set_port(new_port);
                            println!("trying port {}", new_port);
                            continue;
                        }
                    }
                    process::exit(1)
                }
            };
        };
        Ok(Http { listener: listener })
    }
    pub fn incoming(&self) -> IncomingConnections {
        IncomingConnections {
            listener: &self.listener,
        }
    }

    pub fn listen(self, route: Router) {
        let routes = Arc::new(route.routes);
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let route = Arc::clone(&routes);
            let _ = thread::spawn(move || {
                let mut seen = false;

                let mut buf_reader = BufReader::new(&mut stream);
                let request = match buf_reader.fill_buf() {
                    Ok(buf) => {
                        let request = Request::new(buf.to_vec());
                        request
                    }
                    Err(e) => {
                        eprintln!("Failed to read buffer: {}", e);
                        process::exit(1);
                    }
                };
                for i in route.iter() {
                    if &i.method == request.get_method() && &i.path == request.get_path() {
                        seen = true;
                        let mut response = Response::build();
                        (i.handler)(&request, &mut response);
                        let response: Vec<u8> = response.raw();
                        stream.write(&response).unwrap();
                        stream.flush().unwrap();
                        break;
                    }
                }
                if !seen {
                    let raw = ["HTTP/1.1 404 NOTFOUND ", "\r\n"];
                    let mut response: Vec<u8> = raw.join("\r\n").to_string().into_bytes();
                    response.extend(format!("NotFound",).as_bytes());
                    stream.write(&response).unwrap();
                    stream.flush().unwrap();
                }
            });
        }
    }
}
