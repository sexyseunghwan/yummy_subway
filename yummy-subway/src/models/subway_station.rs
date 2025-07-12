use crate::common::*;


#[derive(Debug, Deserialize)]
pub struct SubwayStation {
    #[serde(rename = "연번")]
    pub seq: u32,
    #[serde(rename = "호선")]
    pub line: u32,
    #[serde(rename = "고유역번호(외부역코드)")]
    pub station_id: u32,
    #[serde(rename = "역명")]
    pub name: String,
    #[serde(rename = "위도")]
    pub lat: String,
    #[serde(rename = "경도")]
    pub lng: String,
    #[serde(rename = "작성일자")]
    pub created_at: String,
}