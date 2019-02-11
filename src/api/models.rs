use actix_web::HttpRequest;
use mongodb::coll::Collection;

pub struct AppState {
    pub posts_collection: Collection
}

pub type RequestWithState = HttpRequest<AppState>;

#[derive(Deserialize, Serialize)]
pub struct RetData<T> {
    pub code: u8,
    pub msg: Option<String>,
    pub data: Option<T>,
}
