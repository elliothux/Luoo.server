pub mod models;
use dotenv::dotenv;
use std::{env, option::Option};
use actix_web::{Json, Result, HttpRequest};
use db::{
    get_vols_info as get_vols_info_db,
    models::{VolInfo, VolTrack},
};
use self::models::{RetData};


pub fn get_vols_info(req: &HttpRequest) -> Result<Json<RetData<Vec<VolInfo>>>> {
    let from = req.match_info()
        .get("from")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let to = req.match_info()
        .get("to")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let vols = get_vols_info_db(from, to);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: Some(vols),
    };
    Ok(Json(ret))
}
