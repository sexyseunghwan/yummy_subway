use crate::common::*;

use crate::traits::api_service::*;
use crate::traits::excel_service::*;
use crate::traits::query_service::*;

use crate::models::subway_station_excel::*;

#[derive(Debug, new)]
pub struct ApiController<A: ApiService, Q: QueryService, E: ExcelService> {
    api_service: A,
    query_service: Q,
    excel_service: E,
}

impl<A: ApiService, Q: QueryService, E: ExcelService> ApiController<A, Q, E> {
    #[doc = "Function that stores information on Seoul subway lines 1 to 8 brought in through API in DB."]
    pub async fn batch_seoul_subway_one_to_eight(&self) -> Result<(), anyhow::Error> {
        /* Information on subway stations brought in through API. */
        // let apiRes: SubwayApiResponse = self.api_service.call_seoul_subway_infos_one_to_eight().await?;

        //println!("{:?}", apiRes);

        /* It stores the subway station information imported through API in DB. */

        Ok(())
    }

    #[doc = ""]
    pub async fn batch_subways_info_from_excel_to_db(&self) -> Result<(), anyhow::Error> {
        /* Information on subway stations brought in through EXCEL file. */
        let stations: Vec<SubwayStationExcel> = self
            .excel_service
            .read_excel_sheet_as_struct("./datas/subways.xlsx", "표준데이터 역사(전체)")?;

        /* It stores the subway station information imported through Excel file in DB. */
        self.query_service.input_subway_infos(stations).await?;

        Ok(())
    }
}
