use sqlx::prelude::*;
use chrono::prelude::*;

/// The entity form of a bookmark.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Bookmark {
    pub id: u64,
    pub short_url: String,
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
