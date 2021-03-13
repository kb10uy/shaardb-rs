pub mod application;
pub mod database;
pub mod entity;
pub mod schema;

mod endpoint;

use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let env = application::capture_environment()?;
    let state = application::create_state(&env.database_uri).await?;

    let mut app = tide::with_state(state);
    app.with(ServerErrorLogMiddleware);
    app.at("/bookmarks/show").get(endpoint::bookmarks_show);
    app.at("/bookmarks/add").post(endpoint::bookmarks_add);
    app.at("/bookmarks/update").put(endpoint::bookmarks_update);
    app.at("/bookmarks/remove")
        .delete(endpoint::bookmarks_remove);
    app.at("/bookmarks/find");
    app.at("/bookmarks/count").get(endpoint::bookmarks_count);

    app.listen(&env.listen_at).await?;

    Ok(())
}

struct ServerErrorLogMiddleware;

#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> tide::Middleware<State> for ServerErrorLogMiddleware {
    async fn handle(&self, req: tide::Request<State>, next: tide::Next<'_, State>) -> tide::Result {
        let response = next.run(req).await;
        let status = response.status();
        if status.is_server_error() {
            if let Some(e) = response.error() {
                log::error!("{:?}", e);
            }
        }

        Ok(response)
    }
}
