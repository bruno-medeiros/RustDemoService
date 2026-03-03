use std::collections::HashMap;
use std::sync::Arc;

use chrono::{NaiveDate, Utc};
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::catalog::api::{
    CatalogItem, CreateCatalogItemBody, ListCatalogItemsRequest, ListCatalogItemsResponse,
    UpdateCatalogItemBody,
};
use crate::catalog::repository::{CatalogItemRepository, RepositoryError};

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

// TODO: change to trait
#[derive(Clone)]
enum CatalogBackend {
    Memory(Arc<RwLock<HashMap<Uuid, CatalogItem>>>),
    Sql(CatalogItemRepository),
}

/// CRUD service for catalog items. Uses either an in-memory store or a SQL repository.
#[derive(Clone)]
pub struct CatalogService {
    backend: CatalogBackend,
}

impl Default for CatalogService {
    fn default() -> Self {
        Self {
            backend: CatalogBackend::Memory(Arc::new(RwLock::new(HashMap::new()))),
        }
    }
}

impl CatalogService {
    /// New in-memory catalog service (e.g. for tests).
    pub fn new() -> Self {
        Self::default()
    }

    /// New catalog service backed by the SQL repository.
    pub fn with_repository(repo: CatalogItemRepository) -> Self {
        Self {
            backend: CatalogBackend::Sql(repo),
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

        match &self.backend {
            CatalogBackend::Memory(store) => {
                store.write().await.insert(item_id, item.clone());
            }
            CatalogBackend::Sql(repo) => {
                repo.create(&item).await?;
            }
        }
        Ok(item)
    }

    /// Get a catalog item by id, if it exists.
    pub async fn get(&self, item_id: Uuid) -> Result<Option<CatalogItem>, CatalogServiceError> {
        match &self.backend {
            CatalogBackend::Memory(store) => Ok(store.read().await.get(&item_id).cloned()),
            CatalogBackend::Sql(repo) => Ok(repo.get(item_id).await?),
        }
    }

    /// List catalog items with optional pagination (max_results, next_token).
    pub async fn list(
        &self,
        req: ListCatalogItemsRequest,
    ) -> Result<ListCatalogItemsResponse, CatalogServiceError> {
        let max_results = req.max_results.unwrap_or(100).clamp(1, 100) as i64;
        let offset = req
            .next_token
            .as_deref()
            .and_then(|t| t.parse::<i64>().ok())
            .unwrap_or(0);

        match &self.backend {
            CatalogBackend::Memory(store) => {
                let items: Vec<CatalogItem> = store.read().await.values().cloned().collect();
                let start = offset as usize;
                let end = (start + max_results as usize).min(items.len());
                let page: Vec<CatalogItem> = items[start..end].to_vec();
                let next_token = if end < items.len() {
                    Some(end.to_string())
                } else {
                    None
                };
                Ok(ListCatalogItemsResponse {
                    items: page,
                    next_token,
                })
            }
            CatalogBackend::Sql(repo) => {
                let (items, next_offset) = repo.list(max_results, offset).await?;
                let next_token = next_offset.map(|o| o.to_string());
                Ok(ListCatalogItemsResponse { items, next_token })
            }
        }
    }

    /// Update a catalog item. Returns the updated item or None if not found.
    pub async fn update(
        &self,
        item_id: Uuid,
        body: UpdateCatalogItemBody,
    ) -> Result<Option<CatalogItem>, CatalogServiceError> {
        let date = NaiveDate::parse_from_str(&body.date, "%Y-%m-%d")
            .map_err(|e| CatalogServiceError::ValidationError(Box::new(e)))?;

        match &self.backend {
            CatalogBackend::Memory(store) => {
                let mut guard = store.write().await;
                if let Some(item) = guard.get_mut(&item_id) {
                    item.name = body.name;
                    item.description = body.description;
                    item.category = body.category;
                    item.date = date;
                    item.brand = body.brand;
                    item.price = body.price;
                    item.modified_at = Utc::now();
                    return Ok(Some(item.clone()));
                }
                Ok(None)
            }
            CatalogBackend::Sql(repo) => {
                let existing = repo.get(item_id).await?;
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
                let updated = repo.update(&item).await?;
                if updated {
                    Ok(Some(item))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Delete a catalog item. Returns true if it existed and was removed.
    pub async fn delete(&self, item_id: Uuid) -> Result<bool, CatalogServiceError> {
        match &self.backend {
            CatalogBackend::Memory(store) => Ok(store.write().await.remove(&item_id).is_some()),
            CatalogBackend::Sql(repo) => Ok(repo.delete(item_id).await?),
        }
    }
}
