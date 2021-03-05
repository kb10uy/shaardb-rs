use chrono::prelude::*;
use serde::Deserialize;
use sqlx::prelude::*;

/// Represents bookmark visibility.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookmarkVisibility {
    All,
    Public,
    Private,
}

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
        id: u64,
        visibility: BookmarkVisibility,
    },
}

/// The entity form of a bookmark.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Bookmark {
    pub id: u64,
    pub hash: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub thumbnail: Option<String>,
    pub sticky: bool,
    pub private: bool,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
}

/// The entity form of a tag.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Tag {
    pub id: u64,
    pub tag: String,
}

/// The relation entry between a bookmark and a tag.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct BookmarkTagRelation {
    pub bookmark_id: u64,
    pub tag_id: u64,
}
