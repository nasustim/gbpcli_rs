mod api;
mod auth;
mod config;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gbpcli_rs", about = "CLI for Google Business Profile API")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get daily metrics time series for a location
    GetDailyMetrics {
        /// Location ID (numeric, e.g. 12345)
        #[arg(long)]
        location_id: String,

        /// Daily metric (e.g. WEBSITE_CLICKS, CALL_CLICKS, BUSINESS_DIRECTION_REQUESTS, etc.)
        #[arg(long)]
        metric: String,

        /// Start date in YYYY-MM-DD format
        #[arg(long)]
        start_date: String,

        /// End date in YYYY-MM-DD format
        #[arg(long)]
        end_date: String,
    },

    /// Fetch multiple daily metrics time series for a location
    FetchMultiDailyMetrics {
        /// Location ID (numeric, e.g. 12345)
        #[arg(long)]
        location_id: String,

        /// Daily metrics (comma-separated, e.g. WEBSITE_CLICKS,CALL_CLICKS)
        #[arg(long, value_delimiter = ',')]
        metrics: Vec<String>,

        /// Start date in YYYY-MM-DD format
        #[arg(long)]
        start_date: String,

        /// End date in YYYY-MM-DD format
        #[arg(long)]
        end_date: String,
    },
}

fn parse_date(date_str: &str) -> Result<(i32, i32, i32), String> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return Err(format!("Invalid date format: {}. Expected YYYY-MM-DD", date_str));
    }
    let year = parts[0].parse::<i32>().map_err(|e| format!("Invalid year: {}", e))?;
    let month = parts[1].parse::<i32>().map_err(|e| format!("Invalid month: {}", e))?;
    let day = parts[2].parse::<i32>().map_err(|e| format!("Invalid day: {}", e))?;
    Ok((year, month, day))
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

    match cli.command {
        Commands::GetDailyMetrics {
            location_id,
            metric,
            start_date,
            end_date,
        } => {
            let (sy, sm, sd) = parse_date(&start_date)?;
            let (ey, em, ed) = parse_date(&end_date)?;

            let resp = api::get_daily_metrics_time_series(
                &client,
                &token.access_token,
                &location_id,
                &metric,
                sy, sm, sd,
                ey, em, ed,
            )
            .await?;

            println!("{}", serde_json::to_string_pretty(&resp)?);
        }

        Commands::FetchMultiDailyMetrics {
            location_id,
            metrics,
            start_date,
            end_date,
        } => {
            let (sy, sm, sd) = parse_date(&start_date)?;
            let (ey, em, ed) = parse_date(&end_date)?;

            let metrics_refs: Vec<&str> = metrics.iter().map(|s| s.as_str()).collect();

            let resp = api::fetch_multi_daily_metrics_time_series(
                &client,
                &token.access_token,
                &location_id,
                &metrics_refs,
                sy, sm, sd,
                ey, em, ed,
            )
            .await?;

            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_valid() {
        let (y, m, d) = parse_date("2024-01-15").unwrap();
        assert_eq!(y, 2024);
        assert_eq!(m, 1);
        assert_eq!(d, 15);
    }

    #[test]
    fn test_parse_date_invalid_format() {
        assert!(parse_date("2024/01/15").is_err());
        assert!(parse_date("2024-01").is_err());
        assert!(parse_date("not-a-date").is_err());
    }
}
