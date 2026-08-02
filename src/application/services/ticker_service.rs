use crate::application::dto::TickerDto;
use crate::application::services::ServiceError;
use crate::domain::repositories::TickerRepository;
#[derive(Clone)]
pub struct TickerService<R: TickerRepository> {
    repository: R,
}

impl<R: TickerRepository> TickerService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_all_tickers(&self) -> Result<Vec<TickerDto>, ServiceError> {
        let tickers = self
            .repository
            .find_all()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(tickers.into_iter().map(TickerDto::from).collect())
    }
}
