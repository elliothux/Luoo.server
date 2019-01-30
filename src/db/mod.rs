pub mod models;
pub mod utils;

use mongodb::{coll::Collection, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use bson::{to_bson, oid::ObjectId};
use self::models::{NewPost, Post};
use super::utils::get_timestamp;



lazy_static! {
    pub static ref POST_COLLECTION: Collection = utils::get_coll("posts");
}


pub fn get_posts(count: Option<i32>, skip: Option<i64>) -> Vec<Post> {
//    let total = POST_COLLECTION.count(None, None);
    let posts = POST_COLLECTION
        .find(None, None)
        .ok()
        .unwrap();

    let mut result: Vec<Post> = vec![];
    for post in posts {
        match utils::doc_to_post(post.ok(), false) {
            Some(p) => result.push(p),
            None => ()
        };
    };
    result
}

pub fn get_post(id: &str) -> Option<Post> {
    let id = match ObjectId::with_string(id) {
        Ok(i) => Some(doc! { "_id": i }),
        Err(_) => return None
    };
    let post = POST_COLLECTION
        .find_one(id, None)
        .ok()
        .unwrap();
    utils::doc_to_post(post, true)
}

pub fn create_post(post: &NewPost) {
    let doc = doc! {
        "created": get_timestamp(),
        "title": post.title.clone(),
        "tags": to_bson(&post.tags).unwrap(),
        "content": post.content.clone(),
        "cover": post.cover.clone(),
    };

    POST_COLLECTION
        .insert_one(doc, None)
        .ok()
        .expect("Failed to insert document.");
}
