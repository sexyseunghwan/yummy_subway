use crate::common::*;

use crate::models::{subway_station::*, subway_station_excel::*};

#[async_trait]
pub trait QueryService {
    #[doc = ""]
    async fn input_subway_info(
        &self,
        subway_stations: Vec<SubwayStation>,
    ) -> Result<(), anyhow::Error>;

    #[doc = ""]
    async fn input_subway_infos(
        &self,
        subway_stations: Vec<SubwayStationExcel>,
    ) -> Result<(), anyhow::Error>;
}
