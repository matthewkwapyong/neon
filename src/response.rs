#[derive(Debug, Clone)]
pub struct Response {
    pub headers: Vec<String>,
    pub body: Vec<u8>,
}
impl Response {
    pub fn build() -> Response{
        Response { headers:Vec::new(), body:Vec::new()}
    }
    pub fn head(&mut self, head: &str) {
        // self.headers.push( format!("{}: {}",head.to_string(),value.to_string()))
        self.headers.push( head.to_string())
    }
    pub fn body(&mut self, data: Vec<u8>) {
        self.body = data;
    }
    pub fn parse(&mut self) -> Vec<u8> {
        // res.head("\r\n");
        self.headers.push("\r\n".to_string());
        let mut response: Vec<u8> = self.headers.join("\r\n").to_string().into_bytes();
        // println!("{}",self.headers.join("\r\n").to_string());
        response.extend(&self.body);
        response
    }
}