use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: Option<u64>,
    pub short_url: String,
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
    pub id: Option<u64>,
    pub visibility: Option<u64>,
    pub private_key: Option<u64>,
}
