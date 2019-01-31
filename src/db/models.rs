
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct VolInfo {
    pub id: i64,
    pub vol: i64,
    pub title: String,
    pub link: String,
    pub cover: String,
    pub color: String,
    pub author: String,
    pub author_avatar: String,
    pub date: String,
    pub desc: String,
    pub tags: Option<Vec<String>>,
    pub similar_vols: Option<Vec<i64>>,
    pub tracks: Vec<VolTrack>
}


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct VolTrack {
    pub id: i64,
    pub vol: i64,
    pub name: String,
    pub artist: String,
    pub album: String,
    pub cover: String,
    pub url: String,
    pub color: String
}
