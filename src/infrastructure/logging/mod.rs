// Настройка логирования
use tracing_subscriber::{EnvFilter, fmt};

pub fn init_tracing() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .with_thread_ids(true)
        .init();
}
