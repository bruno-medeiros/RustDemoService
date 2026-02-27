use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::catalog::api::{
    CatalogItem, CreateCatalogItemBody, ListCatalogItemsRequest, ListCatalogItemsResponse,
    UpdateCatalogItemBody,
};

/// In-memory CRUD service for catalog items.
#[derive(Clone, Default)]
pub struct CatalogService {
    store: Arc<RwLock<HashMap<Uuid, CatalogItem>>>,
}

impl CatalogService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new catalog item. Server assigns item_id and timestamps.
    pub async fn create(&self, body: CreateCatalogItemBody) -> CatalogItem {
        let item_id = Uuid::new_v4();
        let now = Utc::now();
        let item = CatalogItem {
            item_id,
            name: body.name,
            description: body.description,
            category: body.category,
            date: body.date,
            brand: body.brand,
            price: body.price,
            created_at: now,
            modified_at: now,
        };
        self.store.write().await.insert(item_id, item.clone());
        item
    }

    /// Get a catalog item by id, if it exists.
    pub async fn get(&self, item_id: Uuid) -> Option<CatalogItem> {
        self.store.read().await.get(&item_id).cloned()
    }

    /// List catalog items with optional pagination (max_results, next_token).
    pub async fn list(&self, req: ListCatalogItemsRequest) -> ListCatalogItemsResponse {
        let store = self.store.read().await;
        let items: Vec<CatalogItem> = store.values().cloned().collect();
        drop(store);

        let max_results = req.max_results.unwrap_or(100).clamp(1, 100) as usize;
        let start = req
            .next_token
            .as_deref()
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let end = (start + max_results).min(items.len());
        let page: Vec<CatalogItem> = items[start..end].to_vec();
        let next_token = if end < items.len() {
            Some(end.to_string())
        } else {
            None
        };

        ListCatalogItemsResponse {
            items: page,
            next_token,
        }
    }

    /// Update a catalog item. Returns the updated item or None if not found.
    pub async fn update(&self, item_id: Uuid, body: UpdateCatalogItemBody) -> Option<CatalogItem> {
        let mut store = self.store.write().await;
        if let Some(item) = store.get_mut(&item_id) {
            item.name = body.name;
            item.description = body.description;
            item.category = body.category;
            item.date = body.date;
            item.brand = body.brand;
            item.price = body.price;
            item.modified_at = Utc::now();
            return Some(item.clone());
        }
        None
    }

    /// Delete a catalog item. Returns true if it existed and was removed.
    pub async fn delete(&self, item_id: Uuid) -> bool {
        self.store.write().await.remove(&item_id).is_some()
    }
}
