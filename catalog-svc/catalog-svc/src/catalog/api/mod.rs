use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;
use uuid::Uuid;

/// Catalog item category.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, Display, EnumString,
)]
#[serde(rename_all = "PascalCase")]
#[strum(serialize_all = "PascalCase")]
pub enum Category {
    Books,
    Electronics,
}

/// Catalog item: product with id, metadata, and server-set UTC timestamps.
/// exposed in responses only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItem {
    pub item_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: Category,
    /// Date with day resolution only (YYYY-MM-DD).
    pub date: NaiveDate,
    pub brand: Option<String>,
    /// Price (fixed-point decimal, e.g. 19.99). Serializes in JSON as string.
    #[schema(value_type = String, example = "19.99")]
    pub price: Decimal,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

// Request/response types for the REST API (created_at, modified_at not in requests)

/// Body for creating a catalog item (server assigns item_id).
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCatalogItemBody {
    pub name: String,
    pub description: String,
    pub category: Category,
    /// Date with day resolution only (YYYY-MM-DD).
    pub date: String,
    pub brand: Option<String>,
    #[schema(value_type = String, example = "19.99")]
    pub price: Decimal,
}

/// Body for updating a catalog item (same fields as create, except item_id).
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCatalogItemBody {
    pub name: String,
    pub description: String,
    pub category: Category,
    pub date: String,
    pub brand: Option<String>,
    #[schema(value_type = String, example = "19.99")]
    pub price: Decimal,
}

/// Query parameters for the list catalog items endpoint.
#[derive(Debug, Default, Deserialize, ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct ListCatalogItemsRequest {
    /// Maximum number of items to return (page size).
    pub max_results: Option<i32>,
    /// Pagination token from previous response.
    pub next_token: Option<String>,
}

/// Response for the list catalog items endpoint.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListCatalogItemsResponse {
    pub items: Vec<CatalogItem>,
    pub next_token: Option<String>,
}
