use crate::common::*;

use crate::traits::excel_service::*;

#[derive(Debug, new)]
pub struct ExcelServiceImpl;

impl ExcelService for ExcelServiceImpl {
    fn read_excel_sheet_as_struct<T>(
        &self,
        file_path: &str,
        sheet_name: &str,
    ) -> Result<Vec<T>, anyhow::Error>
    where
        T: DeserializeOwned + Debug,
    {
        let mut workbook: calamine::Sheets<BufReader<File>> = open_workbook_auto(file_path)?;

        let range: calamine::Range<calamine::Data> = workbook.worksheet_range(sheet_name)?;

        let iter = RangeDeserializerBuilder::new()
            .has_headers(true) /* Consider the first line as a header */
            .from_range::<_, T>(&range)?;

        let mut result: Vec<T> = Vec::new();

        for row in iter {
            let row_res: T = match row {
                Ok(row_res) => {
                    row_res
                },
                Err(e) => {
                    error!("[Error][ExcelService->read_excel_sheet_as_struct]{:?}", e);
                    continue;
                }
            };

            result.push(row_res);
        }

        Ok(result)
    }
}
