use serde::Deserialize;

use crate::config::Config;

const TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}

pub async fn refresh_access_token(
    client: &reqwest::Client,
    config: &Config,
) -> Result<TokenResponse, reqwest::Error> {
    let params = [
        ("client_id", config.client_id.as_str()),
        ("client_secret", config.client_secret.as_str()),
        ("refresh_token", config.refresh_token.as_str()),
        ("grant_type", "refresh_token"),
    ];

    client
        .post(TOKEN_ENDPOINT)
        .form(&params)
        .send()
        .await?
        .error_for_status()?
        .json::<TokenResponse>()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_response_deserialize() {
        let json = r#"{"access_token": "ya29.abc123", "token_type": "Bearer", "expires_in": 3600}"#;
        let resp: TokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.access_token, "ya29.abc123");
    }

    #[test]
    fn test_token_response_missing_access_token() {
        let json = r#"{"token_type": "Bearer"}"#;
        let result: Result<TokenResponse, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
