extern crate iron;
extern crate params;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;

use std::env::var;
mod handlers;

fn main() {
    let bind_addr = format!("0.0.0.0:{}", var("PORT").unwrap_or("8080".to_string()));
    handlers::start(&bind_addr);
}
