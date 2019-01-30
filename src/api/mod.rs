pub mod models;
use dotenv::dotenv;
use std::{env, option::Option};
use actix_web::{Json, Result, HttpRequest};
use db::{
    create_post as create_post_db,
    get_posts as get_posts_db,
    get_post as get_post_db,
    models::{NewPost, Post},
};
use self::models::{RetData};
use qiniu;
use super::utils::get_timestamp;


pub fn get_post(req: &HttpRequest) -> Result<Json<RetData<Post>>> {
    let id = req.match_info().get("id").unwrap();
    let post = get_post_db(id);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: post
    };
    Ok(Json(ret))
}

pub fn get_posts(_: &HttpRequest) -> Result<Json<RetData<Vec<Post>>>> {
    let posts = get_posts_db(Some(0), Some(10));
    println!("{:?}", posts);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: Some(posts),
    };
    Ok(Json(ret))
}

pub fn create_post(info: Json<NewPost>) -> Result<Json<RetData<Option<String>>>> {
    let post = NewPost {
        title: info.title.clone(),
        tags: info.tags.clone(),
        content: info.content.clone(),
        cover: info.cover.clone(),
    };
    create_post_db(&post);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: None,
    };
    Ok(Json(ret))
}

pub fn get_upload_token(req: &HttpRequest) -> Result<Json<RetData<String>>> {
    dotenv().ok();
    let access_key = env::var("QINIU_ACCESS_KEY").unwrap();
    let secret_key = env::var("QINIU_SECRET_KEY").unwrap();
    let upload_scope = env::var("QINIU_UPLOAD_SCOPE").unwrap();
    let config = qiniu::Config::new(access_key, secret_key);
    let deadline = get_timestamp() + 60 * 10 * 1000;
    let token = qiniu::PutPolicy::new(upload_scope, deadline as u32)
        .generate_uptoken(&config);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: Some(token)
    };
    Ok(Json(ret))
}
