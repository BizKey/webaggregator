use crate::application::dto::SymbolDto;
use crate::application::services::ServiceError;
use crate::domain::repositories::SymbolRepository;
#[derive(Clone)]
pub struct SymbolService<R: SymbolRepository> {
    repository: R,
}

impl<R: SymbolRepository> SymbolService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_all_symbols(&self) -> Result<Vec<SymbolDto>, ServiceError> {
        let symbols = self
            .repository
            .find_all()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(symbols.into_iter().map(SymbolDto::from).collect())
    }
}
