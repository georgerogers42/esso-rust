use hbs;
use std::path::Path;
use std::collections::HashMap;

use iron;
use iron::prelude::*;
use router::Router;
use mount::Mount;
use staticfile::Static;
use rustc_serialize::json::ToJson;

use articles::*;

fn article_h(req: &mut Request, articles: &Vec<Article>, amap: &HashMap<String, Article>) -> IronResult<Response> {
    let router = req.extensions.get::<Router>().unwrap();
    let mut data = HashMap::new();
    data.insert("article".to_string(), amap.get(router.find("slug").unwrap()).unwrap().to_json());
    data.insert("articles".to_string(), articles.to_json());
    let resp = Response::with((iron::status::Ok, hbs::Template::new("article", data)));
    Ok(resp)
}

fn articles_h(_req: &mut Request, articles: &Vec<Article>) -> IronResult<Response> {
    let mut data = HashMap::new();
    data.insert("articles".to_string(), articles.to_json());
    let resp = Response::with((iron::status::Ok, hbs::Template::new("articles", data)));
    Ok(resp)
}

pub type App = Mount;

pub fn esso() -> App {
    let mut routes = Router::new();
    let mut articles = load_articles("articles/*.html");
    articles.sort_by(|a, b| { a.meta.posted.cmp(&b.meta.posted) });
    {
        let avec = articles.clone();
        let amap = articles_map(&avec);
        routes.get("/:slug", move |req: &mut Request| {
            article_h(req, &avec, &amap)
        });
    }
    {
        let avec = articles.clone();
        routes.get("/", move |req: &mut Request| {
            articles_h(req, &avec)
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
