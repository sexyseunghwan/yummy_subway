use crate::common::*;

use crate::traits::api_service::*;

use crate::models::subway_api_response::*;

#[derive(Debug, new)]
pub struct ApiServiceImpl;

impl ApiServiceImpl { }

#[async_trait]
impl ApiService for ApiServiceImpl {
    async fn call_seoul_subway_infos_one_to_eight(&self) -> Result<SubwayApiResponse, anyhow::Error> {
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

        let url: String = format!(
            "{}?page={}&perPage={}&serviceKey={}",
            service_api_url, page, per_page, service_api_key
        );

        let response: SubwayApiResponse = reqwest::get(&url)
            .await?
            .json::<SubwayApiResponse>()
            .await?;

        Ok(response)
    }
}
