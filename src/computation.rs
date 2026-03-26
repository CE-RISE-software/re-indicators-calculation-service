use std::collections::HashMap;

use serde::Deserialize;

use crate::service_types::{
    AssessmentPayload, ComputationResult, ParameterScore, PayloadParameterAssessment,
    PayloadQuestionAnswer, QuestionScore,
};

#[derive(Debug, Clone, Deserialize)]
pub struct CalculationArtifact {
    pub model_version: String,
    pub indicator_configurations: HashMap<String, CalculationIndicatorConfiguration>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalculationIndicatorConfiguration {
    pub parameters: Vec<CalculationParameter>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalculationParameter {
    pub parameter_ref: String,
    pub weight: f64,
    #[serde(default)]
    pub fixed_score: Option<f64>,
    #[serde(default)]
    pub questions: Vec<CalculationQuestion>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalculationQuestion {
    pub question_id: String,
    pub weight: f64,
    pub answers: Vec<CalculationAnswer>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalculationAnswer {
    pub answer_id: String,
    pub score: f64,
}

pub fn ensure_artifact_version(
    artifact: &CalculationArtifact,
    requested_version: &str,
) -> Result<(), ComputationError> {
    if artifact.model_version == requested_version {
        Ok(())
    } else {
        Err(ComputationError::ArtifactVersionMismatch {
            artifact_version: artifact.model_version.clone(),
            requested_version: requested_version.to_string(),
        })
    }
}

pub fn compute_result(
    artifact: &CalculationArtifact,
    assessment: &AssessmentPayload,
) -> Result<ComputationResult, ComputationError> {
    let indicator = artifact
        .indicator_configurations
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

    for parameter in &indicator.parameters {
        let payload_parameter = answers_by_parameter.get(parameter.parameter_ref.as_str());
        let parameter_score = compute_parameter_score(parameter, payload_parameter);
        total_score_sum += parameter_score.computed_score.unwrap_or(0.0) * parameter.weight;
        parameter_scores.push(parameter_score);
    }

    Ok(ComputationResult {
        ..ComputationResult::computed(
            total_score_sum / 5.0,
            parameter_scores,
            "Scores are computed from calculation.json question scores and parameter weights."
                .to_string(),
        )
    })
}

fn compute_parameter_score(
    parameter: &CalculationParameter,
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

    let mut question_scores = Vec::new();
    let computed_score = if let Some(fixed_score) = parameter.fixed_score {
        fixed_score
    } else {
        let mut sum = 0.0_f64;
        for question in &parameter.questions {
            let payload_answer = payload_answers.get(question.question_id.as_str()).copied();
            let answer = payload_answer.and_then(|qa| qa.selected_answer_id.as_deref());
            let answer_score = answer
                .and_then(|answer_id| {
                    question
                        .answers
                        .iter()
                        .find(|candidate| candidate.answer_id == answer_id)
                })
                .map(|answer| answer.score)
                .unwrap_or(0.0);

            sum += answer_score * question.weight;
            question_scores.push(QuestionScore {
                question_id: question.question_id.clone(),
                selected_answer_id: answer.map(ToString::to_string),
                answer_score: Some(answer_score),
            });
        }
        sum
    };

    ParameterScore {
        parameter_id: parameter.parameter_ref.clone(),
        computed_score: Some(computed_score),
        question_scores,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ComputationError {
    #[error("unknown indicator specification id: {0}")]
    UnknownIndicator(String),

    #[error(
        "calculation artifact model version {artifact_version} does not match requested version {requested_version}"
    )]
    ArtifactVersionMismatch {
        artifact_version: String,
        requested_version: String,
    },
}

#[cfg(test)]
mod tests {
    use crate::service_types::AssessmentPayload;

    use super::{CalculationArtifact, compute_result, ensure_artifact_version};

    #[test]
    fn computes_score_from_calculation_artifact() {
        let artifact: CalculationArtifact = serde_json::from_str(
            r#"{
              "model_version":"0.0.4",
              "indicator_configurations":{
                "REuse_Laptop":{
                  "parameters":[
                    {
                      "parameter_ref":"P1_product_diagnosis",
                      "weight":0.5,
                      "questions":[
                        {
                          "question_id":"Q1.1",
                          "weight":1.0,
                          "answers":[
                            {"answer_id":"a","score":3.0},
                            {"answer_id":"b","score":0.0}
                          ]
                        }
                      ]
                    },
                    {
                      "parameter_ref":"P2_warranty_information",
                      "weight":0.5,
                      "questions":[
                        {
                          "question_id":"Q2.1",
                          "weight":1.0,
                          "answers":[
                            {"answer_id":"x","score":5.0},
                            {"answer_id":"y","score":0.0}
                          ]
                        }
                      ]
                    }
                  ]
                }
              }
            }"#,
        )
        .unwrap();

        ensure_artifact_version(&artifact, "0.0.4").unwrap();

        let payload: AssessmentPayload = serde_json::from_value(serde_json::json!({
          "indicator_specification_id":"REuse_Laptop",
          "parameter_assessments":[
            {"parameter_id":"P1_product_diagnosis","question_answers":[{"question_id":"Q1.1","selected_answer_id":"a"}]},
            {"parameter_id":"P2_warranty_information","question_answers":[{"question_id":"Q2.1","selected_answer_id":"x"}]}
          ]
        }))
        .unwrap();

        let result = compute_result(&artifact, &payload).unwrap();

        assert_eq!(result.status, "computed");
        assert_eq!(result.total_score, Some(0.8));
    }

    #[test]
    fn computes_score_from_released_recycle_battery_fixture() {
        let artifact: CalculationArtifact = serde_json::from_str(include_str!(
            "../tests/fixtures/calculation-recycle-battery-0.0.4.json"
        ))
        .unwrap();
        let payload: AssessmentPayload = serde_json::from_str(include_str!(
            "../tests/fixtures/payload-recycle-battery-0.0.4.json"
        ))
        .unwrap();

        ensure_artifact_version(&artifact, "0.0.4").unwrap();

        let result = compute_result(&artifact, &payload).unwrap();

        assert_eq!(result.status, "computed");
        assert_eq!(result.parameter_scores.len(), 5);
        assert!((result.parameter_scores[0].computed_score.unwrap() - 2.85).abs() < 1e-9);
        assert!((result.parameter_scores[1].computed_score.unwrap() - 3.96).abs() < 1e-9);
        assert!((result.parameter_scores[2].computed_score.unwrap() - 5.0).abs() < 1e-9);
        assert!((result.parameter_scores[3].computed_score.unwrap() - 3.000000006).abs() < 1e-9);
        assert!((result.parameter_scores[4].computed_score.unwrap() - 2.958277259).abs() < 1e-9);
        assert!((result.total_score.unwrap() - 0.138572436192).abs() < 1e-12);
    }
}
