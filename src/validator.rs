use crate::{artifacts::ArtifactSet, service_types::ValidationSummary};

pub fn summarize_validation_readiness(
    artifacts: &ArtifactSet,
    payload: &serde_json::Value,
    fetched_shacl: Option<&str>,
    fetch_error: Option<&str>,
) -> ValidationSummary {
    match (fetched_shacl, fetch_error) {
        (Some(shacl_text), _) => ValidationSummary::awaiting_rdf_mapping(
            artifacts,
            shacl_text.len(),
            payload_requires_rdf_mapping_note(payload),
        ),
        (_, Some(error)) => ValidationSummary::shacl_fetch_failed(artifacts, error.to_string()),
        _ => ValidationSummary::artifact_resolved(artifacts),
    }
}

fn payload_requires_rdf_mapping_note(payload: &serde_json::Value) -> String {
    let payload_kind = match payload {
        serde_json::Value::Object(_) => "JSON object",
        serde_json::Value::Array(_) => "JSON array",
        serde_json::Value::String(_) => "JSON string",
        serde_json::Value::Number(_) => "JSON number",
        serde_json::Value::Bool(_) => "JSON boolean",
        serde_json::Value::Null => "null JSON value",
    };

    format!(
        "SHACL execution needs an RDF data graph; the current request payload is a {payload_kind} and no RDF mapping/serialization step is implemented yet."
    )
}
