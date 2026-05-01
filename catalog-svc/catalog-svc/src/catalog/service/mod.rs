use std::sync::Arc;

use chrono::{NaiveDate, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::catalog::api::{
    CatalogItem, CreateCatalogItemBody, ListCatalogItemsRequest, ListCatalogItemsResponse,
    UpdateCatalogItemBody,
};
use crate::common::pagination::Pagination;
use crate::catalog::persistence::{CatalogItemRepository, RepositoryError};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Errors that can occur when using [CatalogService].
#[derive(Error, Debug)]
pub enum CatalogServiceError {
    #[error("validation error: {0}")]
    ValidationError(#[source] BoxError),

    #[error("internal error: {0}")]
    InternalError(#[source] BoxError),
}

impl From<RepositoryError> for CatalogServiceError {
    fn from(err: RepositoryError) -> Self {
        CatalogServiceError::InternalError(Box::new(err))
    }
}

/// CRUD service for catalog items, backed by a [CatalogItemRepository].
#[derive(Clone)]
pub struct CatalogService {
    repo: Arc<dyn CatalogItemRepository>,
}

impl CatalogService {
    pub fn new(repo: impl CatalogItemRepository + 'static) -> Self {
        Self {
            repo: Arc::new(repo),
        }
    }

    /// Create a new catalog item. Server assigns item_id and timestamps.
    pub async fn create(
        &self,
        body: CreateCatalogItemBody,
    ) -> Result<CatalogItem, CatalogServiceError> {
        let item_id = Uuid::new_v4();
        let now = Utc::now();
        let date = NaiveDate::parse_from_str(&body.date, "%Y-%m-%d")
            .map_err(|e| CatalogServiceError::ValidationError(Box::new(e)))?;
        let item = CatalogItem {
            item_id,
            name: body.name,
            description: body.description,
            category: body.category,
            date,
            brand: body.brand,
            price: body.price,
            created_at: now,
            modified_at: now,
        };

        self.repo.create(&item).await?;
        Ok(item)
    }

    /// Get a catalog item by id, if it exists.
    pub async fn get(&self, item_id: Uuid) -> Result<Option<CatalogItem>, CatalogServiceError> {
        Ok(self.repo.get(item_id).await?)
    }

    /// List catalog items with optional offset-based pagination.
    pub async fn list(
        &self,
        req: ListCatalogItemsRequest,
    ) -> Result<ListCatalogItemsResponse, CatalogServiceError> {
        let limit = req.limit.unwrap_or(100).clamp(1, 100);
        let offset = req.offset.unwrap_or(0).max(0);

        let search = self
            .repo
            .search(Pagination { limit, offset })
            .await?;
        Ok(ListCatalogItemsResponse::from_paginated(
            search,
            Pagination { limit, offset },
        ))
    }

    /// Update a catalog item. Returns the updated item or None if not found.
    pub async fn update(
        &self,
        item_id: Uuid,
        body: UpdateCatalogItemBody,
    ) -> Result<Option<CatalogItem>, CatalogServiceError> {
        let date = NaiveDate::parse_from_str(&body.date, "%Y-%m-%d")
            .map_err(|e| CatalogServiceError::ValidationError(Box::new(e)))?;

        let existing = self.repo.get(item_id).await?;
        let Some(mut item) = existing else {
            return Ok(None);
        };
        item.name = body.name;
        item.description = body.description;
        item.category = body.category;
        item.date = date;
        item.brand = body.brand;
        item.price = body.price;
        item.modified_at = Utc::now();
        let updated = self.repo.update(&item).await?;
        if updated {
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    /// Delete a catalog item. Returns true if it existed and was removed.
    pub async fn delete(&self, item_id: Uuid) -> Result<bool, CatalogServiceError> {
        Ok(self.repo.delete(item_id).await?)
    }
}
