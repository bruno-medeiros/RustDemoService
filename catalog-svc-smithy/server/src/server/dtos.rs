use std::convert::TryFrom;
use std::fmt;

use catalog_api::model::{CatalogItem as SmithyCatalogItem, Category as SmithyCategory, DateOnly};
use catalog_api::output;
use catalog_api::types::DateTime as SmithyDateTime;
use catalog_svc::catalog::api::{CatalogItem as ServiceCatalogItem, Category as ServiceCategory};

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

pub fn map_category_from_smithy(value: SmithyCategory) -> ServiceCategory {
    match value {
        SmithyCategory::Books => ServiceCategory::Books,
        SmithyCategory::Electronics => ServiceCategory::Electronics,
    }
}

pub fn map_category_to_smithy(value: ServiceCategory) -> SmithyCategory {
    match value {
        ServiceCategory::Books => SmithyCategory::Books,
        ServiceCategory::Electronics => SmithyCategory::Electronics,
    }
}

pub fn service_item_to_smithy_item(value: ServiceCatalogItem) -> SmithyCatalogItem {
    let created_at = chrono_to_smithy_datetime(value.created_at);
    let modified_at = chrono_to_smithy_datetime(value.modified_at);

    let item_id = smithy_uuid_from_domain(value.item_id);
    let date = naive_date_to_smithy(value.date);

    SmithyCatalogItem {
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
    value: ServiceCatalogItem,
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
    value: ServiceCatalogItem,
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
    value: ServiceCatalogItem,
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

pub fn service_items_to_smithy_items(items: Vec<ServiceCatalogItem>) -> Vec<SmithyCatalogItem> {
    items.into_iter().map(service_item_to_smithy_item).collect()
}

fn smithy_uuid_from_domain(id: uuid::Uuid) -> catalog_api::model::Uuid {
    catalog_api::model::Uuid::try_from(id.to_string())
        .expect("domain uuid::Uuid should always map to catalog_api::model::Uuid")
}

fn naive_date_to_smithy<D: ToString>(date: D) -> DateOnly {
    let s = date.to_string();
    DateOnly::try_from(s.clone())
        .expect("NaiveDate should always map to DateOnly successfully")
}

fn chrono_to_smithy_datetime(dt: chrono::DateTime<chrono::Utc>) -> SmithyDateTime {
    SmithyDateTime::from_secs(dt.timestamp())
}
