use anyhow::{Context, Result};
use std::env;

pub fn get_env(key: &str) -> Result<String> {
    env::var(key)
        .map(|value| value.trim().to_string())
        .with_context(|| format!("Don't find ENV:{}", key))
}
