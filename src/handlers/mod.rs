use iron;
use iron::prelude::*;
use hbs;
use std::path::Path;
use std::collections::HashMap;

use rustc_serialize::json::Json;
use router::Router;
use mount::Mount;
use staticfile::Static;

fn hello(r: &mut Request) -> IronResult<Response> {
    let who = r
    let mut data = HashMap::new();
    data.insert("who", 
    let resp = Response::with((iron::status::Ok, hbs::Template::new("hello", data)));
    Ok(resp)
}

pub type App = Mount;

pub fn esso() -> App {
    let mut routes = Router::new();
    routes.get("/", hello);
    let mut app = Chain::new(routes);
    let mut hb = hbs::HandlebarsEngine::new2();
    hb.add(Box::new(hbs::DirectorySource::new("templates/", ".html")));
    hb.reload().unwrap();
    app.link_after(hb);
    let mut full = Mount::new();
    full.mount("/", app);
    full.mount("/static/", Static::new(Path::new("public/static")));
    return full;
}

