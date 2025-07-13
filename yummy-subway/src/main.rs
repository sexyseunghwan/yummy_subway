mod common;
use common::*;

mod controller;
use controller::api_controller::*;

mod models;

mod service;
use service::{api_service_impl::*, excel_service_impl::*, query_service_impl::*};

mod utils;
use utils::logger_utils::*;

mod entity;

mod traits;

mod repository;

#[tokio::main]
async fn main() {
    dotenv().ok();
    set_global_logger();

    let api_service: ApiServiceImpl = ApiServiceImpl::new();
    let query_service: QueryServiceImpl = QueryServiceImpl::new();
    let excel_service: ExcelServiceImpl = ExcelServiceImpl::new();
    let api_controller: ApiController<ApiServiceImpl, QueryServiceImpl, ExcelServiceImpl> =
        ApiController::new(api_service, query_service, excel_service);

    match api_controller.batch_subways_info_from_excel_to_db().await {
        Ok(_) => (),
        Err(e) => {
            error!("[ERROR][main] {:?}", e);
        }
    }
}
