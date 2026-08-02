use crate::domain::entities::{Event, MsgEvent, MsgSend};
use crate::domain::repositories::RepositoryError;
use async_trait::async_trait;
#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn find_events(&self) -> Result<Vec<Event>, RepositoryError>;
    async fn find_msg_events(&self) -> Result<Vec<MsgEvent>, RepositoryError>;
    async fn find_msg_sends(&self) -> Result<Vec<MsgSend>, RepositoryError>;
}
