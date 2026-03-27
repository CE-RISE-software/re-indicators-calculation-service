use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::DEFAULT_TESTING_VERSION;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ArtifactSet {
    pub model_version: String,
    pub base_url: String,
    pub calculation_url: String,
    pub shacl_url: String,
    pub schema_url: String,
    pub owl_url: String,
}

impl ArtifactSet {
    pub fn for_requested_version_with_template(version: Option<&str>, template: &str) -> Self {
        let resolved_version = version.unwrap_or(DEFAULT_TESTING_VERSION).to_string();
        let base_url = template.replace("{version}", &resolved_version);
        Self::from_base_url(resolved_version, base_url)
    }

    pub fn from_base_url(model_version: String, base_url: String) -> Self {
        Self {
            model_version,
            calculation_url: format!("{base_url}calculation.json"),
            shacl_url: format!("{base_url}shacl.ttl"),
            schema_url: format!("{base_url}schema.json"),
            owl_url: format!("{base_url}owl.ttl"),
            base_url,
        }
    }
}
