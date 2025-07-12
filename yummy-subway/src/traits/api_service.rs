use crate::common::*;

use crate::models::subway_api_response::*;

#[async_trait]
pub trait ApiService {
    //#[doc = ""]
    //async fn call_seoul_subway_infos(&self) -> Result<SubwayApiResponse, anyhow::Error>;

    #[doc = ""]
    async fn input_seoul_subways_info_to_rdb(&self) -> Result<(), anyhow::Error>;
}