use std::env;

#[derive(Debug)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            client_id: env::var("GOOGLE_BUSINESS_API_CLIENT_ID")?,
            client_secret: env::var("GOOGLE_BUSINESS_API_CLIENT_SECRET")?,
            refresh_token: env::var("GOOGLE_BUSINESS_API_REFRESH_TOKEN")?,
        })
    }
}
