//! Define HTTP endpoint handlers here.

use crate::{
    application::State,
    database::{fetch_bookmark, fetch_tags_of_bookmarks},
    entity::{Bookmark as EntityBookmark, BookmarkUniqueQuery},
    schema::{Bookmark, BookmarkVisibility, BookmarksShowQuery},
};

use serde_json::to_value as to_json_value;
use tide::{http::StatusCode, Error as TideError, Request, Response, Result as TideResult};

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
        return Err(TideError::from_str(
            StatusCode::BadRequest,
            "Query must have one of id/hash/url",
        ));
    };

    let bookmark_entity = fetch_bookmark(&state.pool, entity_query)
        .await?
        .ok_or_else(|| TideError::from_str(StatusCode::NotFound, "Bookmark not found"))?;
    let tags_entity = fetch_tags_of_bookmarks(&state.pool, &[bookmark_entity.id]).await?;
    let bookmark = bookmark_from_entity(bookmark_entity, tags_entity.into_iter().map(|(_, t)| t));

    Ok(Response::builder(StatusCode::Ok)
        .body(to_json_value(bookmark)?)
        .build())
}

pub async fn bookmarks_add(request: Request<State>) -> TideResult {
    todo!();
}

pub async fn bookmarks_update(request: Request<State>) -> TideResult {
    todo!();
}

pub async fn bookmarks_remove(request: Request<State>) -> TideResult {
    todo!();
}

fn bookmark_from_entity(
    entity: EntityBookmark,
    tags: impl IntoIterator<Item = String>,
) -> Bookmark {
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
        created: entity.created,
        updated: entity.updated,
    }
}
