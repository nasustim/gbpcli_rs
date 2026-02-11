use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://mybusinessaccountmanagement.googleapis.com/v1";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub name: Option<String>,
    pub account_name: Option<String>,
    pub primary_owner: Option<String>,
    #[serde(rename = "type")]
    pub account_type: Option<String>,
    pub role: Option<String>,
    pub verification_state: Option<String>,
    pub vetted_state: Option<String>,
    pub account_number: Option<String>,
    pub permission_level: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsResponse {
    pub accounts: Option<Vec<Account>>,
    pub next_page_token: Option<String>,
}

pub async fn list_accounts(
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
