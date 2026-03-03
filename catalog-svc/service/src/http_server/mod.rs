use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

use crate::catalog::api::{
    CatalogItem, CreateCatalogItemBody, ListCatalogItemsRequest, ListCatalogItemsResponse,
    UpdateCatalogItemBody,
};
use crate::catalog::service::{CatalogService, CatalogServiceError};

impl From<CatalogServiceError> for StatusCode {
    fn from(err: CatalogServiceError) -> StatusCode {
        match err {
            CatalogServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            CatalogServiceError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// OpenAPI spec for the Catalog API (used for client generation and Swagger UI).
#[derive(OpenApi)]
#[openapi(
    paths(
        create_catalog_item,
        list_catalog_items,
        get_catalog_item,
        update_catalog_item,
        delete_catalog_item,
    ),
    components(schemas(
        CatalogItem,
        CreateCatalogItemBody,
        UpdateCatalogItemBody,
        ListCatalogItemsRequest,
        ListCatalogItemsResponse,
    ))
)]
pub struct ApiDoc;

/// Shared app state: the catalog service.
#[derive(Clone)]
pub struct AppState {
    pub catalog: CatalogService,
}

pub fn router(catalog: CatalogService) -> Router {
    router_with_state(AppState { catalog })
}

/// Build the API router with the given shared state. Use this when you need to keep a copy of [AppState].
pub fn router_with_state(state: AppState) -> Router {
    let api = Router::new()
        .route(
            "/catalog/items",
            post(create_catalog_item).get(list_catalog_items),
        )
        .route(
            "/catalog/items/{item_id}",
            get(get_catalog_item)
                .post(update_catalog_item)
                .delete(delete_catalog_item),
        )
        .with_state(state);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api)
}

#[utoipa::path(
    post,
    path = "/catalog/items",
    request_body = CreateCatalogItemBody,
    responses(
        (status = 201, description = "Catalog item created", body = CatalogItem),
        (status = 400, description = "Validation error"),
    )
)]
async fn create_catalog_item(
    State(state): State<AppState>,
    Json(body): Json<CreateCatalogItemBody>,
) -> Result<(StatusCode, Json<CatalogItem>), StatusCode> {
    let item = state.catalog.create(body).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

#[utoipa::path(
    get,
    path = "/catalog/items",
    params(ListCatalogItemsRequest),
    responses((status = 200, description = "List of catalog items", body = ListCatalogItemsResponse)),
)]
async fn list_catalog_items(
    State(state): State<AppState>,
    Query(req): Query<ListCatalogItemsRequest>,
) -> Result<Json<ListCatalogItemsResponse>, StatusCode> {
    let response = state.catalog.list(req).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/catalog/items/{item_id}",
    params(("item_id" = Uuid, Path, description = "Catalog item ID")),
    responses(
        (status = 200, description = "Catalog item found", body = CatalogItem),
        (status = 404, description = "Catalog item not found"),
    )
)]
async fn get_catalog_item(
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<Json<CatalogItem>, StatusCode> {
    state
        .catalog
        .get(item_id)
        .await?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[utoipa::path(
    post,
    path = "/catalog/items/{item_id}",
    params(("item_id" = Uuid, Path, description = "Catalog item ID")),
    request_body = UpdateCatalogItemBody,
    responses(
        (status = 200, description = "Catalog item updated", body = CatalogItem),
        (status = 404, description = "Catalog item not found"),
    )
)]
async fn update_catalog_item(
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
    Json(body): Json<UpdateCatalogItemBody>,
) -> Result<Json<CatalogItem>, StatusCode> {
    state
        .catalog
        .update(item_id, body)
        .await?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[utoipa::path(
    delete,
    path = "/catalog/items/{item_id}",
    params(("item_id" = Uuid, Path, description = "Catalog item ID")),
    responses(
        (status = 204, description = "Catalog item deleted"),
        (status = 404, description = "Catalog item not found"),
    )
)]
async fn delete_catalog_item(
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let deleted = state.catalog.delete(item_id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
