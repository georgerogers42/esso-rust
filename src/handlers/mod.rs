use hbs;
use std::path::Path;
use std::collections::HashMap;

use iron;
use iron::prelude::*;
use router::Router;
use mount::Mount;
use staticfile::Static;

use articles::*;

fn articles_h(_req: &mut Request, articles: &Vec<Article>) -> IronResult<Response> {
    let mut data = HashMap::new();
    data.insert("articles".to_string(), articles.clone());
    let resp = Response::with((iron::status::Ok, hbs::Template::new("hello", data)));
    Ok(resp)
}

pub type App = Mount;

pub fn esso() -> App {
    let mut routes = Router::new();
    {
        let articles = load_articles("articles/*.html");
        routes.get("/", move |req: &mut Request| {
            articles_h(req, &articles)
        });
    }
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
