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

use crate::api::{
    CreateNoteBodyRequest, ListNotesRequest, ListNotesResponse, Note, UpdateNoteBodyRequest,
};
use crate::service::{NotesService, ServiceError};

/// OpenAPI spec for the Notes API (used for client generation and Swagger UI).
#[derive(OpenApi)]
#[openapi(
    paths(create_note, list_notes, get_note, update_note, delete_note,),
    components(schemas(
        Note,
        CreateNoteBodyRequest,
        UpdateNoteBodyRequest,
        ListNotesRequest,
        ListNotesResponse,
    ))
)]
pub struct ApiDoc;

/// Shared app state: the notes service.
#[derive(Clone)]
pub struct AppState {
    pub notes: NotesService,
}

pub fn router(notes: NotesService) -> Router {
    let state = AppState { notes };
    let api = Router::new()
        .route("/notes", post(create_note).get(list_notes))
        .route(
            "/notes/:id",
            get(get_note).put(update_note).delete(delete_note),
        )
        .with_state(state);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api)
}

#[utoipa::path(
    post,
    path = "/notes",
    request_body = CreateNoteBodyRequest,
    responses(
        (status = 201, description = "Note created", body = Note),
        (status = 400, description = "A note with this author and title already exists"),
    )
)]
async fn create_note(
    State(state): State<AppState>,
    Json(body): Json<CreateNoteBodyRequest>,
) -> impl IntoResponse {
    match state.notes.create(body).await {
        Ok(note) => (StatusCode::CREATED, Json(note)).into_response(),
        Err(ServiceError::DuplicateAuthorTitle { .. }) => (
            StatusCode::BAD_REQUEST,
            "A note with this author and title already exists",
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/notes",
    params(ListNotesRequest),
    responses((status = 200, description = "List of notes", body = ListNotesResponse)),
)]
async fn list_notes(
    State(state): State<AppState>,
    Query(_req): Query<ListNotesRequest>,
) -> impl IntoResponse {
    let notes = state.notes.list().await;
    Json(ListNotesResponse { notes })
}

#[utoipa::path(
    get,
    path = "/notes/{id}",
    params(("id" = Uuid, Path, description = "Note ID")),
    responses(
        (status = 200, description = "Note found", body = Note),
        (status = 404, description = "Note not found"),
    )
)]
async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Note>, StatusCode> {
    state
        .notes
        .get(id)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[utoipa::path(
    put,
    path = "/notes/{id}",
    params(("id" = Uuid, Path, description = "Note ID")),
    request_body = UpdateNoteBodyRequest,
    responses(
        (status = 200, description = "Note updated", body = Note),
        (status = 404, description = "Note not found"),
    )
)]
async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateNoteBodyRequest>,
) -> Result<Json<Note>, StatusCode> {
    state
        .notes
        .update(id, body)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[utoipa::path(
    delete,
    path = "/notes/{id}",
    params(("id" = Uuid, Path, description = "Note ID")),
    responses(
        (status = 204, description = "Note deleted"),
        (status = 404, description = "Note not found"),
    )
)]
async fn delete_note(State(state): State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
    if state.notes.delete(id).await {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
