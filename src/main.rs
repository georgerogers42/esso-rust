extern crate esso_rust;
use std::env::var;

fn main() {
    let bind_addr = format!("0.0.0.0:{}", var("PORT").unwrap_or("8080".to_string()));
    esso_rust::start(&bind_addr);
}
