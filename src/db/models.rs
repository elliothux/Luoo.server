
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct VolInfo {
    pub id: u32,
    pub vol: u32,
    pub title: string,
    pub link: string,
    pub cover: string,
    pub color: string,
    pub author: string,
    pub authorAvatar: string,
    pub date: string,
    pub desc: string,
    tags: string[],
    similarVols: number[],
    tracks: VolTrack[]
}
