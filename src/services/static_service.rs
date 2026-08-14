use crate::core::error::AppResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct StaticService {
    cache: Arc<RwLock<HashMap<String, StaticFile>>>,
}

#[derive(Clone)]
pub struct StaticFile {
    pub content: Vec<u8>,
    pub content_type: String,
    pub etag: String,
}

impl StaticService {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn load_file(&self, path: &str, content_type: &str) -> AppResult<StaticFile> {
        {
            let cache = self.cache.read().await;
            if let Some(file) = cache.get(path) {
                return Ok(file.clone());
            }
        }

        let content = tokio::fs::read(path).await?;
        let etag = format!("\"{:x}\"", md5::compute(&content));

        let file = StaticFile {
            content,
            content_type: content_type.to_string(),
            etag,
        };

        {
            let mut cache = self.cache.write().await;
            cache.insert(path.to_string(), file.clone());
        }

        Ok(file)
    }

    pub async fn get_css(&self) -> AppResult<StaticFile> {
        self.load_file("./static/style.css", "text/css; charset=utf-8")
            .await
    }

    pub async fn get_favicon(&self) -> AppResult<StaticFile> {
        self.load_file("./static/favicon.png", "image/png").await
    }
}
