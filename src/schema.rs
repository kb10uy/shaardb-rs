use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents bookmark visibility.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookmarkVisibility {
    All,
    Public,
    Private,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
}

/// Request query parameter of `GET /bookmarks/show`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct BookmarksShowQuery {
    pub hash: Option<String>,
    pub url: Option<String>,
    pub id: Option<i64>,
    pub visibility: Option<BookmarkVisibility>,
    pub private_key: Option<String>,
}
