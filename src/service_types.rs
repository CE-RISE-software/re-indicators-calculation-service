use serde::{Deserialize, Serialize};

use crate::artifacts::ArtifactSet;

#[derive(Debug, Deserialize, Serialize)]
pub struct ComputeRequest {
    pub model_version: Option<String>,
    pub payload: AssessmentPayload,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComputeResponse {
    pub model_family: String,
    pub model_version: String,
    pub artifact_base_url: String,
    pub artifacts: ArtifactSet,
    pub payload: AssessmentPayload,
    pub validation: ValidationSummary,
    pub result: ComputationResult,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssessmentPayload {
    pub indicator_specification_id: String,
    #[serde(default)]
    pub parameter_assessments: Vec<PayloadParameterAssessment>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PayloadParameterAssessment {
    pub parameter_id: String,
    #[serde(default)]
    pub question_answers: Vec<PayloadQuestionAnswer>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PayloadQuestionAnswer {
    pub question_id: String,
    pub selected_answer_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationSummary {
    pub basis: String,
    pub source: String,
    pub validation_url: String,
    pub status: String,
    pub passed: Option<bool>,
    pub finding_count: Option<u64>,
    pub findings_present: Option<bool>,
    pub raw_report: Option<serde_json::Value>,
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
            finding_count: None,
            findings_present: None,
            raw_report: None,
            details: vec![
                "Validation delegation to hex-core-service is disabled for this runtime."
                    .to_string(),
            ],
        }
    }

    pub fn validated_by_hex_core(validation_url: String, report: serde_json::Value) -> Self {
        let passed = report.get("passed").and_then(serde_json::Value::as_bool);
        let finding_count = extract_finding_count(&report);
        Self {
            basis: "shacl".to_string(),
            source: "hex-core-service".to_string(),
            validation_url,
            status: "validated_by_hex_core".to_string(),
            passed,
            finding_count,
            findings_present: finding_count.map(|count| count > 0),
            raw_report: Some(report),
            details: vec!["Payload validation was delegated to hex-core-service.".to_string()],
        }
    }
}

fn extract_finding_count(report: &serde_json::Value) -> Option<u64> {
    let keys = ["results", "findings", "violations"];
    for key in keys {
        if let Some(values) = report.get(key).and_then(serde_json::Value::as_array) {
            return Some(values.len() as u64);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::ValidationSummary;

    #[test]
    fn validation_summary_extracts_findings_from_results() {
        let summary = ValidationSummary::validated_by_hex_core(
            "http://127.0.0.1:8080/models/re-indicators-specification/versions/0.0.4:validate"
                .to_string(),
            serde_json::json!({
                "passed": false,
                "results": [
                    {"path": "x"},
                    {"path": "y"}
                ]
            }),
        );

        assert_eq!(summary.passed, Some(false));
        assert_eq!(summary.finding_count, Some(2));
        assert_eq!(summary.findings_present, Some(true));
        assert!(summary.raw_report.is_some());
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
    pub fn computed(total_score: f64, parameter_scores: Vec<ParameterScore>, note: String) -> Self {
        Self {
            status: "computed".to_string(),
            total_score: Some(total_score),
            parameter_scores,
            notes: vec![note],
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
    pub artifact_base_url_template: String,
    pub hex_core_base_url: String,
    pub http_timeout_secs: u64,
    pub bind_address: String,
    pub port: u16,
    pub calculation_artifact_url_template: String,
}
