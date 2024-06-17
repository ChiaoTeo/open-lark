use crate::core::config::Config;

pub use operate_sheets::*;
mod operate_sheets;


/// 工作表
pub struct SpreadsheetSheetService {
    config: Config,
}

impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
