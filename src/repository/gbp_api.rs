mod data;
mod list_accounts;

use reqwest;

use crate::repository::gbp_api::data::BASE_URL;

pub struct GbpApiClient<'a> {
    client: &'a reqwest::Client,
}

impl<'a> GbpApiClient<'a> {
    pub fn new(client: &'a reqwest::Client) -> Self {
        GbpApiClient { client }
    }

    pub async fn list_accounts(
        &self,
        access_token: &str,
        parent_account: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
        filter: Option<&str>,
    ) -> Result<list_accounts::ListAccountsResponse, reqwest::Error> {
        list_accounts::run(
            self.client,
            BASE_URL,
            access_token,
            parent_account,
            page_size,
            page_token,
            filter,
        )
        .await
    }
}
