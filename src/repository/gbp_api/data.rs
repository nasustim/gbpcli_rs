use serde::{Deserialize, Serialize};

pub const BASE_URL: &str = "https://mybusinessaccountmanagement.googleapis.com/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
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
