#[macro_use(bson, doc)]
extern crate bson;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate mongodb;
extern crate dotenv;
extern crate futures;
extern crate actix_web;
extern crate actix;
extern crate listenfd;
extern crate qiniu;

pub mod db;
pub mod api;
pub mod utils;

use listenfd::ListenFd;
use actix_web::{server, http, App, fs};
use api::{get_post, get_posts, create_post, get_upload_token};



fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        vec![
            App::new()
                .prefix("/api")
                .resource("/posts", |r| {
                    r.method(http::Method::GET).f(get_posts);
                    r.method(http::Method::POST).with(create_post);
                })
                .resource("/post/{id}", |r| {
                    r.method(http::Method::GET).f(get_post);
                })
                .resource("/upload_token", |r| {
                    r.method(http::Method::GET).f(get_upload_token);
                })
                .boxed(),
            App::new()
                .handler("/",
                    fs::StaticFiles::new("./static").ok().unwrap().index_file("index.html"))
                .boxed()
        ]
    });

    server
        .bind("127.0.0.1:8087")
        .expect("Can not bind to 127.0.0.1:8087")
        .run();

//    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
//        server.listen(l)
//    } else {
//        server.bind("127.0.0.1:8087").unwrap();
//    };

//    server.run();
}
