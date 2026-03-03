//! Error conversion from domain/DTO errors to Smithy API error types.

use catalog_api::error;
use catalog_svc::catalog::service::CatalogServiceError;

use crate::server::dtos::DtoConversionError;

pub fn dto_internal(err: DtoConversionError) -> error::InternalServerError {
    error::InternalServerError {
        message: Some(format!("DTO mapping failure: {err}")),
    }
}

pub fn not_found_error_404() -> error::NotFoundError {
    error::NotFoundError {
        message: Some("Resource not found".into()),
    }
}

fn catalog_error_to_internal(err: CatalogServiceError) -> error::InternalServerError {
    error::InternalServerError {
        message: Some(err.to_string()),
    }
}

fn catalog_error_to_validation(err: CatalogServiceError) -> error::ValidationException {
    error::ValidationException {
        message: err.to_string(),
        field_list: None,
    }
}

pub fn catalog_error_to_create(err: CatalogServiceError) -> error::CreateCatalogItemError {
    match err {
        CatalogServiceError::ValidationError(_) => catalog_error_to_validation(err).into(),
        CatalogServiceError::InternalError(_) => catalog_error_to_internal(err).into(),
    }
}

pub fn catalog_error_to_get(err: CatalogServiceError) -> error::GetCatalogItemError {
    match err {
        CatalogServiceError::ValidationError(_) => catalog_error_to_validation(err).into(),
        CatalogServiceError::InternalError(_) => catalog_error_to_internal(err).into(),
    }
}

pub fn catalog_error_to_update(err: CatalogServiceError) -> error::UpdateCatalogItemError {
    match err {
        CatalogServiceError::ValidationError(_) => catalog_error_to_validation(err).into(),
        CatalogServiceError::InternalError(_) => catalog_error_to_internal(err).into(),
    }
}

pub fn catalog_error_to_delete(err: CatalogServiceError) -> error::DeleteCatalogItemError {
    match err {
        CatalogServiceError::ValidationError(_) => catalog_error_to_validation(err).into(),
        CatalogServiceError::InternalError(_) => catalog_error_to_internal(err).into(),
    }
}

pub fn catalog_error_to_list(err: CatalogServiceError) -> error::ListCatalogItemsError {
    catalog_error_to_internal(err).into()
}
