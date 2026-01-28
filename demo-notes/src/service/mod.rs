use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::api::{CreateNoteBodyRequest, Note, UpdateNoteBodyRequest};

/// Service-level errors.
#[derive(Debug, Clone)]
pub enum ServiceError {
    /// A note with the same author and title already exists.
    DuplicateAuthorTitle {
        author: String,
        title: String,
    },
}

/// In-memory CRUD service for notes.
#[derive(Clone, Default)]
pub struct NotesService {
    store: Arc<RwLock<HashMap<Uuid, Note>>>,
}

impl NotesService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new note. Returns the created note, or an error if a note with the same author and title already exists.
    pub async fn create(&self, req: CreateNoteBodyRequest) -> Result<Note, ServiceError> {
        let store = self.store.read().await;
        let duplicate = store
            .values()
            .any(|n| n.author == req.author && n.title == req.title);
        drop(store);

        if duplicate {
            return Err(ServiceError::DuplicateAuthorTitle {
                author: req.author,
                title: req.title,
            });
        }

        let id = Uuid::new_v4();
        let now = Utc::now();
        let note = Note::new(id, req.title, req.author, req.text, now, now);
        self.store.write().await.insert(id, note.clone());
        Ok(note)
    }

    /// Get a note by id, if it exists.
    pub async fn get(&self, id: Uuid) -> Option<Note> {
        self.store.read().await.get(&id).cloned()
    }

    /// List all notes.
    pub async fn list(&self) -> Vec<Note> {
        self.store.read().await.values().cloned().collect()
    }

    /// Update a note's title, author and text. Returns the updated note or None if not found.
    pub async fn update(&self, id: Uuid, req: UpdateNoteBodyRequest) -> Option<Note> {
        let mut store = self.store.write().await;
        if let Some(note) = store.get_mut(&id) {
            note.title = req.title;
            note.author = req.author;
            note.text = req.text;
            note.last_modified_at = Utc::now();
            return Some(note.clone());
        }
        None
    }

    /// Delete a note. Returns true if it existed and was removed.
    pub async fn delete(&self, id: Uuid) -> bool {
        self.store.write().await.remove(&id).is_some()
    }
}
