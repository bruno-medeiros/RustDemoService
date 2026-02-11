//! Catalog service server handlers and state.

pub mod dtos;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

use catalog_api::model::{CatalogItem, Uuid};
use catalog_api::server::request::extension::Extension;
use catalog_api::{error, input, output};
use catalog_api::{error::ValidationException, types::DateTime};

use crate::server::dtos::{shape_to_create_output, shape_to_get_output, shape_to_update_output};

/// Shared application state holding catalog items by id.
#[derive(Debug, Default)]
pub struct AppState {
    pub items: RwLock<HashMap<Uuid, CatalogItem>>,
}

fn not_found_error() -> ValidationException {
    ValidationException::builder()
        .message("Resource not found".into())
        .build()
        .expect("ValidationException builder")
}

fn now() -> DateTime {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    DateTime::from_secs(secs)
}

/// Handler for CreateCatalogItem: stores a new item and returns it.
pub async fn create_catalog_item(
    input: input::CreateCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::CreateCatalogItemOutput, error::CreateCatalogItemError> {
    let item_id = catalog_api::model::Uuid::try_from(uuid::Uuid::new_v4().to_string())
        .expect("UUID v4 satisfies Uuid pattern");
    let now = now();
    let item = CatalogItem {
        name: input.name,
        description: input.description,
        category: input.category,
        date: input.date,
        brand: input.brand,
        price: input.price,
        item_id: item_id.clone(),
        created_at: now,
        modified_at: now,
    };
    state.items.write().unwrap().insert(item_id, item.clone());
    Ok(shape_to_create_output(item))
}

/// Handler for GetCatalogItem: returns the item by id.
pub async fn get_catalog_item(
    input: input::GetCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::GetCatalogItemOutput, error::GetCatalogItemError> {
    let guard = state.items.read().unwrap();
    let item = guard
        .get(input.item_id())
        .cloned()
        .ok_or_else(|| error::GetCatalogItemError::from(not_found_error()))?;
    drop(guard);
    Ok(shape_to_get_output(item))
}

/// Handler for UpdateCatalogItem: updates an existing item.
pub async fn update_catalog_item(
    input: input::UpdateCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::UpdateCatalogItemOutput, error::UpdateCatalogItemError> {
    let item_id = input.item_id.clone();
    let mut guard = state.items.write().unwrap();
    let item = guard
        .get_mut(&item_id)
        .ok_or_else(|| error::UpdateCatalogItemError::from(not_found_error()))?;
    let now = now();
    item.name = input.name;
    item.description = input.description;
    item.category = input.category;
    item.date = input.date;
    item.brand = input.brand;
    item.price = input.price;
    item.modified_at = now;
    let item = item.clone();
    drop(guard);
    Ok(shape_to_update_output(item))
}

/// Handler for DeleteCatalogItem: removes the item.
pub async fn delete_catalog_item(
    input: input::DeleteCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::DeleteCatalogItemOutput, error::DeleteCatalogItemError> {
    let removed = state.items.write().unwrap().remove(input.item_id());
    if removed.is_some() {
        Ok(output::DeleteCatalogItemOutput {})
    } else {
        Err(error::DeleteCatalogItemError::from(not_found_error()))
    }
}

/// Handler for ListCatalogItems: returns a page of items with optional pagination.
pub async fn list_catalog_items(
    input: input::ListCatalogItemsInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::ListCatalogItemsOutput, error::ListCatalogItemsError> {
    let guard = state.items.read().unwrap();
    let items: Vec<CatalogItem> = guard.values().cloned().collect();
    drop(guard);

    let max_results = input.max_results.unwrap_or(100).clamp(1, 100) as usize;
    let start = input
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

    Ok(output::ListCatalogItemsOutput {
        items: page,
        next_token,
    })
}
