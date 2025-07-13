use crate::common::*;

pub trait ExcelService {
    #[doc = ""]
    fn read_excel_sheet_as_struct<T>(
        &self,
        file_path: &str,
        sheet_name: &str,
    ) -> Result<Vec<T>, anyhow::Error>
    where
        T: DeserializeOwned + Debug;
}
