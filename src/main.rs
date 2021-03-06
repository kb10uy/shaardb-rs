pub mod application;
pub mod database;
pub mod entity;
pub mod schema;

mod endpoint;

use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    let env = application::capture_environment()?;
    let state = application::create_state(&env.database_uri).await?;

    let mut app = tide::with_state(state);
    app.at("/bookmarks/show").get(endpoint::bookmarks_show);
    app.at("/bookmarks/add").post(endpoint::bookmarks_add);
    app.at("/bookmarks/update").put(endpoint::bookmarks_update);
    app.at("/bookmarks/remove").delete(endpoint::bookmarks_remove);

    app.listen(&env.listen_at).await?;

    Ok(())
}
