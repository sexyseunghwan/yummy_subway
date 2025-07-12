use crate::common::*;

use crate::models::subway_station::*;

#[async_trait]
pub trait QueryService {
    #[doc = ""]
    async fn input_subway_info(&self, subway_stations: Vec<SubwayStation>) -> Result<(), anyhow::Error>;
}