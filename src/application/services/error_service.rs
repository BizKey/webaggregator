use crate::application::dto::ErrorDto;
use crate::application::services::ServiceError;
use crate::domain::repositories::ErrorRepository;
#[derive(Clone)]
pub struct ErrorService<R: ErrorRepository> {
    repository: R,
}

impl<R: ErrorRepository> ErrorService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_all_errors(&self) -> Result<Vec<ErrorDto>, ServiceError> {
        let errors = self
            .repository
            .find_all()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(errors.into_iter().map(ErrorDto::from).collect())
    }
}
