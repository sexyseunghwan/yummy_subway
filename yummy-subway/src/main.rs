mod common;
use common::*;

mod controller;
use controller::api_controller::*;

mod models;

mod service;
use service::{api_service_impl::*, query_service_impl::*};

mod utils;
use utils::logger_utils::*;

mod entity;

mod traits;

mod repository;


#[tokio::main]
async fn main() {
    dotenv().ok();
    set_global_logger();

    let query_service: QueryServiceImpl = QueryServiceImpl::new();
    let api_service: ApiServiceImpl<QueryServiceImpl> = ApiServiceImpl::new(query_service);
    let api_controller: ApiController<ApiServiceImpl<QueryServiceImpl>> = ApiController::new(api_service);
    
    match api_controller.seoul_subway_api().await {
        Ok(_) => (),
        Err(e) => {
            error!("[ERROR][main] {:?}", e);
        }
    }

    // info!("Hello, world!");  

    // let service_api_url: String = env::var("SEOUL_SUB_API").unwrap();
    // let service_api_key: String = env::var("SEOUL_SUB_KEY").unwrap();

    // let page = 1;
    // let per_page = 1000;


    // let url = format!(
    //     "{}?page={}&perPage={}&serviceKey={}",
    //     service_api_url,
    //     page,
    //     per_page,
    //     service_api_key
    // );

    // // println!("{}", url);

    // let res = reqwest::get(url).await.unwrap();
    // let body = res.text().await.unwrap();
    // println!("{}", body); // 먼저 raw 출력해서 구조 확인


    // 호출할 URL 구성
    // let url = "https://api.odcloud.kr/api/15099316/v1/uddi:cfee6c20-4fee-4c6b-846b-a11c075d0987?page=1&perPage=10&serviceKey=i1dbYYC48j1GW9eINvuy6ttO7Y6klBdjyn9F%2F55tFjzBj48kZirSYD2YP33b%2B2PMF7cFf%2FlDKQgFfdGBuYM%2BRA%3D%3D";


    //let res = reqwest::get(url).await.unwrap();
    //let body = res.text().await.unwrap();
    //println!("{}", body); // 먼저 raw 출력해서 구조 확인


    // let response = reqwest::get(&url).await.unwrap().json::<SubwayApiResponse>().await.unwrap();

    // for station in response.data {
    //     println!(
    //         "[{}호선] {} (위도: {}, 경도: {})",
    //         station.line, station.name, station.lat, station.lng
    //     );
    // }

}
