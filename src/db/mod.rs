pub mod models;
pub mod utils;

use bson::{Document};
use mongodb::{coll::Collection, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use bson::{to_bson, oid::ObjectId};
use self::models::{VolInfo, VolTrack};



lazy_static! {
    pub static ref Vol_COLLECTION: Collection = utils::get_coll("vols");
}


pub fn get_vols_info(from: u32, to: u32) -> Vec<VolInfo> {
    let filter = doc! {
        "vol": {
            "$gte": from,
            "$lt": to
        }
    };

    let docs = Vol_COLLECTION
        .find(Some(filter), None)
        .ok()
        .unwrap()
        .into_iter();

    println!("3");
    let mut result: Vec<VolInfo> = vec![];
    for doc in docs {
        result.push(utils::doc_to_vol_info(doc.ok().unwrap()))
    };
    result
}
