use crate::common::*;

use crate::models::subway_api_response::*;

#[async_trait]
pub trait ApiService {
    #[doc = "Function that can be obtained through API Seoul subway information lines 1 to 8"]
    async fn call_seoul_subway_infos_one_to_eight(&self) -> Result<SubwayApiResponse, anyhow::Error>;
}
