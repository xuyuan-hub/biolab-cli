use std::path::PathBuf;

pub const DEFAULT_BASE_URL: &str = "http://8.136.56.203/api/v1";
const TOKEN_ENV_VAR: &str = "BIOLAB_TOKEN";
const TOKEN_FILE_NAME: &str = ".biolab_token";

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub token_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let base_url =
            std::env::var("BIOLAB_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
        let token_path = token_file_path();
        Self {
            base_url,
            token_path,
        }
    }

    pub fn load_token(&self) -> Option<String> {
        if let Ok(token) = std::env::var(TOKEN_ENV_VAR) {
            return Some(token);
        }
        if self.token_path.exists() {
            if let Ok(token) = std::fs::read_to_string(&self.token_path) {
                let token = token.trim().to_string();
                if !token.is_empty() {
                    return Some(token);
                }
            }
        }
        None
    }

    pub fn save_token(&self, token: &str) -> std::io::Result<()> {
        std::fs::write(&self.token_path, token)
    }

    pub fn remove_token(&self) -> std::io::Result<()> {
        if self.token_path.exists() {
            std::fs::remove_file(&self.token_path)
        } else {
            Ok(())
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

fn token_file_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(TOKEN_FILE_NAME)
}
