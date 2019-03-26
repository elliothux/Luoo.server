use std::{env, option::Option};

use bson::{Bson, Document};
use dotenv::dotenv;
use mongodb::{Client, ThreadedClient};
use mongodb::{coll::Collection};
use mongodb::db::ThreadedDatabase;

use super::models::{Single, VolInfo, VolTrack, Article, ArticleTrack};


lazy_static! {
    pub static ref LYRIC_COLLECTION: Collection = get_coll("lyrics");
}

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

pub fn get_vol_tracks(doc: &Document) -> Option<Vec<VolTrack>> {
    match doc.get_array("tracks") {
        Ok(i) => {
            Some(
                i
                    .into_iter()
                    .filter_map(|i| {
                        match i.as_document() {
                            Some(c) => {
                                let id = get_i32(c, "id");
                                Some(
                                    VolTrack {
                                        id,
                                        vol: get_i32(c, "vol"),
                                        name: get_string(c, "name"),
                                        artist: get_string(c, "artist"),
                                        album: get_string(c, "album"),
                                        cover: get_string(c, "cover"),
                                        url: get_string(c, "url"),
                                        color: get_string(c, "color"),
                                        lyric: get_lyric(LyricType::VolTrack, id)
                                    }
                                )
                            },
                            None => None
                        }
                    })
                    .collect()
            )
        }
        _ => None
    }
}

pub fn doc_to_vol_info(doc: Document) -> Option<VolInfo> {
    let tracks = get_vol_tracks(&doc);
    match tracks {
        None => return None,
        _ => ()
    };
    let tags = match doc.get_array("tags") {
        Ok(s) => {
            Some(s.into_iter().map(|i| i.as_str().unwrap().to_string()).collect())
        }
        _ => None
    };
    let similar_vols = match doc.get_array("similarVols") {
        Ok(s) => {
            Some(s.into_iter().map(|i| i.as_i32().unwrap().to_owned()).collect())
        }
        _ => None
    };

    Some(
        VolInfo {
            id: get_i32(&doc, "id"),
            vol: get_i32(&doc, "vol"),
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
            tracks: tracks.unwrap(),
        }
    )
}

pub fn doc_to_single(doc: Document) -> Single {
    let id = get_i32(&doc, "id");
    Single {
        id,
        from_id: get_i32(&doc, "fromId"),
        name: get_string(&doc, "name"),
        artist: get_string(&doc, "artist"),
        cover: get_string(&doc, "cover"),
        desc: get_string(&doc, "desc"),
        date: get_i32(&doc, "date"),
        recommender: get_string(&doc, "recommender"),
        url: get_string(&doc, "url"),
        color: get_string(&doc, "color"),
        lyric: get_lyric(LyricType::Single, id)
    }
}

pub fn get_article_tracks(doc: &Document) -> Option<Vec<ArticleTrack>> {
    match doc.get_array("tracks") {
        Ok(i) => {
            Some(
                i
                    .into_iter()
                    .filter_map(|i| {
                        match i.as_document() {
                            Some(c) => {
                                let id = get_i32(c, "id");
                                Some(
                                    ArticleTrack {
                                        id,
                                        article_id: get_i32(c, "articleId"),
                                        name: get_string(c, "name"),
                                        artist: get_string(c, "artist"),
                                        album: get_string(c, "album"),
                                        cover: get_string(c, "cover"),
                                        url: get_string(c, "url"),
                                        color: get_string(c, "color"),
                                        lyric: get_lyric(LyricType::ArticleTrack, id)
                                    }
                                )
                            },
                            None => None
                        }
                    })
                    .collect()
            )
        }
        _ => None
    }
}

enum LyricType {
    VolTrack,
    Single,
    ArticleTrack
}
struct Lyric {
    pub id: i32,
    pub name: String,
    pub artist: String,
    pub lyric: String,
}
fn get_lyric(lyric_type: LyricType, id: i32) -> Option<String> {
    let type_code = match lyric_type {
        LyricType::VolTrack => 0,
        LyricType::Single => 1,
        LyricType::ArticleTrack => 2
    };
    let filter = doc! {
        "id": id,
        "type": type_code
    };

    match LYRIC_COLLECTION.find_one(Some(filter), None).unwrap() {
        None => None,
        Some(doc) => Some(doc_to_lyric(doc).lyric)
    }
}

fn doc_to_lyric(doc: Document) -> Lyric {
    Lyric {
        id: get_i32(&doc, "id"),
        name: get_string(&doc, "name"),
        artist: get_string(&doc, "artist"),
        lyric: get_string(&doc, "lyric")
    }
}

pub fn doc_to_article(doc: Document) -> Option<Article> {
    let tracks = get_article_tracks(&doc);
    match tracks {
        None => return None,
        _ => ()
    };

    Some(
        Article {
            id: get_i32(&doc, "id"),
            title: get_string(&doc, "title"),
            meta_info: get_string(&doc, "metaInfo"),
            cover: get_string(&doc, "cover"),
            url: get_string(&doc, "url"),
            desc: get_string(&doc, "desc"),
            author: get_string(&doc, "author"),
            author_avatar: get_string(&doc, "authorAvatar"),
            color: get_string(&doc, "color"),
            tracks: tracks.unwrap()
        }
    )
}

pub fn get_i32(doc: &Document, key: &str) -> i32 {
    doc.get_i32(key).unwrap().to_owned()
}

pub fn get_string(doc: &Document, key: &str) -> String {
    doc.get_str(key).unwrap().to_owned()
}
