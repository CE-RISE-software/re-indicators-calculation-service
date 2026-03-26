use serde::{Deserialize, Serialize};

use crate::{ARTIFACT_BASE_URL_TEMPLATE, DEFAULT_TESTING_VERSION};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArtifactSet {
    pub model_version: String,
    pub base_url: String,
    pub shacl_url: String,
    pub schema_url: String,
    pub owl_url: String,
    pub route_url: String,
}

impl ArtifactSet {
    pub fn for_requested_version(version: Option<&str>) -> Self {
        let resolved_version = version.unwrap_or(DEFAULT_TESTING_VERSION).to_string();
        Self::for_version(&resolved_version)
    }

    pub fn for_version(version: &str) -> Self {
        let base_url = ARTIFACT_BASE_URL_TEMPLATE.replace("{version}", version);
        Self::from_base_url(version.to_string(), base_url)
    }

    pub fn from_base_url(model_version: String, base_url: String) -> Self {
        Self {
            model_version,
            shacl_url: format!("{base_url}shacl.ttl"),
            schema_url: format!("{base_url}schema.json"),
            owl_url: format!("{base_url}owl.ttl"),
            route_url: format!("{base_url}route.json"),
            base_url,
        }
    }
}
