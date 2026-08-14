use crate::api::models::Error;
use crate::core::error::AppResult;
use crate::repositories::ErrorRepository;

pub struct ErrorService<R: ErrorRepository> {
    repo: R,
}

impl<R: ErrorRepository> ErrorService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_errors(&self) -> AppResult<Vec<Error>> {
        self.repo.get_errors().await.map_err(Into::into)
    }
}
