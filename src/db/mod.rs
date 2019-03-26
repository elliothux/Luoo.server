use bson::Document;
use bson::{oid::ObjectId, to_bson};
use mongodb::{coll::Collection, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use self::models::{Single, VolInfo, VolTrack, Article};

pub mod models;
pub mod utils;

lazy_static! {
    pub static ref VOL_COLLECTION: Collection = utils::get_coll("vols");
    pub static ref SINGLE_COLLECTION: Collection = utils::get_coll("singles");
    pub static ref ARTICLE_COLLECTION: Collection = utils::get_coll("articles");
}


pub fn get_vols_info(from_id: u32) -> Vec<VolInfo> {
    let filter = doc! {
        "id": {
            "$gte": from_id
        }
    };

    let docs = VOL_COLLECTION
        .find(Some(filter), None)
        .ok()
        .unwrap()
        .into_iter();

    let mut result: Vec<VolInfo> = vec![];
    for doc in docs {
        if let Some(v) = utils::doc_to_vol_info(doc.ok().unwrap()) {
            result.push(v);
        }
    };
    result
}

pub fn get_singles_info(from_id: u32) -> Vec<Single> {
    let filter = doc! {
        "id": {
            "$gte": from_id
        }
    };

    let docs = SINGLE_COLLECTION
        .find(Some(filter), None)
        .ok()
        .unwrap()
        .into_iter();

    docs
        .map(|doc| utils::doc_to_single(doc.ok().unwrap()))
        .collect()
}

pub fn get_articles_info(from_id: u32) -> Vec<Article> {
    let filter = doc! {
        "id": {
            "$gte": from_id
        }
    };

    let docs = ARTICLE_COLLECTION
        .find(Some(filter), None)
        .ok()
        .unwrap()
        .into_iter();

    let mut result: Vec<Article> = vec![];
    for doc in docs {
        if let Some(v) = utils::doc_to_article(doc.ok().unwrap()) {
            result.push(v);
        }
    };
    result
}
