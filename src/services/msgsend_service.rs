use crate::api::models::MsgSend;
use crate::core::error::AppResult;
use crate::repositories::MsgSendRepository;

pub struct MsgSendService<R: MsgSendRepository> {
    repo: R,
}

impl<R: MsgSendRepository> MsgSendService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_msgsends(&self) -> AppResult<Vec<MsgSend>> {
        self.repo.get_msgsends().await.map_err(Into::into)
    }
}
