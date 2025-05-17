use crate::request;
use crate::response;
use request::Request;
use response::{Response, ResponseStream};
use std::io::Read;
use std::io::{self, BufRead, BufReader};
use std::iter;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
// use std::io::prelude::*;SocketAddr
use std::process;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
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
    workers: Vec<Worker>,
}
#[derive(Clone)]
pub struct Route {
    method: Methods,
    path: String,
    handler: fn(Request, &mut Response),
    middleware:Vec<fn(&mut Request, &mut Response)>
}
struct Worker {
    thread_handle: JoinHandle<()>,
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
// impl<'a> Iterator for IncomingConnections<'a> {
//     type Item = io::Result<(Request, ResponseStream)>;

//     fn next(&mut self) -> Option<Self::Item> {
//         // `accept` will block until a new connection is available
//         let res = match self.listener.accept() {
//             Ok((mut stream, _addr)) => {
//                 let mut buf_reader = BufReader::new(&mut stream);
//                 match buf_reader.fill_buf() {
//                     Ok(buf) => {
//                         let request = Request::new(buf.to_vec());
//                         let r = ResponseStream::new(stream);
//                         Ok((request, r))
//                     }
//                     Err(e) => {
//                         eprintln!("Failed to read buffer: {}", e);
//                         Err(e)
//                         // process::exit(1);
//                     }
//                 }
//             }
//             Err(e) => Err(e),
//         };
//         Some(res)
//     }
// }
impl Route{
    pub fn get(path: &str, handler: fn(Request, &mut Response)) -> Self {
        Route {
            method: Methods::Get,
            path: path.to_string(),
            handler,
            middleware:Vec::new(),
        }
    }
    pub fn post(path: &str, handler: fn(Request, &mut Response)) -> Self {
        Route {
            method: Methods::Post,
            path: path.to_string(),
            handler,
            middleware:Vec::new(),
        }
    }
    pub fn set_middleware(mut self,handler: fn(&mut Request, &mut Response)) -> Self{
        self.middleware.push(handler);
        self
    }
}
pub struct Router {
    routes: Vec<Route>,
    middleware:Vec<fn(&mut Request, &mut Response)>
}

impl Router {
    pub fn new() -> Self {
        Router { routes: Vec::new(),middleware:Vec::new() }
    }
    pub fn route(&mut self, handler: Route) {
        self.routes.push(handler);
    }
    pub fn set_routes(&mut self,routes:Vec<Route>){
        for i in routes{
            self.routes.push(i);
        }
    }
    pub fn set_middleware(mut self,handler: fn(&mut Request, &mut Response)) -> Self{
        self.middleware.push(handler);
        self
    }
    // pub fn get(&mut self, handler: Route) {
    //     self.routes.push(handler);
    // }
    // pub fn post(&mut self, path: &str, handler: Route) {
    //     self.routes.push(handler);
    // }
}
impl Http {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Http, std::io::Error> {
        let socket_addr = addr.to_socket_addrs().unwrap().next().unwrap();
        let mut addr = socket_addr;
        let listener = loop {
            match TcpListener::bind(addr) {
                Ok(a) => {
                    println!("{addr}");
                    println!("listening on port {}", addr.port());
                    break a;
                }
                Err(e) => {
                    if let Some(code) = e.raw_os_error() {
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
                    println!("Err {}", e);
                    process::exit(1)
                }
            };
        };
        Ok(Http {
            listener,
            workers: Vec::new(),
        })
    }
    pub fn incoming(&self) -> IncomingConnections {
        IncomingConnections {
            listener: &self.listener,
        }
    }
    pub fn listen(&mut self, route: Router) {
        let routes = Arc::new(route.routes);
        let middleware = Arc::new(route.middleware);

        for stream in self.listener.incoming() {
            // println!("workers\n {}",self.workers.len());
            let mut stream = stream.unwrap();
            let route = Arc::clone(&routes);
            let worker = thread::spawn(move || {
                let mut seen = false; 
                let (mut request, mut stream) = match Request::new(stream){
                    Ok((req,s)) => (req,s),
                    Err(err) => panic!("Error: {}",err),
                };
                // println!("{:?} -> {:?}",request.get_method(),request.get_path());
              
                for i in route.iter() {
                    if &i.method == request.get_method() && &i.path == request.get_path() {
                        seen = true;
                        let mut response = Response::build();
                        let mut requestt = &mut request;
                        if i.middleware.len() != 0{
                            for j in i.middleware.iter(){
                                (j)(requestt, &mut response);
                            }
                        }
                        (i.handler)(request, &mut response);
                        let response: Vec<u8> = response.raw();
                        stream.write(&response).unwrap();
                        stream.flush().unwrap();
                        break;
                    }
                }
                if !seen {
                    let raw = ["HTTP/1.1 404 NOTFOUND ", "\r\n"];
                    let response: Vec<u8> = raw.join("\r\n").to_string().into_bytes();
                    // response.extend(format!("path {}",&request.get_path()).as_bytes());
                    stream.write(&response).unwrap();
                    stream.flush().unwrap();
                }
            });
            let workerr = Worker {
                thread_handle: worker,
            };
            self.workers.push(workerr);
        }
    }
}
