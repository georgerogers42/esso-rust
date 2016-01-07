use hbs;
use std::path::Path;
use std::collections::HashMap;

use iron;
use iron::prelude::*;
use router::Router;
use mount::Mount;
use staticfile::Static;
use params::*;

fn hello(req: &mut Request) -> IronResult<Response> {
    let params = req.get_ref::<Params>().unwrap();
    let who = params.get("who").map(|x| { String::from_value(x).unwrap() }).unwrap_or("World".to_string());
    let mut data = HashMap::new();
    data.insert("who".to_string(), who);
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

pub fn start(bind_addr: &str) {
    Iron::new(esso()).http(bind_addr).unwrap();
}
