//! Transport-neutral request construction for Evento Globolo.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Client {
    base_url: String,
    bearer_token: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestSpec {
    pub method: &'static str,
    pub url: String,
    pub authorization: Option<String>,
    pub headers: Vec<(String, String)>,
    pub json_body: Option<String>,
}

impl Client {
    pub fn new(base_url: impl Into<String>) -> Result<Self, &'static str> {
        let base_url = base_url.into().trim_end_matches('/').to_owned();
        if !(base_url.starts_with("https://") || base_url.starts_with("http://localhost")) {
            return Err("base URL must use HTTPS or localhost HTTP");
        }
        Ok(Self {
            base_url,
            bearer_token: None,
        })
    }

    pub fn with_bearer_token(mut self, token: impl Into<String>) -> Self {
        self.bearer_token = Some(token.into());
        self
    }

    pub fn health(&self) -> RequestSpec {
        self.get("/healthz")
    }
    pub fn config(&self) -> RequestSpec {
        self.get("/api/config")
    }
    pub fn events(&self) -> RequestSpec {
        self.post("/api/events")
    }
    pub fn alerts(&self) -> RequestSpec {
        self.post("/api/alerts")
    }
    pub fn providers(&self) -> RequestSpec {
        self.get("/v1/providers")
    }
    pub fn connections(&self) -> RequestSpec {
        self.get("/v1/connections")
    }
    pub fn list_events(&self) -> RequestSpec {
        self.get("/v1/events")
    }
    pub fn create_event(&self, json: impl Into<String>) -> RequestSpec {
        self.post_json("/v1/events", json)
    }

    pub fn start_oauth(&self, provider: &str) -> Result<RequestSpec, &'static str> {
        let provider = path_segment(provider)?;
        Ok(self.post_json(&format!("/v1/oauth/{provider}/start"), "{}"))
    }

    pub fn job(&self, job_id: &str) -> Result<RequestSpec, &'static str> {
        let job_id = path_segment(job_id)?;
        Ok(self.get(&format!("/v1/jobs/{job_id}")))
    }

    pub fn cross_post(
        &self,
        event_id: &str,
        idempotency_key: impl Into<String>,
        targets_json: impl Into<String>,
    ) -> Result<RequestSpec, &'static str> {
        let event_id = path_segment(event_id)?;
        let idempotency_key = idempotency_key.into();
        if idempotency_key.is_empty() || idempotency_key.len() > 200 {
            return Err("idempotency key must contain 1..=200 bytes");
        }
        let mut request =
            self.post_json(&format!("/v1/events/{event_id}/cross-post"), targets_json);
        request
            .headers
            .push(("idempotency-key".into(), idempotency_key));
        Ok(request)
    }

    pub fn job_websocket_url(&self, job_id: &str) -> Result<String, &'static str> {
        let job_id = path_segment(job_id)?;
        let scheme = if self.base_url.starts_with("https://") {
            "wss://"
        } else {
            "ws://"
        };
        let authority = self
            .base_url
            .split_once("://")
            .map(|(_, value)| value)
            .unwrap_or(&self.base_url);
        Ok(format!("{scheme}{authority}/v1/jobs/{job_id}/ws"))
    }

    fn get(&self, path: &str) -> RequestSpec {
        self.request("GET", path)
    }
    fn post(&self, path: &str) -> RequestSpec {
        self.request("POST", path)
    }
    fn post_json(&self, path: &str, json: impl Into<String>) -> RequestSpec {
        let mut request = self.request("POST", path);
        request
            .headers
            .push(("content-type".into(), "application/json".into()));
        request.json_body = Some(json.into());
        request
    }
    fn request(&self, method: &'static str, path: &str) -> RequestSpec {
        RequestSpec {
            method,
            url: format!("{}{}", self.base_url, path),
            authorization: self
                .bearer_token
                .as_ref()
                .map(|token| format!("Bearer {token}")),
            headers: Vec::new(),
            json_body: None,
        }
    }
}

fn path_segment(value: &str) -> Result<&str, &'static str> {
    if !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        Ok(value)
    } else {
        Err("path identifier contains unsupported characters")
    }
}

pub const PRODUCT: &str = "evento-globolo";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_authenticated_requests() {
        let client = Client::new("https://api.example.com/")
            .unwrap()
            .with_bearer_token("secret");
        assert_eq!(client.health().url, "https://api.example.com/healthz");
        assert_eq!(client.events().method, "POST");
        assert_eq!(
            client.config().authorization.as_deref(),
            Some("Bearer secret")
        );
        let cross_post = client
            .cross_post("event-1", "idem-1", r#"{"targets":[]}"#)
            .unwrap();
        assert_eq!(
            cross_post.url,
            "https://api.example.com/v1/events/event-1/cross-post"
        );
        assert!(cross_post
            .headers
            .contains(&("idempotency-key".into(), "idem-1".into())));
        assert_eq!(
            client.job_websocket_url("job-1").unwrap(),
            "wss://api.example.com/v1/jobs/job-1/ws"
        );
    }

    #[test]
    fn rejects_cleartext_remote_urls() {
        assert!(Client::new("http://example.com").is_err());
        assert!(Client::new("http://localhost:8787").is_ok());
    }

    #[test]
    fn rejects_unsafe_dynamic_path_segments() {
        let client = Client::new("https://api.example.com").unwrap();
        assert!(client.job("../other-user").is_err());
        assert!(client.start_oauth("meta/facebook").is_err());
    }
}
