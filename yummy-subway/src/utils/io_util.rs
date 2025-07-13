use crate::common::*;

#[doc = ""]
pub fn parse_optional_decimal(s: &str) -> Option<Decimal> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Decimal::from_str(trimmed).ok()
    }
}