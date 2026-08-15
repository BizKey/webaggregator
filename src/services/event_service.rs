use crate::api::models::Event;
use crate::core::error::AppResult;
use crate::repositories::EventRepository;

pub struct EventService<R: EventRepository> {
    repo: R,
}

impl<R: EventRepository> EventService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_events(&self) -> AppResult<Vec<Event>> {
        self.repo.get_events().await.map_err(Into::into)
    }
}
