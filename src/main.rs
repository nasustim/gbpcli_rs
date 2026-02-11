mod auth;
mod config;
mod repository;

use crate::auth::TokenResponse;
use crate::repository::gbp_api::GbpApiClient;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gbpcli_rs", about = "CLI for Google Business Profile API")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all accounts accessible to the authenticated user
    ListAccounts {
        /// Parent account resource name (e.g. accounts/123)
        #[arg(long)]
        parent_account: Option<String>,

        /// Maximum number of accounts to return (default/max: 20)
        #[arg(long)]
        page_size: Option<u32>,

        /// Page token for pagination
        #[arg(long)]
        page_token: Option<String>,

        /// Filter (e.g. type=USER_GROUP)
        #[arg(long)]
        filter: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = config::Config::from_env().map_err(|e| {
        format!(
            "Missing environment variable: {}. Set GOOGLE_BUSINESS_API_CLIENT_ID, \
             GOOGLE_BUSINESS_API_CLIENT_SECRET, and GOOGLE_BUSINESS_API_REFRESH_TOKEN in .env",
            e
        )
    })?;

    let cli = Cli::parse();
    let client = reqwest::Client::new();

    let token = auth::refresh_access_token(&client, &config).await?;

    run(cli.command, token).await?;

    Ok(())
}

async fn run(cmd: Commands, token: TokenResponse) -> Result<(), Box<dyn std::error::Error>> {
    let http_client = reqwest::Client::new();
    let client = GbpApiClient::new(&http_client);

    match cmd {
        Commands::ListAccounts {
            parent_account,
            page_size,
            page_token,
            filter,
        } => {
            let resp = client
                .list_accounts(
                    &token.access_token,
                    parent_account.as_deref(),
                    page_size,
                    page_token.as_deref(),
                    filter.as_deref(),
                )
                .await?;

            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{bearer_token, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::repository::gbp_api::GbpApiClient;

    #[tokio::test]
    async fn test_list_accounts_calls_api() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/accounts"))
            .and(bearer_token("fake_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "accounts": [
                    {
                        "name": "accounts/123",
                        "accountName": "Test Account",
                        "type": "PERSONAL",
                        "role": "PRIMARY_OWNER"
                    }
                ]
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let http_client = reqwest::Client::new();
        let client = GbpApiClient::new(&http_client);
        let resp = client
            .list_accounts("fake_token", None, None, None, None)
            .await?;

        let accounts = resp.accounts.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name.as_deref(), Some("accounts/123"));
        assert_eq!(accounts[0].account_name.as_deref(), Some("Test Account"));
        assert_eq!(accounts[0].account_type.as_deref(), Some("PERSONAL"));
    }

    #[tokio::test]
    async fn test_list_accounts_with_params() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/accounts"))
            .and(bearer_token("fake_token"))
            .and(query_param("parentAccount", "accounts/456"))
            .and(query_param("pageSize", "10"))
            .and(query_param("filter", "type=USER_GROUP"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "accounts": [],
                "nextPageToken": "next_page"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let http_client = reqwest::Client::new();
        let client = GbpApiClient::new(&http_client);
        let resp = client
            .list_accounts(
                "fake_token",
                Some("accounts/456"),
                Some(10),
                None,
                Some("type=USER_GROUP"),
            )
            .await?;

        let accounts = resp.accounts.unwrap();
        assert!(accounts.is_empty());
        assert_eq!(resp.next_page_token.as_deref(), Some("next_page"));
    }
}
