use crate::common::*;

use crate::models::subway_station::*;

#[derive(Debug, Deserialize)]
pub struct SubwayApiResponse {
    pub page: usize,
    pub perPage: usize,
    pub totalCount: usize,
    pub currentCount: usize,
    pub matchCount: usize,
    pub data: Vec<SubwayStation>,
}
