use serde::{Deserialize, Serialize};

use crate::artifacts::ArtifactSet;

#[derive(Debug, Deserialize, Serialize)]
pub struct ComputeRequest {
    pub model_version: Option<String>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComputeResponse {
    pub model_family: String,
    pub model_version: String,
    pub artifact_base_url: String,
    pub artifacts: ArtifactSet,
    pub payload: serde_json::Value,
    pub validation: ValidationSummary,
    pub result: ComputationResult,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationSummary {
    pub basis: String,
    pub source: String,
    pub validation_url: String,
    pub status: String,
    pub passed: Option<bool>,
    pub report: Option<serde_json::Value>,
    pub details: Vec<String>,
}

impl ValidationSummary {
    pub fn skipped(validation_url: String) -> Self {
        Self {
            basis: "shacl".to_string(),
            source: "hex-core-service".to_string(),
            validation_url,
            status: "validation_skipped".to_string(),
            passed: None,
            report: None,
            details: vec![
                "Validation delegation to hex-core-service is disabled for this runtime."
                    .to_string(),
            ],
        }
    }

    pub fn validated_by_hex_core(validation_url: String, report: serde_json::Value) -> Self {
        let passed = report.get("passed").and_then(serde_json::Value::as_bool);
        Self {
            basis: "shacl".to_string(),
            source: "hex-core-service".to_string(),
            validation_url,
            status: "validated_by_hex_core".to_string(),
            passed,
            report: Some(report),
            details: vec!["Payload validation was delegated to hex-core-service.".to_string()],
        }
    }

    pub fn hex_core_validation_failed(validation_url: String, reason: String) -> Self {
        Self {
            basis: "shacl".to_string(),
            source: "hex-core-service".to_string(),
            validation_url,
            status: "hex_core_validation_failed".to_string(),
            passed: None,
            report: None,
            details: vec![format!(
                "Delegated validation through hex-core-service failed: {reason}"
            )],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComputationResult {
    pub status: String,
    pub total_score: Option<f64>,
    pub parameter_scores: Vec<ParameterScore>,
    pub notes: Vec<String>,
}

impl ComputationResult {
    pub fn failed(reason: String) -> Self {
        Self {
            status: "failed".to_string(),
            total_score: None,
            parameter_scores: Vec::new(),
            notes: vec![reason],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParameterScore {
    pub parameter_id: String,
    pub computed_score: Option<f64>,
    pub question_scores: Vec<QuestionScore>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionScore {
    pub question_id: String,
    pub selected_answer_id: Option<String>,
    pub answer_score: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub model_family: &'static str,
    pub validation_basis: &'static str,
    pub default_testing_version: &'static str,
    pub artifact_base_url_template: &'static str,
    pub hex_core_base_url: String,
    pub spec_source_dir: String,
}
