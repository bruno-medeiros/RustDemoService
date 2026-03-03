//! SQL repository for [CatalogItem] CRUD operations.

use sqlx::{PgPool, Row};
use thiserror::Error;
use uuid::Uuid;

use crate::catalog::api::{CatalogItem, Category};

/// Errors from [CatalogItemRepository] operations.
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("invalid category in row: {0}")]
    InvalidCategory(String),
}

/// SQL repository for catalog items. Handles CRUD against the `catalog_items` table.
#[derive(Clone)]
pub struct CatalogItemRepository {
    pool: PgPool,
}

// TODO: review this

impl CatalogItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Insert a new catalog item.
    pub async fn create(&self, item: &CatalogItem) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO catalog_items (
                item_id,
                name,
                description,
                category,
                date,
                brand,
                price,
                created_at,
                modified_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(item.item_id)
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.category.to_string())
        .bind(item.date)
        .bind(&item.brand)
        .bind(item.price)
        .bind(item.created_at)
        .bind(item.modified_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Fetch a catalog item by id.
    pub async fn get(&self, item_id: Uuid) -> Result<Option<CatalogItem>, RepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT
                item_id,
                name,
                description,
                category,
                date,
                brand,
                price,
                created_at,
                modified_at
            FROM catalog_items
            WHERE item_id = $1
            "#,
        )
        .bind(item_id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(|r| row_to_catalog_item(&r)).transpose()
    }

    /// List catalog items with limit and offset. Returns (items, next_offset) where next_offset is
    /// Some(offset + limit) if there are more rows.
    pub async fn list(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<CatalogItem>, Option<i64>), RepositoryError> {
        let rows = sqlx::query(
            r#"
            SELECT
                item_id,
                name,
                description,
                category,
                date,
                brand,
                price,
                created_at,
                modified_at
            FROM catalog_items
            ORDER BY created_at
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit + 1)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let has_more = rows.len() as i64 > limit;
        let take = if has_more { limit as usize } else { rows.len() };
        let items: Result<Vec<_>, _> = rows.iter().take(take).map(row_to_catalog_item).collect();
        let items = items?;
        let next_offset = if has_more { Some(offset + limit) } else { None };
        Ok((items, next_offset))
    }

    /// Update an existing catalog item. Returns true if a row was updated.
    pub async fn update(&self, item: &CatalogItem) -> Result<bool, RepositoryError> {
        let result = sqlx::query(
            r#"
            UPDATE catalog_items
            SET
                name = $2,
                description = $3,
                category = $4,
                date = $5,
                brand = $6,
                price = $7,
                modified_at = $8
            WHERE item_id = $1
            "#,
        )
        .bind(item.item_id)
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.category.to_string())
        .bind(item.date)
        .bind(&item.brand)
        .bind(item.price)
        .bind(item.modified_at)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Delete a catalog item by id. Returns true if a row was deleted.
    pub async fn delete(&self, item_id: Uuid) -> Result<bool, RepositoryError> {
        let result = sqlx::query(
            r#"
            DELETE FROM catalog_items
            WHERE item_id = $1
            "#,
        )
        .bind(item_id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }
}

fn row_to_catalog_item(row: &sqlx::postgres::PgRow) -> Result<CatalogItem, RepositoryError> {
    let category: String = row.try_get("category")?;
    let category = category
        .parse::<Category>()
        .map_err(|_| RepositoryError::InvalidCategory(category.clone()))?;
    Ok(CatalogItem {
        item_id: row.try_get("item_id")?,
        name: row.try_get("name")?,
        description: row.try_get("description")?,
        category,
        date: row.try_get("date")?,
        brand: row.try_get("brand")?,
        price: row.try_get("price")?,
        created_at: row.try_get("created_at")?,
        modified_at: row.try_get("modified_at")?,
    })
}
