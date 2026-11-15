use evgl_domain::{EventDraft, ProviderCapabilities, PublishTarget};
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Clone)]
pub struct EvglClient {
    base_url: Url,
    token: String,
    http: Client,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderView {
    pub capabilities: ProviderCapabilities,
    pub configured: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthStart {
    pub provider: String,
    pub authorization_url: Url,
    pub expires_in_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub event_id: Uuid,
    pub status: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateEvent<'a> {
    pub title: &'a str,
    pub summary: &'a str,
    pub description_html: &'a str,
    pub starts_at: &'a str,
    pub ends_at: &'a str,
    pub timezone: &'a str,
    pub canonical_url: &'a str,
    pub online_url: Option<&'a str>,
    pub venue: Option<Value>,
    pub tags: Vec<String>,
    pub metadata: Value,
}

impl EvglClient {
    pub fn new(base_url: Url, token: impl Into<String>) -> Self {
        Self { base_url, token: token.into(), http: Client::new() }
    }

    pub async fn providers(&self) -> Result<Vec<ProviderView>, Error> {
        self.request(Method::GET, "v1/providers", Option::<&()>::None).await
    }

    pub async fn start_oauth(&self, provider: &str) -> Result<OAuthStart, Error> {
        self.request(Method::POST, &format!("v1/oauth/{provider}/start"), Some(&Value::Null)).await
    }

    pub async fn connections(&self) -> Result<Vec<Value>, Error> {
        self.request(Method::GET, "v1/connections", Option::<&()>::None).await
    }

    pub async fn create_event(&self, input: &CreateEvent<'_>) -> Result<EventDraft, Error> {
        self.request(Method::POST, "v1/events", Some(input)).await
    }

    pub async fn cross_post(
        &self,
        event_id: Uuid,
        idempotency_key: &str,
        targets: &[PublishTarget],
    ) -> Result<Value, Error> {
        let url = self.base_url.join(&format!("v1/events/{event_id}/cross-post"))?;
        let response = self.http.post(url)
            .bearer_auth(&self.token)
            .header("idempotency-key", idempotency_key)
            .json(&serde_json::json!({ "targets": targets }))
            .send().await?;
        decode(response).await
    }

    pub async fn job(&self, job_id: Uuid) -> Result<Value, Error> {
        self.request(Method::GET, &format!("v1/jobs/{job_id}"), Option::<&()>::None).await
    }

    pub fn job_websocket_url(&self, job_id: Uuid) -> Result<Url, Error> {
        let mut url = self.base_url.join(&format!("v1/jobs/{job_id}/ws"))?;
        let scheme = if url.scheme() == "https" { "wss" } else { "ws" };
        url.set_scheme(scheme).map_err(|_| Error::InvalidWebSocketUrl)?;
        Ok(url)
    }

    async fn request<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, Error> {
        let url = self.base_url.join(path)?;
        let mut request = self.http.request(method, url).bearer_auth(&self.token);
        if let Some(body) = body { request = request.json(body); }
        decode(request.send().await?).await
    }
}

async fn decode<T: DeserializeOwned>(response: reqwest::Response) -> Result<T, Error> {
    let status = response.status();
    let text = response.text().await?;
    if !status.is_success() {
        return Err(Error::Api { status: status.as_u16(), body: text });
    }
    Ok(serde_json::from_str(&text)?)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid URL: {0}")]
    Url(#[from] url::ParseError),
    #[error("network request failed: {0}")]
    Network(#[from] reqwest::Error),
    #[error("invalid response JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API rejected the request ({status}): {body}")]
    Api { status: u16, body: String },
    #[error("could not construct WebSocket URL")]
    InvalidWebSocketUrl,
}
