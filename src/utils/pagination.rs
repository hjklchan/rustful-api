use serde::Deserialize;
use thiserror::Error;

static MAX_LIMIT: i64 = 100;
static DEFAULT_LIMIT: i64 = 10;

#[derive(Debug, Error)]
pub enum PaginationError {
    #[error("The limit field cannot be greater than {MAX_LIMIT}")]
    LimitExceeded,
}

// TODO: PaginationQueries 应该实现 FromRequest 或者 FromRequestParts
// TODO: PaginationQueries should implements FromRequest or FromRequestParts
// ! 应该通过 PaginationQueries 转为 PaginationUtil 等等...
// !!
// ! 为 PaginationUtil 实现 From<PaginationQueries> 特征
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct PaginationQueries {
    #[serde(default)]
    page: i64,
    #[serde(default)]
    size: i64,
}

impl PaginationQueries {
    pub fn to_sql(&self) -> Result<String, PaginationError> {
        if self.size > MAX_LIMIT {
            return Err(PaginationError::LimitExceeded);
        }

        // Calculate offset
        let limit = self.size();
        let offset = (self.page() - 1) * limit;
        // Output sql of string type
        Ok(format!("LIMIT {} OFFSET {}", limit, offset))
    }
}

impl PaginationQueries {
    pub fn page(self) -> i64 {
        if self.page <= 0 {
            1
        } else {
            self.page
        }
    }

    pub fn size(self) -> i64 {
        if self.size <= 0 {
            DEFAULT_LIMIT
        } else {
            self.size
        }
    }

    pub fn total_page(self, total: i64) -> i64 {
        total / self.size()
    }

    pub fn page_cursors(self, total_page: i64) -> (Option<String>, Option<String>) {
        (
            Some(format!("page={}&size={}", self.page() - 1, self.size())),
            if self.page() + 1 > total_page {
                None
            } else {
                Some(format!("page={}&size={}", self.page() + 1, self.size()))
            },
        )
    }
}
