use crate::application::dto::CurrencyDto;
use crate::application::services::ServiceError;
use crate::domain::repositories::CurrencyRepository;
#[derive(Clone)]
pub struct CurrencyService<R: CurrencyRepository> {
    repository: R,
}

impl<R: CurrencyRepository> CurrencyService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_all_currencies(&self) -> Result<Vec<CurrencyDto>, ServiceError> {
        let currencies = self
            .repository
            .find_all()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(currencies.into_iter().map(CurrencyDto::from).collect())
    }
}
