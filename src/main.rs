extern crate actix;
extern crate actix_web;
#[macro_use(bson, doc)]
extern crate bson;
extern crate dotenv;
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate listenfd;
extern crate mongodb;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


use actix_web::{App, fs, http, server};
use listenfd::ListenFd;

use api::{get_singles_info, get_vols_info};

pub mod db;
pub mod api;
pub mod utils;

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        vec![
            App::new()
                .prefix("/api")
                .resource("/vols/{from}", |r| {
                    r.method(http::Method::GET).f(get_vols_info);
                })
                .resource("/singles/{from_id}", |r| {
                    r.method(http::Method::GET).f(get_singles_info);
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
