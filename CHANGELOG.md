# Changelog

All notable changes to the CE-RISE RE Indicators Calculation Service project will be documented in this file.

## [0.0.1] - 2026-03-27

### Added
- Initial Rust HTTP service for RE indicators calculation
- `GET /health`, `GET /openapi.json`, and `POST /compute` endpoints
- Delegated validation through `hex-core-service`
- Computation based on published `calculation.json` artifacts for `re-indicators-specification`
- Structured request and response types for payload, validation summary, artifacts, and computed scores
- OpenAPI generation from the implemented service routes
- Container build files and release workflow publishing the image as `re-indicators-calculation`
- Published Pages documentation for API, deployment, scope, integration, and local testing
- Local-only demonstrator using a pulled `hex-core-service` image and the published RE indicators artifacts

### Tested
- Added Rust unit and route tests for request validation, delegated validation outcomes, OpenAPI exposure, and real computation against released `0.0.4` calculation fixtures
- Verified the local demonstrator flow with delegated validation and successful computation for `REcycle_Battery`
