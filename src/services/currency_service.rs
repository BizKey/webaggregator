use crate::api::models::Currency;
use crate::core::error::AppResult;
use crate::repositories::CurrencyRepository;

pub struct CurrencyService<R: CurrencyRepository> {
    repo: R,
}

impl<R: CurrencyRepository> CurrencyService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_currencies(&self) -> AppResult<Vec<Currency>> {
        self.repo.get_currencies().await.map_err(Into::into)
    }
}
