use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
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
use crate::catalog::service::CatalogService;

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
    let state = AppState { catalog };
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
) -> impl IntoResponse {
    let item = state.catalog.create(body).await;
    (StatusCode::CREATED, Json(item)).into_response()
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
) -> impl IntoResponse {
    let response = state.catalog.list(req).await;
    Json(response)
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
        .await
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
        .await
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
) -> StatusCode {
    if state.catalog.delete(item_id).await {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
