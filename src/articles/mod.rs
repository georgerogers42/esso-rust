use std::io;
use std::io::{Read, BufRead, BufReader};
use std::fs::File;
use glob::glob;
use std::collections::HashMap;
use rustc_serialize::json;

#[derive(PartialEq, Clone, RustcDecodable, RustcEncodable) ]
pub struct Meta {
    pub title: String,
    pub author: String,
    pub slug: String,
    pub posted: String
}

impl json::ToJson for Meta {
    fn to_json(&self) -> json::Json {
        let mut hm = HashMap::new();
        hm.insert("title".to_string(), self.title.to_json());
        hm.insert("author".to_string(), self.author.to_json());
        hm.insert("slug".to_string(), self.slug.to_json());
        hm.insert("posted".to_string(), self.posted.to_json());
        hm.to_json()
    }
}

#[derive(PartialEq, Clone, RustcDecodable, RustcEncodable) ]
pub struct Article {
    pub meta: Meta,
    pub contents: String
}

impl json::ToJson for Article {
    fn to_json(&self) -> json::Json {
        let mut hm = HashMap::new();
        hm.insert("meta".to_string(), self.meta.to_json());
        hm.insert("contents".to_string(), self.contents.to_json());
        hm.to_json()
    }
}

pub fn load_article(fname: &str) -> io::Result<Article> {
    let file = try!(File::open(fname));
    let mut rdr = BufReader::new(file);
    let mut mpara = String::new();
    loop {
        let mut line = String::new();
        try!(rdr.read_line(&mut line));
        if line == "\n" {
            break;
        }
        mpara.push_str(&line);
    }
    let meta = json::decode(&mpara).unwrap();
    let mut contents = String::new();
    try!(rdr.read_to_string(&mut contents));
    Ok(Article { meta: meta, contents: contents })
}

pub fn load_articles(pat: &str) -> Vec<Article> {
    let mut articles = vec![];
    for path in glob(pat).unwrap() {
        match load_article(path.unwrap().to_str().unwrap()) {
            Ok(art) => {
                articles.push(art);
            }, Err(_) => {
            }
        }
    }
    articles
}

pub fn articles_map(articles: &Vec<Article>) -> HashMap<String, Article> {
    let mut amap = HashMap::new();
    for article in articles {
        amap.insert(article.meta.slug.clone(), article.clone());
    }
    amap
}
