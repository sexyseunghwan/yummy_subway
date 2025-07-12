use crate::common::*;

use crate::traits::api_service::*;
use crate::traits::query_service::*;

use crate::models::subway_api_response::*;

#[derive(Debug, new)]
pub struct ApiController<A: ApiService, Q: QueryService> {
    api_service: A,
    query_service: Q,
}

impl<A: ApiService, Q: QueryService> ApiController<A, Q> {
    #[doc = "Function that stores information on Seoul subway lines 1 to 8 brought in through API in DB."]
    pub async fn batch_seoul_subway_one_to_eight(&self) -> Result<(), anyhow::Error> {
        
        /* Information on subway stations brought in through API. */
        let apiRes: SubwayApiResponse = self.api_service.call_seoul_subway_infos_one_to_eight().await?;
        
        println!("{:?}", apiRes);

        /* It stores the subway station information imported through API in DB. */
        

        
        Ok(())
    }
}
