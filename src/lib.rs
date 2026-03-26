mod artifacts;
mod computation;
mod hex_core_api_client;
mod service_types;
mod spec_loader;

use artifacts::ArtifactSet;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use computation::compute_result;
use hex_core_api_client::HexCoreApiClient;
use service_types::{
    ComputationResult, ComputeRequest, ComputeResponse, HealthResponse, ValidationSummary,
};
use spec_loader::IndicatorSpecRepository;

pub const MODEL_FAMILY: &str = "re-indicators-specification";
pub const VALIDATION_BASIS: &str = "shacl";
pub const DEFAULT_TESTING_VERSION: &str = "0.0.3";
pub const ARTIFACT_BASE_URL_TEMPLATE: &str = "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{version}/generated/";
pub const DEFAULT_HEX_CORE_BASE_URL: &str = "http://127.0.0.1:8080";
pub const DEFAULT_SPEC_SOURCE_DIR: &str = "/tmp/re-indicators-specification";

pub fn app() -> Router {
    app_with_hex_core_validation(true)
}

fn app_with_hex_core_validation(validate_via_hex_core: bool) -> Router {
    let state = AppState {
        validate_via_hex_core,
        hex_core_api: HexCoreApiClient::default(),
        spec_repository: load_default_spec_repository().ok(),
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
        hex_core_base_url: DEFAULT_HEX_CORE_BASE_URL.to_string(),
        spec_source_dir: DEFAULT_SPEC_SOURCE_DIR.to_string(),
    })
}

#[derive(Clone)]
struct AppState {
    validate_via_hex_core: bool,
    hex_core_api: HexCoreApiClient,
    spec_repository: Option<IndicatorSpecRepository>,
}

async fn compute(
    State(state): State<AppState>,
    Json(request): Json<ComputeRequest>,
) -> Json<ComputeResponse> {
    let artifacts = ArtifactSet::for_requested_version(request.model_version.as_deref());
    let payload = request.payload;
    let validation_url = state.hex_core_api.validation_url(&artifacts.model_version);
    let validation = if state.validate_via_hex_core {
        match state
            .hex_core_api
            .validate_payload(&artifacts.model_version, &payload)
            .await
        {
            Ok(report) => ValidationSummary::validated_by_hex_core(validation_url, report),
            Err(error) => {
                ValidationSummary::hex_core_validation_failed(validation_url, error.to_string())
            }
        }
    } else {
        ValidationSummary::skipped(validation_url)
    };
    let computation_result = if validation.passed == Some(false) {
        ComputationResult::failed(
            "Computation was skipped because delegated validation did not pass.".to_string(),
        )
    } else if let Some(repository) = &state.spec_repository {
        match compute_result(repository, &payload) {
            Ok(result) => result,
            Err(error) => ComputationResult::failed(error.to_string()),
        }
    } else {
        ComputationResult::failed(format!(
            "Specification repository could not be loaded from {}.",
            DEFAULT_SPEC_SOURCE_DIR
        ))
    };

    Json(ComputeResponse {
        model_family: MODEL_FAMILY.to_string(),
        model_version: artifacts.model_version.clone(),
        artifact_base_url: artifacts.base_url.clone(),
        artifacts: artifacts.clone(),
        payload,
        validation,
        result: computation_result,
    })
}

fn load_default_spec_repository() -> Result<IndicatorSpecRepository, spec_loader::SpecLoadError> {
    IndicatorSpecRepository::load_from_dir(std::path::Path::new(DEFAULT_SPEC_SOURCE_DIR))
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

    use super::{DEFAULT_TESTING_VERSION, app_with_hex_core_validation};

    #[tokio::test]
    async fn health_reports_fixed_model_family() {
        let response = app_with_hex_core_validation(false)
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
        let response = app_with_hex_core_validation(false)
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
        let response = app_with_hex_core_validation(false)
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
            serde_json::Value::String("validation_skipped".to_string())
        );
        assert_eq!(
            json["validation"]["source"],
            serde_json::Value::String("hex-core-service".to_string())
        );
        assert_eq!(
            json["artifacts"]["shacl_url"],
            format!(
                "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{DEFAULT_TESTING_VERSION}/generated/shacl.ttl"
            )
        );
        assert_eq!(
            json["validation"]["validation_url"],
            format!(
                "http://127.0.0.1:8080/models/re-indicators-specification/versions/{DEFAULT_TESTING_VERSION}:validate"
            )
        );
    }
}
