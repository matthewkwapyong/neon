#Example
```rust
use neon::{Http, Methods, Router};
fn main(){
	let app = Http::new("0.0.0.0:2000").unwrap();
	let mut route = Router::new();
	route.get("/",|_req,res|{
		res.body("hello world".as_bytes().to_vec())
	});

    route.get("/he", |_req, res| {
	    res.insert("Content-type", " text/json");
	    res.body(r#"{"name":"john"}"#.as_bytes().to_vec());
	});
	
    app.listen(route)
}
```
