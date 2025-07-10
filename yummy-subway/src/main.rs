mod common;
use common::*;

mod controller;

mod models;

mod service;

mod utils;
use utils::logger_utils::*;

//mod entity;

#[tokio::main]
async fn main() {
    set_global_logger();
    
    info!("Hello, world!");
    
}
