use crate::artifacts::ArtifactSet;

#[derive(Debug, Clone)]
pub struct ArtifactFetcher {
    client: reqwest::Client,
}

impl ArtifactFetcher {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .expect("build reqwest client"),
        }
    }

    pub async fn fetch_shacl_text(
        &self,
        artifacts: &ArtifactSet,
    ) -> Result<String, ArtifactFetchError> {
        self.fetch_text(&artifacts.shacl_url).await
    }

    async fn fetch_text(&self, url: &str) -> Result<String, ArtifactFetchError> {
        let response = self.client.get(url).send().await.map_err(|reason| {
            ArtifactFetchError::RequestFailed {
                url: url.to_string(),
                reason: reason.to_string(),
            }
        })?;

        let status = response.status();
        if !status.is_success() {
            return Err(ArtifactFetchError::UnexpectedStatus {
                url: url.to_string(),
                status: status.as_u16(),
            });
        }

        response
            .text()
            .await
            .map_err(|reason| ArtifactFetchError::BodyReadFailed {
                url: url.to_string(),
                reason: reason.to_string(),
            })
    }
}

impl Default for ArtifactFetcher {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ArtifactFetchError {
    #[error("artifact request failed for {url}: {reason}")]
    RequestFailed { url: String, reason: String },

    #[error("artifact fetch returned HTTP {status} for {url}")]
    UnexpectedStatus { url: String, status: u16 },

    #[error("artifact body read failed for {url}: {reason}")]
    BodyReadFailed { url: String, reason: String },
}
