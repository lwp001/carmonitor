use envconfig::{Envconfig, Error};
use envconfig_derive::Envconfig;

#[derive(Debug, Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "SECRET_KEY")]
    pub secret_key: String,

    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,

    #[envconfig(from = "READABLE_API_URL")]
    pub readable_api_url: String,

    #[envconfig(from = "RSS_JOB_INTERVAL")]
    pub rss_job_interval: u64,

    #[envconfig(from = "DEFAULT_LIMIT")]
    pub default_limit: i64,

    #[envconfig(from = "HOST")]
    pub host: String,

    #[envconfig(from = "PORT")]
    pub port: usize,

    #[envconfig(from = "ASSETS")]
    assets: Option<String>,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        Ok(Self::init()?)
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn assets(&self) -> String {
        match &self.assets {
            Some(assets) => assets.clone(),
            None => "./static/".to_string(),
        }
    }
}
