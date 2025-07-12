use crate::common::*;

use crate::traits::api_service::*;
use crate::traits::query_service::*;

use crate::models::subway_api_response::*;

#[derive(Debug, new)]
pub struct ApiServiceImpl<Q: QueryService + Sync + Send> {
    query_service: Q
}

impl<Q: QueryService + Sync + Send> ApiServiceImpl<Q> {

    async fn call_seoul_subway_infos(&self) -> Result<SubwayApiResponse, anyhow::Error> {

        let service_api_url: String = env::var("SEOUL_SUB_API").unwrap_or_else(|e| {
            error!("{:?}", e);
            panic!("{:?}", e);
        });

        let service_api_key: String = env::var("SEOUL_SUB_KEY").unwrap_or_else(|e| {
            error!("{:?}", e);
            panic!("{:?}", e);
        });
        
        let page: i32 = 1;
        let per_page: i32 = 1000;

        let url = format!(
            "{}?page={}&perPage={}&serviceKey={}",
            service_api_url,
            page,
            per_page,
            service_api_key
        );

        let response: SubwayApiResponse = reqwest::get(&url).await?.json::<SubwayApiResponse>().await?;

        Ok(response)
    }

}


#[async_trait]
impl<Q: QueryService + Sync + Send> ApiService for ApiServiceImpl<Q> {

    
    async fn input_seoul_subways_info_to_rdb(&self) -> Result<(), anyhow::Error> {

        

        Ok(())
    }

    


}