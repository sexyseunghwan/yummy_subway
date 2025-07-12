use crate::common::*;

use crate::utils::time_utils::*;

use crate::traits::query_service::*;

use crate::models::subway_station::*;

use crate::repository::mysql_repository::*;

use crate::entity::subway_info_tbl::{ActiveModel, Entity};

#[derive(Debug, new)]
pub struct QueryServiceImpl;

impl QueryServiceImpl {

}

#[async_trait]
impl QueryService for QueryServiceImpl {

    async fn input_subway_info(&self, subway_stations: Vec<SubwayStation>) -> Result<(), anyhow::Error> {
        let db: &DatabaseConnection = establish_connection().await;
        
        let txn: DatabaseTransaction = db.begin().await?;

        let cur_time: NaiveDateTime = get_current_utc_naive_datetime();

        for subway in subway_stations {
            
            let lat_decimal: Decimal = Decimal::from_str(&subway.lat)?; 
            let lng_decimal: Decimal = Decimal::from_str(&subway.lng)?; 

            let active_model: ActiveModel = ActiveModel {
                seq: NotSet,
                subway_line: Set(subway.line),
                station_name: Set(subway.name),
                lat: Set(lat_decimal),
                lng: Set(lng_decimal),
                reg_dt: Set(cur_time),
                chg_dt: Set(None),
                reg_id: Set("input_subway_info".to_string()),
                chg_id: Set(None)
            };

            Entity::insert(active_model).exec(&txn).await?;
        }

        txn.commit().await?;

        Ok(())
    }

}
