extern crate iron;
extern crate params;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;

mod handlers;

use iron::prelude::*;

fn main() {
    Iron::new(handlers::esso()).http("localhost:8080").unwrap();
}
