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
    service_item_to_create_output(item).map_err(internal_error_for_create)
}

/// Handler for GetCatalogItem: delegates to the domain CatalogService.
pub async fn get_catalog_item(
    input: input::GetCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::GetCatalogItemOutput, error::GetCatalogItemError> {
    let item_id: uuid::Uuid = uuid_from_smithy(input.item_id()).map_err(internal_error_for_get)?;

    match state.catalog.get(item_id).await {
        Some(item) => service_item_to_get_output(item).map_err(internal_error_for_get),
        None => Err(error::GetCatalogItemError::from(not_found_error_404())),
    }
}

/// Handler for UpdateCatalogItem: delegates to the domain CatalogService.
pub async fn update_catalog_item(
    input: input::UpdateCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::UpdateCatalogItemOutput, error::UpdateCatalogItemError> {
    let item_id: uuid::Uuid =
        uuid_from_smithy(&input.item_id).map_err(internal_error_for_update)?;

    let body = UpdateCatalogItemBody {
        name: input.name,
        description: input.description,
        category: map_category_from_smithy(input.category),
        date: input.date.to_string(),
        brand: input.brand,
        price: input.price,
    };

    match state.catalog.update(item_id, body).await {
        Some(item) => service_item_to_update_output(item).map_err(internal_error_for_update),
        None => Err(error::UpdateCatalogItemError::from(not_found_error_404())),
    }
}

/// Handler for DeleteCatalogItem: delegates to the domain CatalogService.
pub async fn delete_catalog_item(
    input: input::DeleteCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::DeleteCatalogItemOutput, error::DeleteCatalogItemError> {
    let item_id: uuid::Uuid =
        uuid_from_smithy(input.item_id()).map_err(internal_error_for_delete)?;

    if state.catalog.delete(item_id).await {
        Ok(output::DeleteCatalogItemOutput {})
    } else {
        Err(error::DeleteCatalogItemError::from(not_found_error_404()))
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

fn internal_error_for_create(err: DtoConversionError) -> error::CreateCatalogItemError {
    error::CreateCatalogItemError::from(error::InternalServerError {
        message: Some(format!("DTO mapping failure: {err}")),
    })
}

fn internal_error_for_get(err: DtoConversionError) -> error::GetCatalogItemError {
    error::GetCatalogItemError::from(error::InternalServerError {
        message: Some(format!("DTO mapping failure: {err}")),
    })
}

fn internal_error_for_update(err: DtoConversionError) -> error::UpdateCatalogItemError {
    error::UpdateCatalogItemError::from(error::InternalServerError {
        message: Some(format!("DTO mapping failure: {err}")),
    })
}

fn internal_error_for_delete(err: DtoConversionError) -> error::DeleteCatalogItemError {
    error::DeleteCatalogItemError::from(error::InternalServerError {
        message: Some(format!("DTO mapping failure: {err}")),
    })
}
