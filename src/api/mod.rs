use std::{env, option::Option};

use actix_web::{HttpRequest, Json, Result};
use dotenv::dotenv;

use db::{
    get_singles_info as get_singles_info_db,
    get_vols_info as get_vols_info_db,
    models::{Single, VolInfo, VolTrack},
};

use self::models::RetData;

pub mod models;

pub fn get_vols_info(req: &HttpRequest) -> Result<Json<RetData<Vec<VolInfo>>>> {
    let from = req.match_info()
        .get("from")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let vols = get_vols_info_db(from);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: Some(vols),
    };
    Ok(Json(ret))
}

pub fn get_singles_info(req: &HttpRequest) -> Result<Json<RetData<Vec<Single>>>> {
    let from_id = req.match_info()
        .get("from_id")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let singles = get_singles_info_db(from_id);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: Some(singles),
    };
    Ok(Json(ret))
}
