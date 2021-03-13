use crate::{
    entity::{Bookmark, BookmarkUniqueQuery, Tag, UnregisteredBookmark},
    schema::BookmarkVisibility,
};

use anyhow::Result;
use chrono::prelude::*;
use sqlx::{query, query_as, PgPool};

/// Fetches a bookmark by specified query
pub async fn fetch_bookmark(pool: &PgPool, query: BookmarkUniqueQuery) -> Result<Option<Bookmark>> {
    let query = match query {
        BookmarkUniqueQuery::ById { id, visibility } => {
            let clause = if visibility == BookmarkVisibility::All {
                "SELECT * FROM bookmarks WHERE id = $1;"
            } else {
                "SELECT * FROM bookmarks WHERE id = $1 AND is_private = $2;"
            };
            let is_private = visibility == BookmarkVisibility::Private;

            query_as(clause).bind(id).bind(is_private)
        }

        BookmarkUniqueQuery::ByHash { hash, .. } => {
            // TODO: Store the pribate_key and search with it
            query_as("SELECT * FROM bookmarks WHERE hash = $1;").bind(hash)
        }
        BookmarkUniqueQuery::ByUrl { url } => query_as("SELECT * FROM bookmarks WHERE url = $1;").bind(url),
    };

    let bookmark = query.fetch_optional(pool).await?;
    Ok(bookmark)
}

/// Inserts a new bookmark.
pub async fn insert_bookmark(pool: &PgPool, bookmark: UnregisteredBookmark) -> Result<Bookmark> {
    let now = Local::now();
    let registered = query_as(
        r#"
        INSERT INTO bookmarks (
            hash, url, title, description, thumbnail,
            sticky, private, extra_data, created, updated
        ) VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10
        ) RETURNING *;
        "#,
    )
    .bind(bookmark.hash)
    .bind(bookmark.url)
    .bind(bookmark.title)
    .bind(bookmark.description)
    .bind(bookmark.thumbnail)
    .bind(bookmark.sticky)
    .bind(bookmark.private)
    .bind(bookmark.extra_data)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(registered)
}

/// Updates a bookmark.
/// It assumes that a bookmark with specified ID already exists.
/// If not, nothing will happen.
pub async fn update_bookmark(pool: &PgPool, id: i64, bookmark: UnregisteredBookmark) -> Result<Bookmark> {
    let now = Local::now();
    let registered = query_as(
        r#"
        UPDATE bookmarks
        SET
            hash = $1,
            url = $2,
            title = $3,
            description = $4,
            thumbnail = $5,
            sticky = $6,
            private = $7,
            extra_data = $8,
            updated = $9
        WHERE id = $10
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
    .bind(bookmark.extra_data)
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(registered)
}

/// Deletes a bookmark by ID.
pub async fn delete_bookmark(pool: &PgPool, id: i64) -> Result<()> {
    query("DELETE FROM bookmarks WHERE id = $1;").bind(id).execute(pool).await?;

    Ok(())
}

/// Counts bookmarks by visibility.
pub async fn count_bookmarks_by_visibility(pool: &PgPool, visibility: BookmarkVisibility) -> Result<i64> {
    let clause = if visibility == BookmarkVisibility::All {
        "SELECT COUNT(*) FROM bookmarks;"
    } else {
        "SELECT COUNT(*) FROM bookmarks WHERE is_private = $1;"
    };
    let is_private = visibility == BookmarkVisibility::Private;

    let count: (i64,) = query_as(clause).bind(is_private).fetch_one(pool).await?;
    Ok(count.0)
}

/// Counts bookmarks by its tags and groups with them.
pub async fn count_bookmarks_by_tags(pool: &PgPool, tags: &[impl AsRef<str>], visibility: BookmarkVisibility) -> Result<Vec<(String, i64)>> {
    let query_base = r#"
        SELECT
            tags.id,
            tags.tag,
            COUNT(*) AS count
        FROM bookmarks_tags
        JOIN bookmarks ON bookmarks_tags.bookmark_id = bookmarks.id
        JOIN tags ON bookmarks_tags.tag_id = tags.id
    "#;
    let query_grouping = r#"
        GROUP BY tags.id
        ORDER BY count DESC;
    "#;
    let where_clause = match (tags, visibility) {
        ([], BookmarkVisibility::All) => "",
        (_, BookmarkVisibility::All) => "WHERE tags.tag = ANY($1)",
        ([], _) => "WHERE bookmarks.is_private = $2",
        _ => "WHERE tags.tag = ANY($1) AND bookmarks.is_private = $2",
    };
    let query_str = format!("{} {} {}", query_base, where_clause, query_grouping);

    let tags: Vec<_> = tags.iter().map(|t| t.as_ref()).collect();
    let groups: Vec<(i64, String, i64)> = query_as(&query_str)
        .bind(tags)
        .bind(visibility == BookmarkVisibility::Private)
        .fetch_all(pool)
        .await?;

    let results = groups.into_iter().map(|(_, tag, count)| (tag, count)).collect();
    Ok(results)
}

/// Adds new tags and returns all tag information.
pub async fn add_sync_tags(pool: &PgPool, tags: &[impl AsRef<str>]) -> Result<Vec<Tag>> {
    if !tags.is_empty() {
        let query_str = format!(
            "INSERT INTO tags(tag) VALUES {} ON CONFLICT DO NOTHING;",
            (1..=tags.len()).map(|i| format!("(${})", i)).collect::<Vec<_>>().join(", ")
        );
        let query = tags.iter().fold(query(&query_str), |q, t| q.bind(t.as_ref()));
        query.execute(pool).await?;
    }

    let tags: Vec<_> = tags.iter().map(|t| t.as_ref()).collect();
    let synced_tags = query_as("SELECT * FROM tags WHERE tag = ANY($1);")
        .bind(tags)
        .fetch_all(pool)
        .await?;
    Ok(synced_tags)
}

/// Synchronizes the relation table entries of bookmarks and tags.
pub async fn relate_bookmark_tags(pool: &PgPool, bookmark_id: i64, tag_ids: &[i64]) -> Result<()> {
    query("DELETE FROM bookmarks_tags WHERE bookmark_id = $1;")
        .bind(bookmark_id)
        .execute(pool)
        .await?;

    if !tag_ids.is_empty() {
        let query_str = format!(
            "INSERT INTO bookmarks_tags(bookmark_id, tag_id) VALUES {}",
            (1..=tag_ids.len())
                .map(|i| format!("($1, ${})", i + 1))
                .collect::<Vec<_>>()
                .join(", ")
        );
        let query = tag_ids.iter().fold(query(&query_str).bind(bookmark_id), |q, id| q.bind(id));
        query.execute(pool).await?;
    }

    Ok(())
}

/// Fetch tags of selected bookmarks.
pub async fn fetch_tags_of_bookmarks(pool: &PgPool, bookmark_ids: &[i64]) -> Result<Vec<(i64, String)>> {
    let tags = query_as(
        r#"
        SELECT bookmarks_tags.bookmark_id, tags.tag
        FROM bookmarks_tags
        JOIN tags ON bookmarks_tags.tag_id = tags.id
        WHERE bookmarks_tags.bookmark_id = ANY($1);
        "#,
    )
    .bind(bookmark_ids)
    .fetch_all(pool)
    .await?;

    Ok(tags)
}
