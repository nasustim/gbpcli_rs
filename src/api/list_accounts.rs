use serde::{Deserialize, Serialize};

use crate::api::data::{Account, BASE_URL};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsResponse {
    pub accounts: Option<Vec<Account>>,
    pub next_page_token: Option<String>,
}

pub async fn run(
    client: &reqwest::Client,
    access_token: &str,
    parent_account: Option<&str>,
    page_size: Option<u32>,
    page_token: Option<&str>,
    filter: Option<&str>,
) -> Result<ListAccountsResponse, reqwest::Error> {
    let url = format!("{}/accounts", BASE_URL);

    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(parent) = parent_account {
        params.push(("parentAccount", parent.to_string()));
    }
    if let Some(size) = page_size {
        params.push(("pageSize", size.to_string()));
    }
    if let Some(token) = page_token {
        params.push(("pageToken", token.to_string()));
    }
    if let Some(f) = filter {
        params.push(("filter", f.to_string()));
    }

    client
        .get(&url)
        .bearer_auth(access_token)
        .query(&params)
        .send()
        .await?
        .error_for_status()?
        .json::<ListAccountsResponse>()
        .await
}
