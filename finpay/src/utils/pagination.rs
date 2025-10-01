use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub records: Vec<T>,
    pub page: i64,
    pub per_page: i64,
    pub total_count: i64,
    pub(crate) total_pages: i64,
}

impl<T> PaginatedResponse<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(records: Vec<T>, params: &PaginationParams, total_count: i64) -> Self {
        Self {
            records,
            page: params.page(),
            per_page: params.per_page(),
            total_count,
            total_pages: (total_count / params.per_page()),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
    pub(crate) page: Option<i64>,
    pub(crate) per_page: Option<i64>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1i64),
            per_page: Some(10i64),
        }
    }
}

impl PaginationParams {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1i64)
    }
    pub fn per_page(&self) -> i64 {
        self.per_page.unwrap_or(10i64)
    }
}
