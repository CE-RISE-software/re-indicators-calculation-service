mod service_types;

use axum::{
    Json, Router,
    routing::{get, post},
};
use service_types::{
    ComputationResult, ComputeRequest, ComputeResponse, HealthResponse, ValidationSummary,
};

pub const MODEL_FAMILY: &str = "re-indicators-specification";
pub const VALIDATION_BASIS: &str = "shacl";
pub const DEFAULT_TESTING_VERSION: &str = "0.0.3";
pub const ARTIFACT_BASE_URL_TEMPLATE: &str = "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{version}/generated/";

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/compute", post(compute))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        model_family: MODEL_FAMILY,
        validation_basis: VALIDATION_BASIS,
        default_testing_version: DEFAULT_TESTING_VERSION,
        artifact_base_url_template: ARTIFACT_BASE_URL_TEMPLATE,
    })
}

async fn compute(Json(request): Json<ComputeRequest>) -> Json<ComputeResponse> {
    let model_version = request
        .model_version
        .unwrap_or_else(|| DEFAULT_TESTING_VERSION.to_string());
    let artifact_base_url = artifact_base_url_for_version(&model_version);

    Json(ComputeResponse {
        model_family: MODEL_FAMILY.to_string(),
        model_version,
        artifact_base_url: artifact_base_url.clone(),
        payload: request.payload,
        validation: ValidationSummary::not_implemented(artifact_base_url),
        result: ComputationResult::not_implemented(),
    })
}

fn artifact_base_url_for_version(version: &str) -> String {
    ARTIFACT_BASE_URL_TEMPLATE.replace("{version}", version)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        body::to_bytes,
        http::{Request, StatusCode},
    };
    use serde_json::Value;
    use tower::ServiceExt;

    use super::{DEFAULT_TESTING_VERSION, app};

    #[tokio::test]
    async fn health_reports_fixed_model_family() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn compute_accepts_version_and_payload() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/compute")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{
                          "model_version":"0.0.3",
                          "payload":{"id":"assessment-1","indicator_specification_id":"REcycle_PV"}
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn compute_defaults_to_testing_version() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/compute")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{
                          "payload":{"id":"assessment-1","indicator_specification_id":"REcycle_PV"}
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["model_version"], DEFAULT_TESTING_VERSION);
        assert_eq!(
            json["artifact_base_url"],
            format!(
                "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{DEFAULT_TESTING_VERSION}/generated/"
            )
        );
    }
}
