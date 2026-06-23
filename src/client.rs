use std::sync::Arc;

use crate::config::Config;
use crate::http::ScientexHttp;

pub use crate::errors::ScientexError;

pub struct ScientexClient {
    pub(crate) http: ScientexHttp,
}

impl ScientexClient {
    pub fn new(config: Arc<Config>) -> Result<Self, ScientexError> {
        Ok(Self {
            http: ScientexHttp::new(config)?,
        })
    }

    pub fn with_token(config: Arc<Config>, token: &str) -> Result<Self, ScientexError> {
        config
            .save_token(token)
            .map_err(|e| ScientexError::ParseError(e.to_string()))?;
        Self::new(config)
    }
}
