use once_cell::sync::OnceCell;
use reqwest::Url;
use serde::Serialize;

mod api;
mod mobile_no;
use mobile_no::*;
mod error;
use error::{Context, Error, ErrorKind, Result};

static INSTANCE: OnceCell<Client> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct Client {
    pub inner: reqwest::Client,
    pub base_url: Url,
    pub sender: String,
    pub template_id: String,
    pub auth_key: String,
}

impl Client {
    pub fn from_env() -> Self {
        let inner = reqwest::Client::new();
        let url = std::env::var("API_URL").expect("API URL not set");
        Self {
            inner,
            base_url: Url::parse(&url).unwrap(), 
            sender: std::env::var("SENDER").expect("SENDER not set"), 
            template_id: std::env::var("TEMPLATE_ID").expect("TEMPLATE ID not set"),
            auth_key: std::env::var("AUTH_KEY").expect("AUTH KEY not set"),
        }
    }

    pub fn initialize_from_env() {
        INSTANCE.set(Client::from_env()).unwrap();
    }

    pub fn global() -> &'static Client {
        INSTANCE.get().expect("SMS client not initialized")
    }

}
/// # Utility Methods
impl Client {
    pub fn absolute_url(&self, url: impl AsRef<str>) -> Result<Url> {
        Ok(self
            .base_url
            .join(url.as_ref())
            .map_err(|err| Error::new(err.to_string(), ErrorKind::Internal))?)
    }
}

/// # HTTP methods
impl Client {
    pub async fn get<A, P>(&self, route: A, parameters: Option<&P>) -> Result<String>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
    {
        self._get(self.absolute_url(route)?, parameters)
            .await?
            .text()
            .await
            .map_err(|err| Error::new(err.to_string(), ErrorKind::ApiError(err.to_string())))
    }

    pub async fn _get<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.inner.get(url);
        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }
        self.execute(request).await
    }

    pub async fn execute(&self, request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        request.send().await.context("Http execution failure")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
	use std::collections::HashMap;

    #[tokio::test]
    async fn test_send_otp() -> Result<()> {
        dotenv::dotenv().ok();
        Client::initialize_from_env();
        let client = Client::global();
        let data = r#"{
            "mobile": "8610993906",
            "otp": 123456
        }"#;
        let input_data = serde_json::from_str(&data).unwrap();
        println!("\n data: {:?} \n", &input_data);

        let handler = api::Handler::new(&client);
        let result = handler.send_otp(input_data).await?;
        println!("\n result: {:?} \n", result);
        Ok(())
    }
}