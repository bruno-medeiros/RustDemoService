use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A note: a text blob identified by an id, with title and author.
/// created_at and last_modified_at are set by the service; exposed in responses only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub last_modified_at: DateTime<Utc>,
}

impl Note {
    pub fn new(
        id: Uuid,
        title: String,
        author: String,
        text: String,
        created_at: DateTime<Utc>,
        last_modified_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            title,
            author,
            text,
            created_at,
            last_modified_at,
        }
    }
}

// Request/response types for the REST API (not used in requests: created_at, last_modified_at)

#[derive(Deserialize)]
pub struct CreateNoteBodyRequest {
    pub title: String,
    pub author: String,
    pub text: String,
}

#[derive(Deserialize)]
pub struct UpdateNoteBodyRequest {
    pub title: String,
    pub author: String,
    pub text: String,
}

/// Query parameters for the list notes endpoint.
#[derive(Debug, Default, Deserialize)]
pub struct ListNotesRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Response body for the list notes endpoint.
#[derive(Serialize)]
pub struct ListNotesResponse {
    pub notes: Vec<Note>,
}
