use crate::application::dto::BotDto;
use crate::application::services::ServiceError;
use crate::domain::repositories::BotRepository;
#[derive(Clone)]
pub struct BotService<R: BotRepository> {
    repository: R,
}

impl<R: BotRepository> BotService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_all_bots(&self) -> Result<Vec<BotDto>, ServiceError> {
        let bots = self
            .repository
            .find_all()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(bots.into_iter().map(BotDto::from).collect())
    }
}
