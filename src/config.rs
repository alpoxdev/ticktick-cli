use url::Url;

use crate::error::Result;

pub const API_BASE_URL: &str = "https://api.ticktick.com";
pub const OAUTH_BASE_URL: &str = "https://ticktick.com";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthUrlConfig {
    pub client_id: String,
    pub scope: String,
    pub state: String,
    pub redirect_uri: String,
}

impl AuthUrlConfig {
    pub fn build_url(&self) -> Result<Url> {
        let mut url = Url::parse(&format!("{OAUTH_BASE_URL}/oauth/authorize"))?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("client_id", &self.client_id);
            pairs.append_pair("scope", &self.scope);
            pairs.append_pair("state", &self.state);
            pairs.append_pair("redirect_uri", &self.redirect_uri);
            pairs.append_pair("response_type", "code");
        }
        Ok(url)
    }
}
