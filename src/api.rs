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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_list_accounts_response() {
        let json = r#"{
            "accounts": [
                {
                    "name": "accounts/123456",
                    "accountName": "Test User",
                    "type": "PERSONAL",
                    "role": "PRIMARY_OWNER",
                    "verificationState": "VERIFIED",
                    "vettedState": "NOT_VETTED",
                    "permissionLevel": "OWNER_LEVEL"
                },
                {
                    "name": "accounts/789012",
                    "accountName": "My Business",
                    "type": "LOCATION_GROUP",
                    "role": "OWNER",
                    "accountNumber": "1234567890"
                }
            ],
            "nextPageToken": "abc123"
        }"#;

        let resp: ListAccountsResponse = serde_json::from_str(json).unwrap();
        let accounts = resp.accounts.unwrap();
        assert_eq!(accounts.len(), 2);

        assert_eq!(accounts[0].name.as_deref(), Some("accounts/123456"));
        assert_eq!(accounts[0].account_name.as_deref(), Some("Test User"));
        assert_eq!(accounts[0].account_type.as_deref(), Some("PERSONAL"));
        assert_eq!(accounts[0].role.as_deref(), Some("PRIMARY_OWNER"));
        assert_eq!(accounts[0].verification_state.as_deref(), Some("VERIFIED"));

        assert_eq!(accounts[1].name.as_deref(), Some("accounts/789012"));
        assert_eq!(accounts[1].account_type.as_deref(), Some("LOCATION_GROUP"));
        assert_eq!(accounts[1].account_number.as_deref(), Some("1234567890"));

        assert_eq!(resp.next_page_token.as_deref(), Some("abc123"));
    }

    #[test]
    fn test_deserialize_empty_response() {
        let json = r#"{}"#;
        let resp: ListAccountsResponse = serde_json::from_str(json).unwrap();
        assert!(resp.accounts.is_none());
        assert!(resp.next_page_token.is_none());
    }

    #[test]
    fn test_deserialize_no_next_page() {
        let json = r#"{
            "accounts": [
                {
                    "name": "accounts/111",
                    "accountName": "Solo Account",
                    "type": "PERSONAL",
                    "role": "PRIMARY_OWNER"
                }
            ]
        }"#;

        let resp: ListAccountsResponse = serde_json::from_str(json).unwrap();
        let accounts = resp.accounts.unwrap();
        assert_eq!(accounts.len(), 1);
        assert!(resp.next_page_token.is_none());
    }
}
