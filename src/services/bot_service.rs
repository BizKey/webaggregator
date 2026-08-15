use crate::api::models::Bot;
use crate::core::error::AppResult;
use crate::repositories::BotRepository;

pub struct BotService<R: BotRepository> {
    repo: R,
}

pub struct BotsWithStats {
    pub bots: Vec<(usize, Bot)>,
    pub init_balance: f64,
    pub final_balance: f64,
}

impl<R: BotRepository> BotService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_bots(&self) -> AppResult<Vec<Bot>> {
        self.repo.get_bots().await.map_err(Into::into)
    }

    pub async fn get_bots_with_stats(&self) -> AppResult<BotsWithStats> {
        let bots = self.get_bots().await?;

        let bots_with_index: Vec<(usize, Bot)> = bots
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i + 1, v))
            .collect();

        let final_balance = bots_with_index
            .iter()
            .filter_map(|(_, bot)| bot.balance.as_ref().and_then(|s| s.parse::<f64>().ok()))
            .sum();

        let init_balance = (20 * bots_with_index.len()) as f64;

        Ok(BotsWithStats {
            bots: bots_with_index,
            init_balance,
            final_balance,
        })
    }
}
