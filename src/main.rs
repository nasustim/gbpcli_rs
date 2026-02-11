mod api;
mod auth;
mod config;

use clap::{Parser, Subcommand};

use crate::auth::TokenResponse;

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
    let client = reqwest::Client::new();

    match cmd {
        Commands::ListAccounts {
            parent_account,
            page_size,
            page_token,
            filter,
        } => {
            let resp = api::list_accounts::run(
                &client,
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
