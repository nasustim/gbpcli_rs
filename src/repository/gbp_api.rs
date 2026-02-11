mod data;
mod list_accounts;

use reqwest;

use crate::repository::gbp_api::data::BASE_URL;

pub use list_accounts::ListAccountsResponse;

pub trait GbpApi {
    async fn list_accounts(
        &self,
        access_token: &str,
        parent_account: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
        filter: Option<&str>,
    ) -> Result<ListAccountsResponse, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct GbpApiClient<'a> {
    client: &'a reqwest::Client,
}

impl<'a> GbpApiClient<'a> {
    pub fn new(client: &'a reqwest::Client) -> Self {
        GbpApiClient { client }
    }
}

impl<'a> GbpApi for GbpApiClient<'a> {
    async fn list_accounts(
        &self,
        access_token: &str,
        parent_account: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
        filter: Option<&str>,
    ) -> Result<ListAccountsResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(list_accounts::run(
            self.client,
            BASE_URL,
            access_token,
            parent_account,
            page_size,
            page_token,
            filter,
        )
        .await?)
    }
}
