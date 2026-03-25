use serde::{Deserialize, Serialize};

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
    pub payload: serde_json::Value,
    pub validation: ValidationSummary,
    pub result: ComputationResult,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationSummary {
    pub basis: String,
    pub artifact_base_url: String,
    pub status: String,
    pub details: Vec<String>,
}

impl ValidationSummary {
    pub fn not_implemented(artifact_base_url: String) -> Self {
        Self {
            basis: "shacl".to_string(),
            artifact_base_url,
            status: "not_implemented".to_string(),
            details: vec![
                "SHACL-backed validation will be implemented against published RE indicators artifacts."
                    .to_string(),
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
            notes: vec![
                "The scoring engine is not implemented yet; this response only establishes the API contract."
                    .to_string(),
            ],
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
