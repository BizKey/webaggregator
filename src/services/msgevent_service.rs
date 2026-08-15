use crate::api::models::MsgEvent;
use crate::core::error::AppResult;
use crate::repositories::MsgEventRepository;

pub struct MsgEventService<R: MsgEventRepository> {
    repo: R,
}

impl<R: MsgEventRepository> MsgEventService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_msgevents(&self) -> AppResult<Vec<MsgEvent>> {
        self.repo.get_msgevents().await.map_err(Into::into)
    }
}
