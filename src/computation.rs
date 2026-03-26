use std::collections::HashMap;

use serde::Deserialize;

use crate::service_types::{ComputationResult, ParameterScore, QuestionScore};
use crate::spec_loader::{IndicatorSpecRepository, ParameterSpec};

#[derive(Debug, Deserialize)]
pub struct AssessmentPayload {
    pub indicator_specification_id: String,
    #[serde(default)]
    pub parameter_assessments: Vec<PayloadParameterAssessment>,
}

#[derive(Debug, Deserialize)]
pub struct PayloadParameterAssessment {
    pub parameter_id: String,
    #[serde(default)]
    pub question_answers: Vec<PayloadQuestionAnswer>,
}

#[derive(Debug, Deserialize)]
pub struct PayloadQuestionAnswer {
    pub question_id: String,
    pub selected_answer_id: Option<String>,
}

pub fn compute_result(
    repository: &IndicatorSpecRepository,
    payload: &serde_json::Value,
) -> Result<ComputationResult, ComputationError> {
    let assessment: AssessmentPayload =
        serde_json::from_value(payload.clone()).map_err(ComputationError::InvalidPayload)?;
    let indicator = repository
        .indicator_configs
        .get(&assessment.indicator_specification_id)
        .ok_or_else(|| {
            ComputationError::UnknownIndicator(assessment.indicator_specification_id.clone())
        })?;

    let answers_by_parameter: HashMap<&str, &PayloadParameterAssessment> = assessment
        .parameter_assessments
        .iter()
        .map(|pa| (pa.parameter_id.as_str(), pa))
        .collect();

    let mut parameter_scores = Vec::new();
    let mut total_score_sum = 0.0_f64;

    for application in &indicator.parameter_applications {
        let parameter_spec = repository
            .parameter_specs
            .get(&application.parameter_ref)
            .ok_or_else(|| ComputationError::UnknownParameter(application.parameter_ref.clone()))?;
        let payload_parameter = answers_by_parameter.get(application.parameter_ref.as_str());
        let parameter_score = compute_parameter_score(parameter_spec, payload_parameter);
        total_score_sum += parameter_score.computed_score.unwrap_or(0.0) * application.weight;
        parameter_scores.push(parameter_score);
    }

    Ok(ComputationResult {
        status: "computed".to_string(),
        total_score: Some(total_score_sum / 5.0),
        parameter_scores,
        notes: vec![
            "Scores are computed from model-defined question weights, answer scores, and parameter weights."
                .to_string(),
        ],
    })
}

fn compute_parameter_score(
    parameter_spec: &ParameterSpec,
    payload_parameter: Option<&&PayloadParameterAssessment>,
) -> ParameterScore {
    let payload_answers: HashMap<&str, &PayloadQuestionAnswer> = payload_parameter
        .map(|pa| {
            pa.question_answers
                .iter()
                .map(|qa| (qa.question_id.as_str(), qa))
                .collect()
        })
        .unwrap_or_default();

    let mut computed_score = 0.0_f64;
    let mut question_scores = Vec::new();

    for question in &parameter_spec.questions {
        let payload_answer = payload_answers.get(question.id.as_str()).copied();
        let answer = payload_answer.and_then(|qa| qa.selected_answer_id.as_deref());
        let answer_score = answer
            .and_then(|answer_id| question.answer_options.get(answer_id))
            .copied()
            .unwrap_or(0.0);

        computed_score += answer_score * question.weight;
        question_scores.push(QuestionScore {
            question_id: question.id.clone(),
            selected_answer_id: answer.map(ToString::to_string),
            answer_score: Some(answer_score),
        });
    }

    ParameterScore {
        parameter_id: parameter_spec.id.clone(),
        computed_score: Some(computed_score),
        question_scores,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ComputationError {
    #[error("payload is not a supported assessment shape: {0}")]
    InvalidPayload(serde_json::Error),

    #[error("unknown indicator specification id: {0}")]
    UnknownIndicator(String),

    #[error("unknown parameter specification id: {0}")]
    UnknownParameter(String),
}
