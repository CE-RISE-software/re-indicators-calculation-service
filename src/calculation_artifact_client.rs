#[derive(Debug, Clone)]
pub struct CalculationArtifactClient {
    client: reqwest::Client,
}

impl CalculationArtifactClient {
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(timeout_secs))
                .build()
                .expect("build reqwest client"),
        }
    }

    pub async fn fetch(
        &self,
        calculation_url: &str,
    ) -> Result<crate::computation::CalculationArtifact, CalculationArtifactError> {
        let response = self
            .client
            .get(calculation_url)
            .send()
            .await
            .map_err(|reason| CalculationArtifactError::RequestFailed {
                url: calculation_url.to_string(),
                reason: reason.to_string(),
            })?;

        let status = response.status();
        let body =
            response
                .text()
                .await
                .map_err(|reason| CalculationArtifactError::BodyReadFailed {
                    url: calculation_url.to_string(),
                    reason: reason.to_string(),
                })?;

        if !status.is_success() {
            return Err(CalculationArtifactError::UnexpectedStatus {
                url: calculation_url.to_string(),
                status: status.as_u16(),
                body,
            });
        }

        serde_json::from_str(&body).map_err(|reason| CalculationArtifactError::InvalidJson {
            url: calculation_url.to_string(),
            reason: reason.to_string(),
        })
    }
}

impl Default for CalculationArtifactClient {
    fn default() -> Self {
        Self::new(crate::DEFAULT_HTTP_TIMEOUT_SECS)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CalculationArtifactError {
    #[error("calculation artifact request failed for {url}: {reason}")]
    RequestFailed { url: String, reason: String },

    #[error("calculation artifact response read failed for {url}: {reason}")]
    BodyReadFailed { url: String, reason: String },

    #[error("calculation artifact returned HTTP {status} for {url}: {body}")]
    UnexpectedStatus {
        url: String,
        status: u16,
        body: String,
    },

    #[error("calculation artifact returned invalid JSON for {url}: {reason}")]
    InvalidJson { url: String, reason: String },
}
