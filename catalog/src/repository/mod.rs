use async_trait::async_trait;
use sqlx::FromRow;
use thiserror::Error;

pub mod artist;

pub struct RepositoryPaginatedResponse<T> {
    items: Vec<T>,
    next_token: Option<String>,
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("unable to create item")]
    ItemCreate,
    #[error("unable to read item")]
    ItemRead,
}

#[async_trait]
pub trait Repository {
    type Item: for<'r> FromRow<'r, sqlx::any::AnyRow>;

    async fn create(&mut self, item: Self::Item) -> Result<Self::Item, RepositoryError>;
    async fn read(&mut self, id: &i64) -> Result<Self::Item, RepositoryError>;
}
