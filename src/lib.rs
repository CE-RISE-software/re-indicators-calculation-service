mod artifacts;
mod calculation_artifact_client;
mod computation;
pub mod config;
mod hex_core_api_client;
mod service_types;

use artifacts::ArtifactSet;
use axum::{
    Json, Router,
    extract::{State, rejection::JsonRejection},
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use calculation_artifact_client::CalculationArtifactClient;
use computation::{ComputationError, compute_result, ensure_artifact_version};
use config::RuntimeConfig;
use hex_core_api_client::{HexCoreApiClient, HexCoreError};
use service_types::{
    ComputationResult, ComputeRequest, ComputeResponse, ErrorResponse, HealthResponse,
    ValidationSummary,
};
#[cfg(test)]
use std::sync::{Arc, Mutex};

pub const MODEL_FAMILY: &str = "re-indicators-specification";
pub const VALIDATION_BASIS: &str = "shacl";
pub const DEFAULT_TESTING_VERSION: &str = "0.0.4";
pub const DEFAULT_ARTIFACT_BASE_URL_TEMPLATE: &str = "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v{version}/generated/";
pub const DEFAULT_HEX_CORE_BASE_URL: &str = "http://127.0.0.1:8080";
pub const DEFAULT_HTTP_TIMEOUT_SECS: u64 = 15;
pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 8081;

pub fn app() -> Router {
    app_with_config(RuntimeConfig::from_env(), true)
}

fn app_with_config(config: RuntimeConfig, validate_via_hex_core: bool) -> Router {
    let state = AppState {
        runtime_config: config.clone(),
        validate_via_hex_core,
        hex_core_api: HexCoreApiClient::new(
            config.hex_core_base_url.clone(),
            config.http_timeout_secs,
        ),
        calculation_artifact_client: CalculationArtifactClient::new(config.http_timeout_secs),
        fetch_calculation_artifact: validate_via_hex_core,
        #[cfg(test)]
        mock_validation_report: None,
        #[cfg(test)]
        mock_validation_error: None,
        #[cfg(test)]
        observed_bearer_token: None,
        #[cfg(test)]
        mock_calculation_artifact: None,
    };

    Router::new()
        .route("/health", get(health))
        .route("/compute", post(compute))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        model_family: MODEL_FAMILY,
        validation_basis: VALIDATION_BASIS,
        default_testing_version: DEFAULT_TESTING_VERSION,
        artifact_base_url_template: state.runtime_config.artifact_base_url_template.clone(),
        hex_core_base_url: state.runtime_config.hex_core_base_url.clone(),
        http_timeout_secs: state.runtime_config.http_timeout_secs,
        bind_address: state.runtime_config.bind_address.clone(),
        port: state.runtime_config.port,
        calculation_artifact_url_template: format!(
            "{}calculation.json",
            state.runtime_config.artifact_base_url_template
        ),
    })
}

#[derive(Clone)]
struct AppState {
    runtime_config: RuntimeConfig,
    validate_via_hex_core: bool,
    hex_core_api: HexCoreApiClient,
    calculation_artifact_client: CalculationArtifactClient,
    fetch_calculation_artifact: bool,
    #[cfg(test)]
    mock_validation_report: Option<serde_json::Value>,
    #[cfg(test)]
    mock_validation_error: Option<HexCoreError>,
    #[cfg(test)]
    observed_bearer_token: Option<Arc<Mutex<Option<String>>>>,
    #[cfg(test)]
    mock_calculation_artifact: Option<computation::CalculationArtifact>,
}

