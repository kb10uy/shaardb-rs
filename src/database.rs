use crate::{
    entity::{Bookmark, BookmarkUniqueQuery, UnregisteredBookmark},
    schema::BookmarkVisibility,
};

use anyhow::Result;
use chrono::prelude::*;
use sqlx::{query_as, PgPool};

/// Fetches a bookmark by specified query
pub async fn fetch_bookmark(pool: &PgPool, query: BookmarkUniqueQuery) -> Result<Option<Bookmark>> {
    let query = match query {
        BookmarkUniqueQuery::ById { id, visibility } => {
            let clause = if visibility == BookmarkVisibility::All {
                "SELECT * FROM bookmarks WHERE id = ?;"
            } else {
                "SELECT * FROM bookmarks WHERE id = ? AND is_private = ?;"
            };
            let is_private = visibility == BookmarkVisibility::Private;

            query_as(clause).bind(id).bind(is_private)
        }

        BookmarkUniqueQuery::ByHash { hash, .. } => {
            // TODO: Store the pribate_key and search with it
            query_as("SELECT * FROM bookmarks WHERE hash = ?;").bind(hash)
        }
        BookmarkUniqueQuery::ByUrl { url } => {
            query_as("SELECT * FROM bookmarks WHERE url = ?;").bind(url)
        }
    };

    let bookmark = query.fetch_optional(pool).await?;
    Ok(bookmark)
}

/// Inserts a new bookmark.
pub async fn insert_bookmark(pool: &PgPool, bookmark: UnregisteredBookmark) -> Result<Bookmark> {
    let now = Local::now();
    let registered = query_as(
        r#"
        INSERT INTO bookmarks (hash, url, title, description, thumbnail, sticky, private, created, updated)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *;
        "#,
    )
    .bind(bookmark.hash)
    .bind(bookmark.url)
    .bind(bookmark.title)
    .bind(bookmark.description)
    .bind(bookmark.thumbnail)
    .bind(bookmark.sticky)
    .bind(bookmark.private)
    .bind(now)
    .bind(now)
    .fetch_one(pool).await?;

    Ok(registered)
}

/// Updates a bookmark.
/// It assumes that a bookmark with specified id already exists.
/// If not, nothing will happen.
pub async fn update_bookmark(pool: &PgPool, bookmark: Bookmark) -> Result<Bookmark> {
    let now = Local::now();
    let registered = query_as(
        r#"
        UPDATE bookmarks
        SET
            hash = ?,
            url = ?,
            title = ?,
            description = ?,
            thumbnail = ?,
            sticky = ?,
            private = ?,
            updated = ?
        WHERE id = ?
        RETURNING *;
        "#,
    )
    .bind(bookmark.hash)
    .bind(bookmark.url)
    .bind(bookmark.title)
    .bind(bookmark.description)
    .bind(bookmark.thumbnail)
    .bind(bookmark.sticky)
    .bind(bookmark.private)
    .bind(now)
    .bind(bookmark.id)
    .fetch_one(pool)
    .await?;

    Ok(registered)
}

/// Fetch tags of selected bookmarks.
pub async fn fetch_tags_of_bookmarks(
    pool: &PgPool,
    bookmark_ids: &[i64],
) -> Result<Vec<(i64, String)>> {
    let tags = query_as(
        r#"
        SELECT bookmarks_tags.bookmark_id, tags.tag
        FROM bookmarks_tags
        JOIN tags ON bookmarks_tags.tag_id = tags.id
        WHERE bookmarks_tags.id = any(?);
        "#,
    )
    .bind(bookmark_ids)
    .fetch_all(pool)
    .await?;

    Ok(tags)
}
