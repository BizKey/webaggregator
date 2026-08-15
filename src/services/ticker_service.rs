use crate::api::models::Ticker;
use crate::core::error::AppResult;
use crate::repositories::TickerRepository;

pub struct TickerService<R: TickerRepository> {
    repo: R,
}

impl<R: TickerRepository> TickerService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_tickers(&self) -> AppResult<Vec<Ticker>> {
        self.repo.get_tickers().await.map_err(Into::into)
    }

    pub async fn get_tickers_with_index(&self) -> AppResult<Vec<(usize, Ticker)>> {
        let tickers = self.get_tickers().await?;
        Ok(tickers
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i + 1, v))
            .collect())
    }
}
