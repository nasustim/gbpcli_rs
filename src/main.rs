mod auth;
mod config;
mod repository;

use crate::auth::TokenResponse;
use crate::repository::gbp_api::{GbpApi, GbpApiClient};
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
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    let api_client = GbpApiClient::new(&client);

    run(cli.command, token, &api_client).await?;

    Ok(())
}

async fn run(
    cmd: Commands,
    token: TokenResponse,
    client: &impl GbpApi,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    use std::cell::Cell;

    use crate::repository::gbp_api::{GbpApi, ListAccountsResponse};
    use crate::{Commands, run};

    struct MockGbpApiClient {
        response: ListAccountsResponse,
        called: Cell<bool>,
    }

    impl GbpApi for MockGbpApiClient {
        async fn list_accounts(
            &self,
            _access_token: &str,
            _parent_account: Option<&str>,
            _page_size: Option<u32>,
            _page_token: Option<&str>,
            _filter: Option<&str>,
        ) -> Result<ListAccountsResponse, Box<dyn std::error::Error + Send + Sync>> {
            self.called.set(true);
            Ok(ListAccountsResponse {
                accounts: self.response.accounts.clone(),
                next_page_token: self.response.next_page_token.clone(),
            })
        }
    }

    #[tokio::test]
    async fn api_called_with_command() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mock_client = MockGbpApiClient {
            response: ListAccountsResponse {
                accounts: None,
                next_page_token: None,
            },
            called: Cell::new(false),
        };

        let cmd = Commands::ListAccounts {
            parent_account: None,
            page_size: None,
            page_token: None,
            filter: None,
        };
        let token = crate::auth::TokenResponse {
            access_token: "fake_token".to_string(),
        };
        run(cmd, token, &mock_client).await?;
        assert!(mock_client.called.get(), "list_accounts should be called");
        Ok(())
    }
}
