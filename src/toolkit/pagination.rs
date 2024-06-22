use serde::Deserialize;
use thiserror::Error;

static MAX_LIMIT: i64 = 100;
static DEFAULT_LIMIT: i64 = 10;

#[derive(Debug, Error)]
pub enum PaginationError {
    #[error("The limit field cannot be greater than {MAX_LIMIT}")]
    LimitExceeded,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQueries {
    #[serde(default)]
    page: i64,
    #[serde(default)]
    size: i64,
}

impl PaginationQueries {
    pub fn to_sql(self) -> Result<String, PaginationError> {
        if self.size > MAX_LIMIT {
            return Err(PaginationError::LimitExceeded);
        }

        let page = if self.page <= 0 { 1 } else { self.page };
        let size = if self.size <= 0 {
            DEFAULT_LIMIT
        } else {
            self.size
        };

        // Calculate offset
        let limit = size;
        let offset = (page - 1) * limit;
        // Output sql of string type
        Ok(format!(" LIMIT {} OFFSET {}", limit, offset))
    }
}
