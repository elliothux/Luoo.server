#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VolInfo {
    pub id: i32,
    pub vol: i32,
    pub title: String,
    pub link: String,
    pub cover: String,
    pub color: String,
    pub author: String,
    pub author_avatar: String,
    pub date: String,
    pub desc: String,
    pub tags: Option<Vec<String>>,
    pub similar_vols: Option<Vec<i32>>,
    pub tracks: Vec<VolTrack>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VolTrack {
    pub id: i32,
    pub vol_id: i32,
    pub name: String,
    pub artist: String,
    pub album: String,
    pub cover: String,
    pub url: String,
    pub color: String,
    pub lyric: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Single {
    pub id: i32,
    pub from_id: i32,
    pub name: String,
    pub artist: String,
    pub cover: String,
    pub desc: String,
    pub date: i32,
    pub recommender: String,
    pub url: String,
    pub color: String,
    pub lyric: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub meta_info: String,
    pub cover: String,
    pub color: String,
    pub url: String,
    pub desc: String,
    pub author: String,
    pub author_avatar: String,
    pub tracks: Vec<ArticleTrack>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleTrack {
    pub id: i32,
    pub article_id: i32,
    pub name: String,
    pub artist: String,
    pub album: String,
    pub cover: String,
    pub url: String,
    pub color: String,
    pub lyric: Option<String>,
}
