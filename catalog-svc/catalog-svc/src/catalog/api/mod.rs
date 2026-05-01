use std::error::Error as StdError;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use thiserror::Error;
use crate::common::pagination::{PaginatedSearchResponse, Pagination};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;
use uuid::Uuid;

type BoxError = Box<dyn StdError + Send + Sync>;

/// Errors that can occur when using [CatalogServiceApi] / [crate::catalog::service::CatalogService].
#[derive(Error, Debug)]
pub enum CatalogServiceError {
    #[error("validation error: {0}")]
    ValidationError(#[source] BoxError),

    #[error("internal error: {0}")]
    InternalError(#[source] BoxError),
}

/// HTTP-exposed catalog operations implemented by [crate::catalog::service::CatalogService].
#[async_trait]
pub trait CatalogServiceApi: Send + Sync {
    async fn create(
        &self,
        body: CreateCatalogItemBody,
    ) -> Result<CatalogItem, CatalogServiceError>;

    async fn get(&self, item_id: Uuid) -> Result<Option<CatalogItem>, CatalogServiceError>;

    async fn list(
        &self,
        req: ListCatalogItemsRequest,
    ) -> Result<ListCatalogItemsResponse, CatalogServiceError>;

    async fn update(
        &self,
        item_id: Uuid,
        body: UpdateCatalogItemBody,
    ) -> Result<Option<CatalogItem>, CatalogServiceError>;

    async fn delete(&self, item_id: Uuid) -> Result<bool, CatalogServiceError>;
}

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
    /// Maximum number of items to return (page size). Defaults to 100; clamped server-side.
    pub limit: Option<i64>,
    /// Zero-based offset into the result set. Defaults to 0.
    pub offset: Option<i64>,
}

/// Response for the list catalog items endpoint.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListCatalogItemsResponse {
    pub items: Vec<CatalogItem>,
    pub has_more: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    pub pagination: Pagination,
}

impl ListCatalogItemsResponse {
    pub(crate) fn from_paginated(
        page: PaginatedSearchResponse<CatalogItem>,
        pagination: Pagination,
    ) -> Self {
        let PaginatedSearchResponse {
            items,
            has_more,
            total_count,
        } = page;
        Self {
            items,
            has_more,
            total_count,
            pagination,
        }
    }
}
