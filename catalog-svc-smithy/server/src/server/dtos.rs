use std::convert::TryFrom;
use std::fmt;

use catalog_api::model as smithy;
use catalog_api::output;
use catalog_api::types as smithy_types;
use catalog_svc::catalog::api::{CatalogItem, Category};

/// Error type for DTO conversions between smithy `catalog_api` types and `catalog_svc` types.
#[derive(Debug)]
pub enum DtoConversionError {
    InvalidUuid(String),
    InvalidDate(String),
}

impl fmt::Display for DtoConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DtoConversionError::InvalidUuid(v) => write!(f, "invalid UUID: {v}"),
            DtoConversionError::InvalidDate(v) => write!(f, "invalid date: {v}"),
        }
    }
}

impl std::error::Error for DtoConversionError {}

pub fn map_category_from_smithy(value: smithy::Category) -> Category {
    match value {
        smithy::Category::Books => Category::Books,
        smithy::Category::Electronics => Category::Electronics,
    }
}

pub fn map_category_to_smithy(value: Category) -> smithy::Category {
    match value {
        Category::Books => smithy::Category::Books,
        Category::Electronics => smithy::Category::Electronics,
    }
}

pub fn service_item_to_smithy_item(value: CatalogItem) -> smithy::CatalogItem {
    let created_at = chrono_to_smithy_datetime(value.created_at);
    let modified_at = chrono_to_smithy_datetime(value.modified_at);

    let item_id = smithy_uuid_from_domain(value.item_id);
    let date = naive_date_to_smithy(value.date);

    smithy::CatalogItem {
        name: value.name,
        description: value.description,
        category: map_category_to_smithy(value.category),
        date,
        brand: value.brand,
        price: value.price,
        item_id,
        created_at,
        modified_at,
    }
}

pub fn service_item_to_create_output(
    value: CatalogItem,
) -> Result<output::CreateCatalogItemOutput, DtoConversionError> {
    let item = service_item_to_smithy_item(value);
    Ok(output::CreateCatalogItemOutput {
        name: item.name,
        description: item.description,
        category: item.category,
        date: item.date,
        brand: item.brand,
        price: item.price,
        item_id: item.item_id,
        created_at: item.created_at,
        modified_at: item.modified_at,
    })
}

pub fn service_item_to_get_output(
    value: CatalogItem,
) -> Result<output::GetCatalogItemOutput, DtoConversionError> {
    let item = service_item_to_smithy_item(value);
    Ok(output::GetCatalogItemOutput {
        name: item.name,
        description: item.description,
        category: item.category,
        date: item.date,
        brand: item.brand,
        price: item.price,
        item_id: item.item_id,
        created_at: item.created_at,
        modified_at: item.modified_at,
    })
}

pub fn service_item_to_update_output(
    value: CatalogItem,
) -> Result<output::UpdateCatalogItemOutput, DtoConversionError> {
    let item = service_item_to_smithy_item(value);
    Ok(output::UpdateCatalogItemOutput {
        name: item.name,
        description: item.description,
        category: item.category,
        date: item.date,
        brand: item.brand,
        price: item.price,
        item_id: item.item_id,
        created_at: item.created_at,
        modified_at: item.modified_at,
    })
}

pub fn service_items_to_smithy_items(items: Vec<CatalogItem>) -> Vec<smithy::CatalogItem> {
    items.into_iter().map(service_item_to_smithy_item).collect()
}

pub fn uuid_from_smithy(value: &smithy::Uuid) -> Result<uuid::Uuid, DtoConversionError> {
    uuid::Uuid::parse_str(&value.to_string())
        .map_err(|_| DtoConversionError::InvalidUuid(value.to_string()))
}

fn smithy_uuid_from_domain(id: uuid::Uuid) -> smithy::Uuid {
    smithy::Uuid::try_from(id.to_string())
        .expect("domain uuid::Uuid should always map to smithy::Uuid")
}

fn naive_date_to_smithy<D: ToString>(date: D) -> smithy::DateOnly {
    let s = date.to_string();
    smithy::DateOnly::try_from(s.clone())
        .expect("NaiveDate should always map to DateOnly successfully")
}

fn chrono_to_smithy_datetime(dt: chrono::DateTime<chrono::Utc>) -> smithy_types::DateTime {
    smithy_types::DateTime::from_secs(dt.timestamp())
}
