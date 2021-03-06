use anyhow::Result;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Environments {
    pub listen_at: String,
    pub database_uri: String,
}

#[derive(Debug, Clone)]
pub struct State {
    pub pool: PgPool,
}

pub fn capture_environment() -> Result<Environments> {
    let environment = envy::from_env()?;
    Ok(environment)
}

pub async fn create_state(connection_uri: &str) -> Result<State> {
    let pool = PgPool::connect(connection_uri).await?;

    Ok(State { pool })
}
