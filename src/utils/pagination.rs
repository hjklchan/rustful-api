use std::fmt::Display;

use serde::Deserialize;
use thiserror::Error;

/// ### 创建 Trait 名为 PaginatorParts
/// TODO or Deprecated it
/// 
/// ```rust
/// trait PaginatorParts {
///     fn page(self) -> i64;
///     fn size(self) -> i64;
/// }
/// ```
pub struct Introduction;

static MAX_LIMIT: i64 = 100;
static DEFAULT_LIMIT: i64 = 10;

pub trait PaginatorParts {
    fn page(self) -> i64;
    fn size(self) -> i64;
}

#[derive(Debug, Error)]
pub enum PaginationError {
    #[error("The limit field cannot be greater than {MAX_LIMIT}")]
    LimitExceeded,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct PaginationQueries {
    #[serde(default)]
    page: i64,
    #[serde(default)]
    size: i64,
}

pub struct PaginationUtil {
    // 当前多少夜
    page: u64,
    // 当前一夜展示多少条
    size: u64,
    // 总数据量
    total_data_size: u64,
}

impl From<PaginationQueries> for PaginationUtil {
    fn from(value: PaginationQueries) -> Self {
        Self {
            page: value.page as u64,
            size: value.size as u64,
            total_data_size: Default::default(),
        }
    }
}

impl PaginationUtil {
    pub fn set_total_date_size(&mut self, total: u64) {
        self.total_data_size = total
    }

    pub fn get_total_data_size(&self) -> u64 {
        self.total_data_size
    }

    pub fn get_page(&self) -> u64 {
        if self.page <= 0 {
            1
        } else {
            self.page
        }
    }

    pub fn get_size(&self) -> u64 {
        if self.size <= 0 {
            DEFAULT_LIMIT as u64
        } else {
            self.size
        }
    }

    pub fn total_page(&self) -> u64 {
        self.get_total_data_size() / self.get_size()
    }

    pub fn cursors(&self) -> (Option<String>, Option<String>) {
        (
            Some(format!(
                "page={}&size={}",
                self.get_page() - 1,
                self.get_size()
            )),
            if self.get_page() + 1 > self.total_page() {
                None
            } else {
                Some(format!(
                    "page={}&size={}",
                    self.get_page() + 1,
                    self.get_size()
                ))
            },
        )
    }
}

impl PaginationUtil {
    pub fn to_sql<'a>(&self) -> Result<String, PaginationError> {
        if self.size > MAX_LIMIT as u64 {
            return Err(PaginationError::LimitExceeded);
        }

        // Calculate offset
        let limit = self.get_size();
        let offset = (self.get_page() - 1) * limit;

        Ok(format!("LIMIT {} OFFSET {}", limit, offset))
    }
}
