use bson::{Document, Bson};
use mongodb::{Client, ThreadedClient};
use mongodb::{coll::Collection};
use mongodb::db::ThreadedDatabase;
use dotenv::dotenv;
use std::{env, option::Option};
use super::models::{VolInfo, VolTrack};



pub fn get_coll(coll_name: &str) -> Collection {
    dotenv().ok();
    let db_address = env::var("DB_ADDRESS").unwrap();
    let db_port = env::var("DB_PORT").unwrap();
    let db_name = env::var("DB_NAME").unwrap();

    let client = Client::connect(
        &db_address,
        db_port.parse().unwrap(),
    ).expect("Failed to initialize client.");
    let db = client.db(&db_name);
    db.collection(coll_name)
}

pub fn get_tracks(doc: &Document) -> Vec<VolTrack> {
    let tracks: Vec<VolTrack> = doc
        .get_array("tracks")
        .unwrap()
        .iter()
        .map(|i| {
            let c = i.as_document().unwrap();
            VolTrack {
                id: get_i64(c, "id"),
                vol: get_i64(c, "vol"),
                name: get_string(c, "name"),
                artist: get_string(c, "artist"),
                album: get_string(c, "album"),
                cover: get_string(c, "cover"),
                url: get_string(c, "url"),
                color: get_string(c, "color"),
            }
        })
        .collect();
    tracks
}

pub fn count_comments(doc: Document) -> i32 {
    match doc.get_array("comments") {
        Ok(c) => c.len() as i32,
        _ => 0
    }
}

pub fn doc_to_vol_info(doc: Document) -> VolInfo {
    println!("1");

    let tags = match doc.get_array("tags") {
        Ok(s) => {
            println!("3");
            Some(s.into_iter().map(|i| i.as_str().unwrap().to_string()).collect())
        },
        _ => None
    };
    let similar_vols= match doc.get_array("similarVols") {
        Ok(s) => {
            println!("4");
            Some(s.into_iter().map(|i| i.as_i64().unwrap().to_owned()).collect())
        },
        _ => None
    };

    println!("2");

    return VolInfo {
        id: get_i64(&doc, "id"),
        vol: get_i64(&doc, "vol"),
        title: get_string(&doc, "title"),
        link: get_string(&doc, "link"),
        cover: get_string(&doc, "cover"),
        color: get_string(&doc, "color"),
        author: get_string(&doc, "author"),
        author_avatar: get_string(&doc, "authorAvatar"),
        date: get_string(&doc, "date"),
        desc: get_string(&doc, "desc"),
        tags,
        similar_vols,
        tracks: get_tracks(&doc)
    };
}

pub fn get_i64(doc: &Document, key: &str) -> i64 {
    doc.get_i64(key).unwrap().to_owned()
}

pub fn get_string(doc: &Document, key: &str) -> String {
    doc.get_str(key).unwrap().to_owned()
}
//
//pub fn get_vec<T>(doc: &Document, key: &str) -> Vec<T> {
//    doc
//        .get_array("tags").unwrap().iter()
//        .map(|i| i.as_str().unwrap().to_string()).collect()
//}
