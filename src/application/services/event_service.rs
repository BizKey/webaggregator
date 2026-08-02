use crate::application::dto::{EventDto, MsgEventDto, MsgSendDto};
use crate::application::services::ServiceError;
use crate::domain::repositories::EventRepository;
#[derive(Clone)]
pub struct EventService<R: EventRepository> {
    repository: R,
}

impl<R: EventRepository> EventService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_events(&self) -> Result<Vec<EventDto>, ServiceError> {
        let events = self
            .repository
            .find_events()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(events.into_iter().map(EventDto::from).collect())
    }

    pub async fn get_msg_events(&self) -> Result<Vec<MsgEventDto>, ServiceError> {
        let events = self
            .repository
            .find_msg_events()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(events.into_iter().map(MsgEventDto::from).collect())
    }

    pub async fn get_msg_sends(&self) -> Result<Vec<MsgSendDto>, ServiceError> {
        let events = self
            .repository
            .find_msg_sends()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        Ok(events.into_iter().map(MsgSendDto::from).collect())
    }
}
