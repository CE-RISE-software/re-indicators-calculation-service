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
    pub artifact_base_url: String,
    pub shacl_url: String,
    pub status: String,
    pub details: Vec<String>,
}

impl ValidationSummary {
    pub fn artifact_resolved(artifacts: &ArtifactSet) -> Self {
        Self {
            basis: "shacl".to_string(),
            artifact_base_url: artifacts.base_url.clone(),
            shacl_url: artifacts.shacl_url.clone(),
            status: "artifact_resolved".to_string(),
            details: vec![
                "Published RE indicators artifact URLs were resolved successfully.".to_string(),
                "SHACL execution is not implemented yet.".to_string(),
            ],
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
    pub fn not_implemented() -> Self {
        Self {
            status: "not_implemented".to_string(),
            total_score: None,
            parameter_scores: Vec::new(),
            notes: vec!["The scoring engine is not implemented yet.".to_string()],
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
}
