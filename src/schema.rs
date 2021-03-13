use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Represents bookmark visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BookmarkVisibility {
    All,
    Public,
    Private,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: Option<i64>,
    pub hash: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub thumbnail: Option<String>,
    pub tags: Vec<String>,
    pub sticky: bool,
    pub private: bool,
    pub extra_data: Option<JsonValue>,
    pub created: Option<DateTime<Local>>,
    pub updated: Option<DateTime<Local>>,
}

/// Request query parameter of `GET /bookmarks/show`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct BookmarksShowQuery {
    pub hash: Option<String>,
    pub url: Option<String>,
    pub id: Option<i64>,
    pub visibility: Option<BookmarkVisibility>,
    pub private_key: Option<String>,
}

/// Request query parameter of `DELETE /bookmarks/remove`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct BookmarksRemoveQuery {
    pub id: i64,
}

/// Request query parameter of `GET /bookmarks/count`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct BookmarksCountQuery {
    pub visibility: Option<BookmarkVisibility>,
}

/// Response of `GET /bookmarks/count`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BookmarksCountResponse {
    pub visibility: BookmarkVisibility,
    pub count: i64,
}
