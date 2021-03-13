//! Define HTTP endpoint handlers here.

use crate::{
    application::State,
    database::{
        add_sync_tags, count_bookmarks_by_visibility, delete_bookmark, fetch_bookmark, fetch_tags_of_bookmarks, insert_bookmark, relate_bookmark_tags,
        update_bookmark,
    },
    entity::{Bookmark as EntityBookmark, BookmarkUniqueQuery, UnregisteredBookmark as EntityUnregisteredBookmark},
    schema::{Bookmark, BookmarkVisibility, BookmarksCountQuery, BookmarksCountResponse, BookmarksRemoveQuery, BookmarksShowQuery},
};

use serde_json::{json, to_value as to_json_value};
use tide::{http::StatusCode, Request, Response, Result as TideResult};

/// Endpoint of `GET /bookmarks/show`.
pub async fn bookmarks_show(request: Request<State>) -> TideResult {
    let state = request.state();
    let query: BookmarksShowQuery = request.query()?;
    let entity_query = if let Some(id) = query.id {
        BookmarkUniqueQuery::ById {
            id,
            visibility: query.visibility.unwrap_or(BookmarkVisibility::All),
        }
    } else if let Some(hash) = query.hash {
        BookmarkUniqueQuery::ByHash {
            hash,
            private_key: query.private_key,
        }
    } else if let Some(url) = query.url {
        BookmarkUniqueQuery::ByUrl { url }
    } else {
        return Ok(Response::builder(StatusCode::BadRequest).body("Query must have one of id/hash/url").build());
    };

    let bookmark_entity = match fetch_bookmark(&state.pool, entity_query).await? {
        Some(bm) => bm,
        None => return Ok(Response::builder(StatusCode::NotFound).body("Bookmark not found").build()),
    };
    let tags_entity = fetch_tags_of_bookmarks(&state.pool, &[bookmark_entity.id]).await?;
    let bookmark = bookmark_from_entity(bookmark_entity, tags_entity.into_iter().map(|(_, t)| t));

    Ok(Response::builder(StatusCode::Ok).body(to_json_value(bookmark)?).build())
}

/// Endpoint of `POST /bookmarks/add`.
pub async fn bookmarks_add(mut request: Request<State>) -> TideResult {
    let body: Bookmark = request.body_json().await?;
    let state = request.state();
    if let Some(_) = body.id {
        return Ok(Response::builder(StatusCode::NotFound).body("New bookmark must not have an ID").build());
    }

    let bookmark_entity = EntityUnregisteredBookmark {
        hash: body.hash,
        url: body.url,
        title: body.title,
        description: body.description,
        thumbnail: body.thumbnail,
        sticky: body.sticky,
        private: body.private,
        extra_data: body.extra_data,
    };
    let tags = body.tags;

    let new_bookmark = insert_bookmark(&state.pool, bookmark_entity).await?;
    let new_tags = add_sync_tags(&state.pool, &tags).await?;
    let tag_ids: Vec<_> = new_tags.iter().map(|t| t.id).collect();
    let tags: Vec<_> = new_tags.into_iter().map(|t| t.tag).collect();
    relate_bookmark_tags(&state.pool, new_bookmark.id, &tag_ids).await?;

    let bookmark = bookmark_from_entity(new_bookmark, tags);
    Ok(Response::builder(StatusCode::Ok).body(to_json_value(bookmark)?).build())
}

/// Endpoint of `PUT /bookmarks/update`.
pub async fn bookmarks_update(mut request: Request<State>) -> TideResult {
    let body: Bookmark = request.body_json().await?;
    let state = request.state();
    let id = match body.id {
        Some(i) => i,
        None => return Ok(Response::builder(StatusCode::BadRequest).body("New bookmark must not have an ID").build()),
    };

    let bookmark_entity = EntityUnregisteredBookmark {
        hash: body.hash,
        url: body.url,
        title: body.title,
        description: body.description,
        thumbnail: body.thumbnail,
        sticky: body.sticky,
        private: body.private,
        extra_data: body.extra_data,
    };
    let tags = body.tags;

    let updated_bookmark = update_bookmark(&state.pool, id, bookmark_entity).await?;
    let updated_tags = add_sync_tags(&state.pool, &tags).await?;
    let tag_ids: Vec<_> = updated_tags.iter().map(|t| t.id).collect();
    let tags: Vec<_> = updated_tags.into_iter().map(|t| t.tag).collect();
    relate_bookmark_tags(&state.pool, updated_bookmark.id, &tag_ids).await?;

    let bookmark = bookmark_from_entity(updated_bookmark, tags);
    Ok(Response::builder(StatusCode::Ok).body(to_json_value(bookmark)?).build())
}

/// Endpoint of `DELETE /bookmarks/remove`.
pub async fn bookmarks_remove(request: Request<State>) -> TideResult {
    let state = request.state();
    let query: BookmarksRemoveQuery = request.query()?;

    delete_bookmark(&state.pool, query.id).await?;
    Ok(Response::builder(StatusCode::Ok).body(json!({})).build())
}

/// Endpoint of `GET /bookmarks/count`.
pub async fn bookmarks_count(request: Request<State>) -> TideResult {
    let state = request.state();
    let query: BookmarksCountQuery = request.query()?;

    let visibility = query.visibility.unwrap_or(BookmarkVisibility::All);
    let count = count_bookmarks_by_visibility(&state.pool, visibility).await?;
    Ok(Response::builder(StatusCode::Ok)
        .body(to_json_value(BookmarksCountResponse { visibility, count })?)
        .build())
}

fn bookmark_from_entity(entity: EntityBookmark, tags: impl IntoIterator<Item = String>) -> Bookmark {
    Bookmark {
        id: Some(entity.id),
        hash: entity.hash,
        url: entity.url,
        title: entity.title,
        description: entity.description,
        thumbnail: entity.thumbnail,
        tags: tags.into_iter().collect(),
        sticky: entity.sticky,
        private: entity.private,
        extra_data: entity.extra_data,
        created: Some(entity.created),
        updated: Some(entity.updated),
    }
}
