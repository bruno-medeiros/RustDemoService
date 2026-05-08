//! Catalog service server handlers and state.

pub mod dtos;
mod errors;

use std::str::FromStr;
use std::sync::Arc;

use crate::server::dtos::{
    map_category_from_smithy, service_item_to_create_output, service_item_to_get_output,
    service_item_to_update_output, service_items_to_smithy_items, uuid_from_smithy,
};
use crate::server::errors::{
    catalog_error_to_create, catalog_error_to_delete, catalog_error_to_get, catalog_error_to_list,
    catalog_error_to_update, dto_internal, not_found_error_404, price_parse_to_validation,
};
use catalog_api::error::{InternalServerError};
use catalog_api::model as smithy;
use catalog_api::server::request::extension::Extension;
use catalog_api::{error, input, output};
use catalog_svc::catalog::api::{
    CreateCatalogItemBody, ListCatalogItemsRequest, ListCatalogItemsResponse, UpdateCatalogItemBody,
};
use catalog_svc::http_server::CatalogApp;
use rust_decimal::Decimal;

type AppState = CatalogApp;

/// Handler for CreateCatalogItem: delegates to the domain CatalogService.
pub async fn create_catalog_item(
    input: input::CreateCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::CreateCatalogItemOutput, error::CreateCatalogItemError> {
    let price = Decimal::from_str(&input.price).map_err(price_parse_to_validation)?;
    let body = CreateCatalogItemBody {
        name: input.name,
        description: input.description,
        category: map_category_from_smithy(input.category),
        date: input.date.to_string(),
        brand: input.brand,
        price,
    };

    let item = state
        .catalog
        .create(body)
        .await
        .map_err(catalog_error_to_create)?;
    Ok(service_item_to_create_output(item))
}

/// Handler for GetCatalogItem: delegates to the domain CatalogService.
pub async fn get_catalog_item(
    input: input::GetCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::GetCatalogItemOutput, error::GetCatalogItemError> {
    let item_id: uuid::Uuid = uuid_from_smithy(input.item_id()).map_err(dto_internal)?;
    let item = state
        .catalog
        .get(item_id)
        .await
        .map_err(catalog_error_to_get)?
        .ok_or_else(not_found_error_404)?;
    Ok(service_item_to_get_output(item))
}

/// Handler for UpdateCatalogItem: delegates to the domain CatalogService.
pub async fn update_catalog_item(
    input: input::UpdateCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::UpdateCatalogItemOutput, error::UpdateCatalogItemError> {
    let item_id: uuid::Uuid = uuid_from_smithy(&input.item_id).map_err(dto_internal)?;

    let price = Decimal::from_str(&input.price).map_err(price_parse_to_validation)?;
    let body = UpdateCatalogItemBody {
        name: input.name,
        description: input.description,
        category: map_category_from_smithy(input.category),
        date: input.date.to_string(),
        brand: input.brand,
        price,
    };

    let item = state
        .catalog
        .update(item_id, body)
        .await
        .map_err(catalog_error_to_update)?
        .ok_or_else(not_found_error_404)?;
    Ok(service_item_to_update_output(item))
}

/// Handler for DeleteCatalogItem: delegates to the domain CatalogService.
pub async fn delete_catalog_item(
    input: input::DeleteCatalogItemInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::DeleteCatalogItemOutput, error::DeleteCatalogItemError> {
    let item_id: uuid::Uuid = uuid_from_smithy(input.item_id()).map_err(dto_internal)?;

    let deleted = state
        .catalog
        .delete(item_id)
        .await
        .map_err(catalog_error_to_delete)?;
    if deleted {
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
        limit: input.limit.map(convert_i64_to_u32).transpose()?,
        offset: input.offset.map(convert_i64_to_u32).transpose()?,
    };

    let ListCatalogItemsResponse {
        items,
        has_more,
        total_count,
        pagination,
        ..
    } = state
        .catalog
        .list(req)
        .await
        .map_err(catalog_error_to_list)?;

    let smithy_items = service_items_to_smithy_items(items);

    Ok(output::ListCatalogItemsOutput {
        items: smithy_items,
        has_more,
        total_count: total_count.map(|c| c.into()),
        pagination: smithy::Pagination {
            limit: pagination.limit.into(),
            offset: pagination.offset.into(),
        },
    })
}

fn convert_i64_to_u32(v: i64) -> Result<u32, InternalServerError> {
    u32::try_from(v).map_err(|err| InternalServerError {
        message: Some(format!("Expect u32 value, but: {err}")),
    })
}
