use crate::api::models::Symbol;
use crate::core::error::AppResult;
use crate::repositories::SymbolRepository;

pub struct SymbolService<R: SymbolRepository> {
    repo: R,
}

impl<R: SymbolRepository> SymbolService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all_symbols(&self) -> AppResult<Vec<Symbol>> {
        self.repo.get_all_symbols().await.map_err(Into::into)
    }

    pub async fn get_tradeable_symbols(&self) -> AppResult<Vec<Symbol>> {
        self.repo.get_tradeable_symbols().await.map_err(Into::into)
    }

    pub async fn get_symbols_with_index(&self, tradeable: bool) -> AppResult<Vec<(usize, Symbol)>> {
        let symbols = if tradeable {
            self.get_tradeable_symbols().await?
        } else {
            self.get_all_symbols().await?
        };

        Ok(symbols
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i + 1, v))
            .collect())
    }
}
