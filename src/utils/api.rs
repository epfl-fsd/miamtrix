use reqwest::Client;
use chrono::Local;

pub struct ApiClient {
    base_url: String,
    api_password: String,
    api_username: String,
    client: Client,


}

impl ApiClient {
    pub fn new(base_url: String, api_username: String, api_password: String) -> Self {
        ApiClient {
            base_url,
            api_password,
            api_username,
            client: Client::new(),
        }
    }
    pub async fn get(&self) -> Result<reqwest::Response, reqwest::Error> {
        let date_fmt = Local::now().format("%Y-%m-%d");
        self.client
            .get(format!("{}?date={}", self.base_url, date_fmt))
            .basic_auth(&self.api_username, Some(&self.api_password))
            .send()
            .await
    }
}
