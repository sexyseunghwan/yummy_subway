use crate::common::*;

use crate::traits::api_service::*;

#[derive(Debug, new)]
pub struct ApiController<A: ApiService> {
    api_service: A
}

impl<A: ApiService> ApiController<A> {

    #[doc = ""]
    pub async fn seoul_subway_api(&self) -> Result<(), anyhow::Error> {
        
        Ok(())
    }

}
    