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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_fields() {
        let config = Config {
            client_id: "test_id".to_string(),
            client_secret: "test_secret".to_string(),
            refresh_token: "test_token".to_string(),
        };
        assert_eq!(config.client_id, "test_id");
        assert_eq!(config.client_secret, "test_secret");
        assert_eq!(config.refresh_token, "test_token");
    }

    #[test]
    fn test_from_env_missing_var() {
        let result = Config::from_env();
        if result.is_err() {
            assert!(matches!(result.unwrap_err(), env::VarError::NotPresent));
        }
    }
}
