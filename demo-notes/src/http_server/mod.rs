use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::api::{
    CreateNoteBodyRequest, ListNotesRequest, ListNotesResponse, Note, UpdateNoteBodyRequest,
};
use crate::service::{NotesService, ServiceError};

/// Shared app state: the notes service.
#[derive(Clone)]
pub struct AppState {
    pub notes: NotesService,
}

pub fn router(notes: NotesService) -> Router {
    let state = AppState { notes };
    Router::new()
        .route("/notes", post(create_note).get(list_notes))
        .route("/notes/:id", get(get_note).put(update_note).delete(delete_note))
        .with_state(state)
}

async fn create_note(
    State(state): State<AppState>,
    Json(body): Json<CreateNoteBodyRequest>,
) -> impl IntoResponse {
    match state.notes.create(body).await {
        Ok(note) => (StatusCode::CREATED, Json(note)).into_response(),
        Err(ServiceError::DuplicateAuthorTitle { .. }) => {
            (StatusCode::BAD_REQUEST, "A note with this author and title already exists")
                .into_response()
        }
    }
}

async fn list_notes(
    State(state): State<AppState>,
    Query(_req): Query<ListNotesRequest>,
) -> impl IntoResponse {
    let notes = state.notes.list().await;
    Json(ListNotesResponse { notes })
}

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

async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    if state.notes.delete(id).await {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
