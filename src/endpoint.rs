//! Define HTTP endpoint handlers here.

use crate::{application::State, schema::BookmarksShowQuery};
use tide::{Request, Result as TideResult};

pub async fn bookmarks_show(request: Request<State>) -> TideResult {
    let query: BookmarksShowQuery = request.query()?;

    todo!();
}
