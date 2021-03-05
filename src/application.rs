use anyhow::Result;
use sqlx::{prelude::*, PgPool};

#[derive(Debug, Clone)]
pub struct State {
    pub pool: PgPool,
}

pub async fn create_state(connection_uri: &str) -> Result<State> {
    let pool = PgPool::connect(connection_uri).await?;

    Ok(State { pool })
}
