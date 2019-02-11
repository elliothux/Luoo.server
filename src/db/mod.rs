pub mod models;
pub mod utils;

use bson::{Document};
use mongodb::{coll::Collection, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use bson::{to_bson, oid::ObjectId};
use self::models::{VolInfo, VolTrack, Single};



lazy_static! {
    pub static ref Vol_COLLECTION: Collection = utils::get_coll("vols");
    pub static ref Single_COLLECTION: Collection = utils::get_coll("singles");
}


pub fn get_vols_info(from: u32) -> Vec<VolInfo> {
    let filter = doc! {
        "vol": {
            "$gte": from
        }
    };

    let docs = Vol_COLLECTION
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

    let docs = Single_COLLECTION
        .find(Some(filter), None)
        .ok()
        .unwrap()
        .into_iter();

    docs
        .map(|doc| utils::doc_to_single(doc.ok().unwrap()))
        .collect()
}
