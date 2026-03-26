use reqwest::header::AUTHORIZATION;

#[derive(Debug, Clone)]
pub struct HexCoreApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl HexCoreApiClient {
    pub fn new(base_url: String, timeout_secs: u64) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(timeout_secs))
                .build()
                .expect("build reqwest client"),
            base_url: normalize_base_url(base_url),
        }
    }

    pub fn validation_url(&self, model_version: &str) -> String {
        format!(
            "{}/models/re-indicators-specification/versions/{}:validate",
            self.base_url, model_version
        )
    }

    pub async fn validate_payload(
        &self,
        model_version: &str,
        payload: &impl serde::Serialize,
        bearer_token: Option<&str>,
    ) -> Result<serde_json::Value, HexCoreError> {
        let url = self.validation_url(model_version);
        let mut request = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "payload": payload }));

        if let Some(token) = bearer_token {
            request = request.header(AUTHORIZATION, format!("Bearer {token}"));
        }

        let response = request
            .send()
            .await
            .map_err(|reason| HexCoreError::RequestFailed {
                url: url.clone(),
                reason: reason.to_string(),
            })?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|reason| HexCoreError::BodyReadFailed {
                url: url.clone(),
                reason: reason.to_string(),
            })?;

        if !status.is_success() {
            return Err(HexCoreError::UnexpectedStatus {
                url,
                status: status.as_u16(),
                body,
            });
        }

        serde_json::from_str(&body).map_err(|reason| HexCoreError::InvalidJson {
            url,
            reason: reason.to_string(),
        })
    }
}

impl Default for HexCoreApiClient {
    fn default() -> Self {
        Self::new(
            crate::DEFAULT_HEX_CORE_BASE_URL.to_string(),
            crate::DEFAULT_HTTP_TIMEOUT_SECS,
        )
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum HexCoreError {
    #[error("hex-core request failed for {url}: {reason}")]
    RequestFailed { url: String, reason: String },

    #[error("hex-core response read failed for {url}: {reason}")]
    BodyReadFailed { url: String, reason: String },

    #[error("hex-core returned HTTP {status} for {url}: {body}")]
    UnexpectedStatus {
        url: String,
        status: u16,
        body: String,
    },

    #[error("hex-core returned invalid JSON for {url}: {reason}")]
    InvalidJson { url: String, reason: String },
}

fn normalize_base_url(mut base_url: String) -> String {
    while base_url.ends_with('/') {
        base_url.pop();
    }
    base_url
}
