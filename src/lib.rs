mod artifacts;
mod fetcher;
mod service_types;
mod validator;

use artifacts::ArtifactSet;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use fetcher::ArtifactFetcher;
use service_types::{
    ComputationResult, ComputeRequest, ComputeResponse, HealthResponse, ValidationSummary,
};
use validator::summarize_validation_readiness;

pub const MODEL_FAMILY: &str = "re-indicators-specification";
pub const VALIDATION_BASIS: &str = "shacl";
pub const DEFAULT_TESTING_VERSION: &str = "0.0.3";
pub const ARTIFACT_BASE_URL_TEMPLATE: &str = "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{version}/generated/";

pub fn app() -> Router {
    app_with_fetch(true)
}

fn app_with_fetch(fetch_shacl_on_compute: bool) -> Router {
    let state = AppState {
        fetch_shacl_on_compute,
        fetcher: ArtifactFetcher::new(),
    };

    Router::new()
        .route("/health", get(health))
        .route("/compute", post(compute))
        .with_state(state)
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

#[derive(Clone)]
struct AppState {
    fetch_shacl_on_compute: bool,
    fetcher: ArtifactFetcher,
}

async fn compute(
    State(state): State<AppState>,
    Json(request): Json<ComputeRequest>,
) -> Json<ComputeResponse> {
    let artifacts = ArtifactSet::for_requested_version(request.model_version.as_deref());
    let payload = request.payload;
    let validation = if state.fetch_shacl_on_compute {
        match state.fetcher.fetch_shacl_text(&artifacts).await {
            Ok(shacl_text) => {
                summarize_validation_readiness(&artifacts, &payload, Some(&shacl_text), None)
            }
            Err(error) => {
                summarize_validation_readiness(&artifacts, &payload, None, Some(&error.to_string()))
            }
        }
    } else {
        ValidationSummary::artifact_resolved(&artifacts)
    };

    Json(ComputeResponse {
        model_family: MODEL_FAMILY.to_string(),
        model_version: artifacts.model_version.clone(),
        artifact_base_url: artifacts.base_url.clone(),
        artifacts: artifacts.clone(),
        payload,
        validation,
        result: ComputationResult::not_implemented(),
    })
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

    use super::{DEFAULT_TESTING_VERSION, app_with_fetch};

    #[tokio::test]
    async fn health_reports_fixed_model_family() {
        let response = app_with_fetch(false)
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
        let response = app_with_fetch(false)
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
        let response = app_with_fetch(false)
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
        assert_eq!(
            json["validation"]["status"],
            serde_json::Value::String("artifact_resolved".to_string())
        );
        assert_eq!(
            json["validation"]["fetched"],
            serde_json::Value::Bool(false)
        );
        assert_eq!(
            json["artifacts"]["shacl_url"],
            format!(
                "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{DEFAULT_TESTING_VERSION}/generated/shacl.ttl"
            )
        );
    }
}