async fn compute(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Result<Json<ComputeRequest>, JsonRejection>,
) -> Result<Json<ComputeResponse>, ApiError> {
    let Json(request) =
        request.map_err(|error| ApiError::invalid_request_body(error.body_text()))?;
    let artifacts = ArtifactSet::for_requested_version_with_template(
        request.model_version.as_deref(),
        &state.runtime_config.artifact_base_url_template,
    );
    let payload = request.payload;
    let bearer_token = extract_bearer_token(&headers);
    let validation_url = state.hex_core_api.validation_url(&artifacts.model_version);
    let validation = if state.validate_via_hex_core {
        #[cfg(test)]
        let report = if let Some(error) = &state.mock_validation_error {
            if let Some(observed) = &state.observed_bearer_token {
                *observed.lock().unwrap() = bearer_token.clone();
            }
            return Err(ApiError::from_hex_core(
                error.clone(),
                validation_url.clone(),
            ));
        } else if let Some(report) = &state.mock_validation_report {
            if let Some(observed) = &state.observed_bearer_token {
                *observed.lock().unwrap() = bearer_token.clone();
            }
            report.clone()
        } else {
            state
                .hex_core_api
                .validate_payload(&artifacts.model_version, &payload, bearer_token.as_deref())
                .await
                .map_err(|error| ApiError::from_hex_core(error, validation_url.clone()))?
        };
        #[cfg(not(test))]
        let report = state
            .hex_core_api
            .validate_payload(&artifacts.model_version, &payload, bearer_token.as_deref())
            .await
            .map_err(|error| ApiError::from_hex_core(error, validation_url.clone()))?;

        let summary = ValidationSummary::validated_by_hex_core(validation_url, report.clone());
        if summary.passed == Some(false) {
            return Err(ApiError::validation_failed(summary));
        }
        summary
    } else {
        ValidationSummary::skipped(validation_url)
    };
    let computation_result = if state.fetch_calculation_artifact {
        #[cfg(test)]
        let artifact = if let Some(artifact) = &state.mock_calculation_artifact {
            artifact.clone()
        } else {
            state
                .calculation_artifact_client
                .fetch(&artifacts.calculation_url)
                .await
                .map_err(|error| {
                    ApiError::from_calculation_artifact(error, &artifacts.calculation_url)
                })?
        };
        #[cfg(not(test))]
        let artifact = state
            .calculation_artifact_client
            .fetch(&artifacts.calculation_url)
            .await
            .map_err(|error| {
                ApiError::from_calculation_artifact(error, &artifacts.calculation_url)
            })?;
        ensure_artifact_version(&artifact, &artifacts.model_version)
            .and_then(|_| compute_result(&artifact, &payload))
            .map_err(ApiError::from_computation)?
    } else {
        ComputationResult::computed(
            0.0,
            Vec::new(),
            "Calculation artifact fetching is disabled for this runtime.".to_string(),
        )
    };

    Ok(Json(ComputeResponse {
        model_family: MODEL_FAMILY.to_string(),
        model_version: artifacts.model_version.clone(),
        artifact_base_url: artifacts.base_url.clone(),
        artifacts: artifacts.clone(),
        payload,
        validation,
        result: computation_result,
    }))
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    let value = headers.get(AUTHORIZATION)?.to_str().ok()?;
    value
        .strip_prefix("Bearer ")
        .map(std::string::ToString::to_string)
}

struct ApiError {
    status: StatusCode,
    body: ErrorResponse,
}

