use crate::schema::BookmarkVisibility;

use chrono::prelude::*;
use sqlx::prelude::*;
use serde_json::Value as JsonValue;

/// Query options given from `GET /bookmarks/show`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookmarkUniqueQuery {
    ByHash {
        hash: String,
        private_key: Option<String>,
    },

    ByUrl {
        url: String,
    },

    ById {
        id: i64,
        visibility: BookmarkVisibility,
    },
}

/// The entity form of a new bookmark.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct UnregisteredBookmark {
    pub hash: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub thumbnail: Option<String>,
    pub sticky: bool,
    pub private: bool,
    pub extra_data: Option<JsonValue>,
}

/// The entity form of a bookmark.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Bookmark {
    pub id: i64,
    pub hash: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub thumbnail: Option<String>,
    pub sticky: bool,
    pub private: bool,
    pub extra_data: Option<JsonValue>,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
}

/// The entity form of a tag.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Tag {
    pub id: i64,
    pub tag: String,
}

/// The relation entry between a bookmark and a tag.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct BookmarkTagRelation {
    pub bookmark_id: i64,
    pub tag_id: i64,
}
