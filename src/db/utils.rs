use bson::{Document};
use mongodb::{Client, ThreadedClient};
use mongodb::{coll::Collection};
use mongodb::db::ThreadedDatabase;
use dotenv::dotenv;
use std::{env, option::Option};
use super::models::{Post, Comment, CommentResponse};



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

pub fn get_comments(doc: Document) -> (i32, Option<Vec<Comment>>) {
    match doc.get_array("comments") {
        Ok(c) => {
            let comments: Vec<Comment> = c.iter()
                .map(|i| {
                    let c = i.as_document().unwrap();
                    Comment {
                        created: c.get_i64("created").unwrap(),
                        username: match c.get_str("username") {
                            Ok(i) => Some(i.to_string()),
                            _ => None
                        },
                        content: c.get_str("content").unwrap().to_string(),
                    }
                })
                .collect();
            (comments.len() as i32, Some(comments))
        }
        _ => (0, None)
    }
}

pub fn count_comments(doc: Document) -> i32 {
    match doc.get_array("comments") {
        Ok(c) => c.len() as i32,
        _ => 0
    }
}

pub fn doc_to_post(post: Option<Document>, with_comments: bool) -> Option<Post> {
    match post {
        Some(doc) => {
            let tags = doc
                .get_array("tags").unwrap().iter()
                .map(|i| i.as_str().unwrap().to_string()).collect();

            return Some(Post {
                id: Some(doc.get_object_id("_id").unwrap().to_hex()),
                created: doc.get_i64("created").unwrap(),
                edited: doc.get_i64("edited").ok(),
                title: doc.get_str("title").unwrap().to_string(),
                content: doc.get_str("content").unwrap().to_string(),
                cover: doc.get_str("cover").unwrap().to_string(),
                tags,
                comments: if with_comments {
                    let (total, data) = get_comments(doc);
                    CommentResponse { total, data }
                } else {
                    CommentResponse { total: count_comments(doc), data: None }
                }
            });
        }
        None => None
    }
}