impl ApiError {
    fn invalid_request_body(message: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            body: ErrorResponse {
                code: "INVALID_REQUEST_BODY".to_string(),
                message: "Request body does not match the supported compute payload shape."
                    .to_string(),
                details: Some(serde_json::json!({ "error": message })),
            },
        }
    }

    fn validation_failed(validation: ValidationSummary) -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            body: ErrorResponse {
                code: "VALIDATION_FAILED".to_string(),
                message: "Delegated validation through hex-core-service did not pass.".to_string(),
                details: Some(serde_json::to_value(validation).unwrap_or(serde_json::Value::Null)),
            },
        }
    }

    fn from_hex_core(error: HexCoreError, validation_url: String) -> Self {
        match error {
            HexCoreError::UnexpectedStatus { status: 401, body, .. } => Self {
                status: StatusCode::UNAUTHORIZED,
                body: ErrorResponse {
                    code: "HEX_CORE_UNAUTHORIZED".to_string(),
                    message: "hex-core-service rejected the delegated validation request.".to_string(),
                    details: Some(serde_json::json!({
                        "validation_url": validation_url,
                        "body": body
                    })),
                },
            },
            HexCoreError::UnexpectedStatus { status: 404, body, .. } => Self {
                status: StatusCode::NOT_FOUND,
                body: ErrorResponse {
                    code: "MODEL_VERSION_NOT_FOUND".to_string(),
                    message: "The requested RE indicators model version was not found in hex-core-service.".to_string(),
                    details: Some(serde_json::json!({
                        "validation_url": validation_url,
                        "body": body
                    })),
                },
            },
            other => Self {
                status: StatusCode::BAD_GATEWAY,
                body: ErrorResponse {
                    code: "HEX_CORE_VALIDATION_ERROR".to_string(),
                    message: "Delegated validation through hex-core-service failed.".to_string(),
                    details: Some(serde_json::json!({
                        "validation_url": validation_url,
                        "error": other.to_string()
                    })),
                },
            },
        }
    }

    fn from_calculation_artifact(
        error: calculation_artifact_client::CalculationArtifactError,
        calculation_url: &str,
    ) -> Self {
        match error {
            calculation_artifact_client::CalculationArtifactError::UnexpectedStatus { status: 404, .. } => Self {
                status: StatusCode::NOT_FOUND,
                body: ErrorResponse {
                    code: "CALCULATION_ARTIFACT_NOT_FOUND".to_string(),
                    message: "The published calculation artifact for the requested version was not found.".to_string(),
                    details: Some(serde_json::json!({ "calculation_url": calculation_url })),
                },
            },
            other => Self {
                status: StatusCode::BAD_GATEWAY,
                body: ErrorResponse {
                    code: "CALCULATION_ARTIFACT_ERROR".to_string(),
                    message: "Fetching the published calculation artifact failed.".to_string(),
                    details: Some(serde_json::json!({
                        "calculation_url": calculation_url,
                        "error": other.to_string()
                    })),
                },
            },
        }
    }

    fn from_computation(error: ComputationError) -> Self {
        match error {
            ComputationError::UnknownIndicator(_) => Self {
                status: StatusCode::UNPROCESSABLE_ENTITY,
                body: ErrorResponse {
                    code: "UNKNOWN_INDICATOR_SPECIFICATION".to_string(),
                    message: error.to_string(),
                    details: None,
                },
            },
            ComputationError::ArtifactVersionMismatch { .. } => Self {
                status: StatusCode::BAD_GATEWAY,
                body: ErrorResponse {
                    code: "CALCULATION_ARTIFACT_VERSION_MISMATCH".to_string(),
                    message: error.to_string(),
                    details: None,
                },
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
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

    use crate::computation::CalculationArtifact;
    use crate::config::RuntimeConfig;

    use super::{DEFAULT_TESTING_VERSION, app_with_config};

    fn test_config() -> RuntimeConfig {
        RuntimeConfig {
            hex_core_base_url: "http://127.0.0.1:8080".to_string(),
            artifact_base_url_template:
                "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v{version}/generated/"
                    .to_string(),
            http_timeout_secs: 15,
            bind_address: "127.0.0.1".to_string(),
            port: 8081,
        }
    }

    fn app_with_mocked_upstreams(
        config: RuntimeConfig,
        validation_report: serde_json::Value,
        observed_bearer_token: std::sync::Arc<std::sync::Mutex<Option<String>>>,
        calculation_artifact: CalculationArtifact,
    ) -> axum::Router {
        let state = super::AppState {
            runtime_config: config.clone(),
            validate_via_hex_core: true,
            hex_core_api: super::HexCoreApiClient::new(
                config.hex_core_base_url.clone(),
                config.http_timeout_secs,
            ),
            calculation_artifact_client: super::CalculationArtifactClient::new(
                config.http_timeout_secs,
            ),
            fetch_calculation_artifact: true,
            mock_validation_report: Some(validation_report),
            mock_validation_error: None,
            observed_bearer_token: Some(observed_bearer_token),
            mock_calculation_artifact: Some(calculation_artifact),
        };

        axum::Router::new()
            .route("/health", axum::routing::get(super::health))
            .route("/compute", axum::routing::post(super::compute))
            .with_state(state)
    }

    #[tokio::test]
    async fn health_reports_fixed_model_family() {
        let response = app_with_config(test_config(), false)
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
        let response = app_with_config(test_config(), false)
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
        let response = app_with_config(test_config(), false)
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
                "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v{DEFAULT_TESTING_VERSION}/generated/"
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
                "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v{DEFAULT_TESTING_VERSION}/generated/shacl.ttl"
            )
        );
        assert_eq!(
            json["validation"]["validation_url"],
            format!(
                "http://127.0.0.1:8080/models/re-indicators-specification/versions/{DEFAULT_TESTING_VERSION}:validate"
            )
        );
    }

    #[tokio::test]
    async fn compute_rejects_invalid_request_body() {
        let response = app_with_config(test_config(), false)
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/compute")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{
                          "model_version":"0.0.4",
                          "payload":{"id":"assessment-1"}
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["code"], "INVALID_REQUEST_BODY");
    }

    #[tokio::test]
    async fn compute_runs_end_to_end_with_mocked_upstreams() {
        let observed_bearer_token = std::sync::Arc::new(std::sync::Mutex::new(None));
        let calculation_artifact: CalculationArtifact = serde_json::from_str(include_str!(
            "../tests/fixtures/calculation-recycle-battery-0.0.4.json"
        ))
        .unwrap();
        let config = RuntimeConfig {
            hex_core_base_url: "http://hex-core-service.test".to_string(),
            artifact_base_url_template: "http://artifact-registry.test/pages-v{version}/generated/"
                .to_string(),
            http_timeout_secs: 15,
            bind_address: "127.0.0.1".to_string(),
            port: 8081,
        };

        let response = app_with_mocked_upstreams(
            config,
            serde_json::json!({"passed": true, "results": []}),
            observed_bearer_token.clone(),
            calculation_artifact,
        )
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/compute")
                .header("content-type", "application/json")
                .header("authorization", "Bearer test-token")
                .body(Body::from(format!(
                    r#"{{
                          "model_version":"0.0.4",
                          "payload":{}
                        }}"#,
                    include_str!("../tests/fixtures/payload-recycle-battery-0.0.4.json")
                )))
                .unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["validation"]["status"], "validated_by_hex_core");
        assert_eq!(json["validation"]["passed"], true);
        assert_eq!(json["validation"]["finding_count"], 0);
        assert_eq!(
            json["payload"]["indicator_specification_id"],
            "REcycle_Battery"
        );
        assert_eq!(
            json["result"]["parameter_scores"].as_array().unwrap().len(),
            5
        );
        assert!((json["result"]["total_score"].as_f64().unwrap() - 0.138572436192).abs() < 1e-12);
        assert_eq!(
            *observed_bearer_token.lock().unwrap(),
            Some("test-token".to_string())
        );
    }

    #[tokio::test]
    async fn compute_returns_422_when_delegated_validation_fails() {
        let observed_bearer_token = std::sync::Arc::new(std::sync::Mutex::new(None));
        let calculation_artifact: CalculationArtifact = serde_json::from_str(include_str!(
            "../tests/fixtures/calculation-recycle-battery-0.0.4.json"
        ))
        .unwrap();
        let config = RuntimeConfig {
            hex_core_base_url: "http://hex-core-service.test".to_string(),
            artifact_base_url_template: "http://artifact-registry.test/pages-v{version}/generated/"
                .to_string(),
            http_timeout_secs: 15,
            bind_address: "127.0.0.1".to_string(),
            port: 8081,
        };

        let response = app_with_mocked_upstreams(
            config,
            serde_json::json!({
                "passed": false,
                "results": [
                    {"message": "shape violation"}
                ]
            }),
            observed_bearer_token.clone(),
            calculation_artifact,
        )
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/compute")
                .header("content-type", "application/json")
                .header("authorization", "Bearer test-token")
                .body(Body::from(format!(
                    r#"{{
                          "model_version":"0.0.4",
                          "payload":{}
                        }}"#,
                    include_str!("../tests/fixtures/payload-recycle-battery-0.0.4.json")
                )))
                .unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["code"], "VALIDATION_FAILED");
        assert_eq!(json["details"]["passed"], false);
        assert_eq!(json["details"]["finding_count"], 1);
        assert_eq!(json["details"]["status"], "validated_by_hex_core");
        assert_eq!(
            *observed_bearer_token.lock().unwrap(),
            Some("test-token".to_string())
        );
    }

    #[tokio::test]
    async fn compute_returns_502_when_delegated_validation_errors() {
        let observed_bearer_token = std::sync::Arc::new(std::sync::Mutex::new(None));
        let calculation_artifact: CalculationArtifact = serde_json::from_str(include_str!(
            "../tests/fixtures/calculation-recycle-battery-0.0.4.json"
        ))
        .unwrap();
        let config = RuntimeConfig {
            hex_core_base_url: "http://hex-core-service.test".to_string(),
            artifact_base_url_template: "http://artifact-registry.test/pages-v{version}/generated/"
                .to_string(),
            http_timeout_secs: 15,
            bind_address: "127.0.0.1".to_string(),
            port: 8081,
        };
        let state = super::AppState {
            runtime_config: config.clone(),
            validate_via_hex_core: true,
            hex_core_api: super::HexCoreApiClient::new(
                config.hex_core_base_url.clone(),
                config.http_timeout_secs,
            ),
            calculation_artifact_client: super::CalculationArtifactClient::new(
                config.http_timeout_secs,
            ),
            fetch_calculation_artifact: true,
            mock_validation_report: None,
            mock_validation_error: Some(super::HexCoreError::RequestFailed {
                url: "http://hex-core-service.test/models/re-indicators-specification/versions/0.0.4:validate".to_string(),
                reason: "connection refused".to_string(),
            }),
            observed_bearer_token: Some(observed_bearer_token.clone()),
            mock_calculation_artifact: Some(calculation_artifact),
        };

        let response = axum::Router::new()
            .route("/health", axum::routing::get(super::health))
            .route("/compute", axum::routing::post(super::compute))
            .with_state(state)
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/compute")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer test-token")
                    .body(Body::from(format!(
                        r#"{{
                          "model_version":"0.0.4",
                          "payload":{}
                        }}"#,
                        include_str!("../tests/fixtures/payload-recycle-battery-0.0.4.json")
                    )))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["code"], "HEX_CORE_VALIDATION_ERROR");
        assert_eq!(
            json["details"]["validation_url"],
            "http://hex-core-service.test/models/re-indicators-specification/versions/0.0.4:validate"
        );
        assert!(
            json["details"]["error"]
                .as_str()
                .unwrap()
                .contains("connection refused")
        );
        assert_eq!(
            *observed_bearer_token.lock().unwrap(),
            Some("test-token".to_string())
        );
    }
}
