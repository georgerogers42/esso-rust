extern crate glob;
extern crate iron;
extern crate params;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;

mod articles;
mod handlers;

pub fn start(bind_addr: &str) {
    println!("Starting on: {}", bind_addr);
    handlers::start(&bind_addr);
}
