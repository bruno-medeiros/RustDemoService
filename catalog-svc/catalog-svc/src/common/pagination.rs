//! Offset-based pagination shared across HTTP and persistence layers in this service.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Page window: maximum rows and zero-based starting offset (request body/query or echoed in list responses).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}

/// Paginated search result: item list plus continuation and optional total.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedSearchResponse<T> {
    pub items: Vec<T>,
    pub has_more: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
}

impl<T> PaginatedSearchResponse<T> {
    pub fn new(items: Vec<T>, has_more: bool) -> Self {
        Self {
            items,
            has_more,
            total_count: None,
        }
    }

    pub fn with_total_count(mut self, total: u64) -> Self {
        self.total_count = Some(total);
        self
    }
}
