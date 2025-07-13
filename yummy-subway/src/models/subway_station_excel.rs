use crate::common::*;

#[derive(Debug, Deserialize)]
pub struct SubwayStationExcel {
    #[serde(rename = "역번호", default)]
    pub code: String,

    #[serde(rename = "역사명", default)]
    pub name: String,

    #[serde(rename = "노선번호", default)]
    pub station_id: String,

    #[serde(rename = "노선명", default)]
    pub line_name: String,

    #[serde(rename = "영문역사명", default)]
    pub name_eng: String,

    #[serde(rename = "한자역사명", default)]
    pub name_hanja: String,

    #[serde(rename = "환승역구분", default)]
    pub station_type: String,

    #[serde(rename = "환승노선번호", default)]
    pub reserved1: Option<String>, // Option이면 default 없어도 OK

    #[serde(rename = "환승노선명", default)]
    pub reserved2: Option<String>,

    #[serde(rename = "역위도", default)]
    pub lat: String,

    #[serde(rename = "역경도", default)]
    pub lng: String,

    #[serde(rename = "운영기관명", default)]
    pub operator: String,

    #[serde(rename = "역사도로명주소", default)]
    pub address: String,

    #[serde(rename = "역사전화번호", default)]
    pub tel: String,

    #[serde(rename = "데이터기준일자", default)]
    pub open_date: String,
}
