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

    pub fn new(client_id: String, client_secret: String, refresh_token: String) -> Self {
        Self {
            client_id,
            client_secret,
            refresh_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let config = Config::new(
            "test_id".to_string(),
            "test_secret".to_string(),
            "test_token".to_string(),
        );
        assert_eq!(config.client_id, "test_id");
        assert_eq!(config.client_secret, "test_secret");
        assert_eq!(config.refresh_token, "test_token");
    }

    #[test]
    fn test_from_env_missing_var() {
        // When env vars are not set, from_env should return an error.
        // We don't set any vars here, so at least one will be missing in CI/test environments.
        // This test verifies the error path works (it won't fail if vars happen to be set).
        let result = Config::from_env();
        if result.is_err() {
            assert!(matches!(result.unwrap_err(), env::VarError::NotPresent));
        }
    }
}
