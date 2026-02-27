//! Catalog service server handlers and state.

pub mod dtos;

use std::sync::Arc;

use catalog_api::server::request::extension::Extension;
use catalog_api::{error, input, output};
use catalog_api::{error::NotFoundError};
use catalog_svc::catalog::api::{
    CatalogItem, CreateCatalogItemBody, ListCatalogItemsRequest, ListCatalogItemsResponse,
    UpdateCatalogItemBody,
};
use catalog_svc::catalog::service::CatalogService;

use crate::server::dtos::{
    DtoConversionError, map_category_from_smithy, service_item_to_create_output,
    service_item_to_get_output, service_item_to_update_output, service_items_to_smithy_items,
    uuid_from_smithy,
};

/// Shared application state holding the catalog domain service.
#[derive(Clone)]
pub struct AppState {
    pub catalog: CatalogService,
}

fn not_found_error_404() -> NotFoundError {
    NotFoundError {
        message: Some("Resource not found".into()),
    }
}

/// Handler for CreateCatalogItem: delegates to the domain CatalogService.
pub async fn create_catalog_item(
    input: input::CreateCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::CreateCatalogItemOutput, error::CreateCatalogItemError> {
    let body = CreateCatalogItemBody {
        name: input.name,
        description: input.description,
        category: map_category_from_smithy(input.category),
        date: input.date.to_string(),
        brand: input.brand,
        price: input.price,
    };

    let item: CatalogItem = state.catalog.create(body).await;
    Ok(service_item_to_create_output(item))
}

/// Handler for GetCatalogItem: delegates to the domain CatalogService.
pub async fn get_catalog_item(
    input: input::GetCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::GetCatalogItemOutput, error::GetCatalogItemError> {
    let item_id: uuid::Uuid = uuid_from_smithy(input.item_id()).map_err(dto_internal)?;
    let item = state.catalog.get(item_id).await.ok_or_else(not_found_error_404)?;
    Ok(service_item_to_get_output(item))
}

/// Handler for UpdateCatalogItem: delegates to the domain CatalogService.
pub async fn update_catalog_item(
    input: input::UpdateCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::UpdateCatalogItemOutput, error::UpdateCatalogItemError> {
    let item_id: uuid::Uuid = uuid_from_smithy(&input.item_id).map_err(dto_internal)?;

    let body = UpdateCatalogItemBody {
        name: input.name,
        description: input.description,
        category: map_category_from_smithy(input.category),
        date: input.date.to_string(),
        brand: input.brand,
        price: input.price,
    };

    let item = state.catalog.update(item_id, body).await.ok_or_else(not_found_error_404)?;
    Ok(service_item_to_update_output(item))
}

/// Handler for DeleteCatalogItem: delegates to the domain CatalogService.
pub async fn delete_catalog_item(
    input: input::DeleteCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::DeleteCatalogItemOutput, error::DeleteCatalogItemError> {
    let item_id: uuid::Uuid = uuid_from_smithy(input.item_id()).map_err(dto_internal)?;

    if state.catalog.delete(item_id).await {
        Ok(output::DeleteCatalogItemOutput {})
    } else {
        Err(not_found_error_404().into())
    }
}

/// Handler for ListCatalogItems: delegates to the domain CatalogService.
pub async fn list_catalog_items(
    input: input::ListCatalogItemsInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::ListCatalogItemsOutput, error::ListCatalogItemsError> {
    let req = ListCatalogItemsRequest {
        max_results: input.max_results,
        next_token: input.next_token,
    };

    let ListCatalogItemsResponse { items, next_token } = state.catalog.list(req).await;

    let smithy_items = service_items_to_smithy_items(items);

    Ok(output::ListCatalogItemsOutput {
        items: smithy_items,
        next_token,
    })
}

fn dto_internal(err: DtoConversionError) -> error::InternalServerError {
    error::InternalServerError {
        message: Some(format!("DTO mapping failure: {err}")),
    }
}
